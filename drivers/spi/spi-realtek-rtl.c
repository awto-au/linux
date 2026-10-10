// SPDX-License-Identifier: GPL-2.0-only

#include <linux/io.h>
#include <linux/ktime.h>
#include <linux/module.h>
#include <linux/platform_device.h>
#include <linux/spi/spi.h>
#include <linux/spi/spi-mem.h>
#include <linux/unaligned.h>

struct rtspi {
	void __iomem *base;
	struct device *dev;
	void __iomem *win_uc;	/* memory-mapped read window ("dirmap" reg), uncached */
	void *win_wb;		/* same window, cached */
	resource_size_t win_size;
	phys_addr_t win_phys;
	u32 boot_sfcr, boot_sfcsr;	/* as the bootloader left them, before init_hw() */
	u32 mmio_sfcsr;			/* nonzero: SFCSR written before window reads */
};

/* Flash reads through spi-mem dirmap; read on every call, so it can be flipped at runtime */
static int dirmap = 2;
module_param(dirmap, int, 0644);
MODULE_PARM_DESC(dirmap, "flash reads: 0 = PIO, 1 = mmio window uncached, 2 = mmio window cached");

/* SPI Flash Configuration Register */
#define RTL_SPI_SFCR			0x00
#define RTL_SPI_SFCR_RBO		BIT(28)
#define RTL_SPI_SFCR_WBO		BIT(27)

/* SPI Flash Configuration Register 2: mmio window read mode, set by the bootloader */
#define RTL_SPI_SFCR2			0x04

/* SPI Flash Control and Status Register */
#define RTL_SPI_SFCSR			0x08
#define RTL_SPI_SFCSR_CSB0		BIT(31)
#define RTL_SPI_SFCSR_CSB1		BIT(30)
#define RTL_SPI_SFCSR_RDY		BIT(27)
#define RTL_SPI_SFCSR_LEN_MASK		~(0x03 << 28)
#define RTL_SPI_SFCSR_LEN1		(0x00 << 28)
#define RTL_SPI_SFCSR_LEN4		(0x03 << 28)
/* Status bits (RTL9300 SDK register map); stock waits for RDY|IDLE|CSBx_STS before mmio */
#define RTL_SPI_SFCSR_CSB0_STS		BIT(11)
#define RTL_SPI_SFCSR_CSB1_STS		BIT(10)
#define RTL_SPI_SFCSR_IDLE		BIT(4)
#define RTL_SPI_SFCSR_MIO_PIO_ERR	BIT(0)

/* SPI Flash Data Register */
#define RTL_SPI_SFDR			0x0c

#define REG(x)		(rtspi->base + x)


static void rt_set_cs(struct spi_device *spi, bool level)
{
	struct rtspi *rtspi = spi_controller_get_devdata(spi->controller);
	unsigned int cs = spi_get_chipselect(spi, 0);
	u32 cs_mask, value;

	switch (cs) {
	case 0:
		cs_mask = RTL_SPI_SFCSR_CSB0;
		break;
	case 1:
		cs_mask = RTL_SPI_SFCSR_CSB1;
		break;
	default:
		return;
	}

	value = __raw_readl(REG(RTL_SPI_SFCSR));

	/* CSBx is active low */
	if (level)
		value |= cs_mask;
	else
		value &= ~cs_mask;

	__raw_writel(value, REG(RTL_SPI_SFCSR));
}

static void set_size(struct rtspi *rtspi, int size)
{
	u32 value;

	value = __raw_readl(REG(RTL_SPI_SFCSR));
	value &= RTL_SPI_SFCSR_LEN_MASK;
	if (size == 4)
		value |= RTL_SPI_SFCSR_LEN4;
	else if (size == 1)
		value |= RTL_SPI_SFCSR_LEN1;
	__raw_writel(value, REG(RTL_SPI_SFCSR));
}

static inline void wait_ready(struct rtspi *rtspi)
{
	while (!(__raw_readl(REG(RTL_SPI_SFCSR)) & RTL_SPI_SFCSR_RDY))
		cpu_relax();
}
static void send4(struct rtspi *rtspi, const u32 *buf)
{
	wait_ready(rtspi);
	set_size(rtspi, 4);
	__raw_writel(*buf, REG(RTL_SPI_SFDR));
}

static void send1(struct rtspi *rtspi, const u8 *buf)
{
	wait_ready(rtspi);
	set_size(rtspi, 1);
	__raw_writel(buf[0] << 24, REG(RTL_SPI_SFDR));
}

static void rcv4(struct rtspi *rtspi, u32 *buf)
{
	wait_ready(rtspi);
	set_size(rtspi, 4);
	*buf = __raw_readl(REG(RTL_SPI_SFDR));
}

static void rcv1(struct rtspi *rtspi, u8 *buf)
{
	wait_ready(rtspi);
	set_size(rtspi, 1);
	*buf = __raw_readl(REG(RTL_SPI_SFDR)) >> 24;
}

static int transfer_one(struct spi_controller *ctrl, struct spi_device *spi,
			struct spi_transfer *xfer)
{
	struct rtspi *rtspi = spi_controller_get_devdata(ctrl);
	void *rx_buf;
	const void *tx_buf;
	int cnt;

	tx_buf = xfer->tx_buf;
	rx_buf = xfer->rx_buf;
	cnt = xfer->len;
	if (tx_buf) {
		while (cnt >= 4) {
			send4(rtspi, tx_buf);
			tx_buf += 4;
			cnt -= 4;
		}
		while (cnt) {
			send1(rtspi, tx_buf);
			tx_buf++;
			cnt--;
		}
	} else if (rx_buf) {
		while (cnt >= 4) {
			rcv4(rtspi, rx_buf);
			rx_buf += 4;
			cnt -= 4;
		}
		while (cnt) {
			rcv1(rtspi, rx_buf);
			rx_buf++;
			cnt--;
		}
	}

	spi_finalize_current_transfer(ctrl);

	return 0;
}

/*
 * Status waits: one word = 32 SCK, CS release = SFCR TCS <= 31 SCK; at the slowest divider
 * (9.6 MHz) both are <= 3.3 us, x1.25 = 4.2 us, rounded to 5. Expiry logs mask, limit, elapsed.
 */
#define RTL_SPI_RDY_TIMEOUT_NS		(5 * NSEC_PER_USEC)
#define RTL_SPI_SFCSR_RELEASED		(RTL_SPI_SFCSR_RDY | RTL_SPI_SFCSR_IDLE | \
					 RTL_SPI_SFCSR_CSB0_STS | RTL_SPI_SFCSR_CSB1_STS)

static int rtspi_wait_bits(struct rtspi *rtspi, u32 bits)
{
	u64 t0 = ktime_get_ns(), dt;

	while ((__raw_readl(REG(RTL_SPI_SFCSR)) & bits) != bits) {
		dt = ktime_get_ns() - t0;
		if (dt > RTL_SPI_RDY_TIMEOUT_NS) {
			if ((__raw_readl(REG(RTL_SPI_SFCSR)) & bits) == bits)
				return 0;	/* preempted by an IRQ, not a stuck controller */
			dev_err_ratelimited(rtspi->dev,
				"SFCSR wait 0x%08x: limit %llu ns, elapsed %llu ns, SFCSR 0x%08x\n",
				bits, (u64)RTL_SPI_RDY_TIMEOUT_NS, dt,
				__raw_readl(REG(RTL_SPI_SFCSR)));
			return -ETIMEDOUT;
		}
		cpu_relax();
	}
	return 0;
}

static int rtspi_wait(struct rtspi *rtspi)
{
	return rtspi_wait_bits(rtspi, RTL_SPI_SFCSR_RDY);
}

static void rtspi_cs(struct rtspi *rtspi, bool assert)
{
	u32 v = __raw_readl(REG(RTL_SPI_SFCSR)) | RTL_SPI_SFCSR_CSB1;

	__raw_writel(assert ? v & ~RTL_SPI_SFCSR_CSB0 : v | RTL_SPI_SFCSR_CSB0, REG(RTL_SPI_SFCSR));
}

/* CS0/CS1 high, then wait until both report released and idle (stock sf_cs_release) */
static int rtspi_release(struct rtspi *rtspi)
{
	int ret = rtspi_wait_bits(rtspi, RTL_SPI_SFCSR_RDY | RTL_SPI_SFCSR_IDLE);

	if (rtspi->mmio_sfcsr)
		__raw_writel(rtspi->mmio_sfcsr, REG(RTL_SPI_SFCSR));
	else
		rtspi_cs(rtspi, false);
	return ret ?: rtspi_wait_bits(rtspi, RTL_SPI_SFCSR_RELEASED);
}

/* One read op by PIO (1-1-1), for the dirmap self-check and dirmap=0 */
static int rtspi_pio_read(struct rtspi *rtspi, const struct spi_mem_op *op, u32 addr,
			  u8 *buf, size_t len)
{
	u8 hdr[1 + 4 + 4];
	int n = 0, i, ret;

	hdr[n++] = op->cmd.opcode;
	for (i = op->addr.nbytes - 1; i >= 0; i--)
		hdr[n++] = addr >> (8 * i);
	for (i = 0; i < op->dummy.nbytes; i++)
		hdr[n++] = 0xff;

	ret = rtspi_wait(rtspi);
	if (ret)
		return ret;
	rtspi_cs(rtspi, true);
	for (i = 0; i < n && !ret; i++) {
		ret = rtspi_wait(rtspi);
		if (!ret) {
			set_size(rtspi, 1);
			__raw_writel((u32)hdr[i] << 24, REG(RTL_SPI_SFDR));
		}
	}
	while (len && !ret) {
		ret = rtspi_wait(rtspi);
		if (ret)
			break;
		if (len >= 4) {
			set_size(rtspi, 4);
			put_unaligned_be32(__raw_readl(REG(RTL_SPI_SFDR)), buf);
			buf += 4;
			len -= 4;
		} else {
			set_size(rtspi, 1);
			*buf++ = __raw_readl(REG(RTL_SPI_SFDR)) >> 24;
			len--;
		}
	}
	i = rtspi_release(rtspi);
	return ret ?: i;
}

/* The window reads in the bootloader's SFCR2 mode; PIO released and idle first */
static int rtspi_mmio_read(struct rtspi *rtspi, u32 addr, void *buf, size_t len, bool cached)
{
	int ret = rtspi_release(rtspi);

	if (ret)
		return ret;
	if (cached) {
		/* flash may have changed under the cache (PIO program/erase): drop stale lines */
		dma_cache_inv((unsigned long)rtspi->win_wb + addr, len);
		memcpy(buf, rtspi->win_wb + addr, len);
	} else {
		memcpy_fromio(buf, rtspi->win_uc + addr, len);
	}
	return 0;
}

/* Self-check mismatch: one boot log names the cause (#61) */
static void rtspi_dirmap_diag(struct rtspi *rtspi, const struct spi_mem_op *op, u32 a,
			      const u8 *pio, const u8 *mmio, size_t len, int rel)
{
	u32 sfcsr = __raw_readl(REG(RTL_SPI_SFCSR));
	char pat[40] = "none";
	u8 b[16];
	int i, s;

	for (i = 0; i < ARRAY_SIZE(b); i++)
		b[i] = __raw_readb(rtspi->win_uc + a + i);
	for (i = 0; i < 16; i += 4)
		if (get_unaligned_le32(mmio + i) != get_unaligned_be32(pio + i))
			break;
	if (!memchr_inv(mmio, 0xff, 16) || !memchr_inv(mmio, 0x00, 16))
		snprintf(pat, sizeof(pat), "window all %02x", mmio[0]);
	else if (!memcmp(b, pio, 16))
		strscpy(pat, "byte reads == PIO, words differ");
	else if (i == 16)
		strscpy(pat, "words byte-swapped");
	for (s = 1; s <= 8 && !strcmp(pat, "none"); s++) {
		if (!memcmp(mmio + s, pio, len - s))
			snprintf(pat, sizeof(pat), "window has %d extra leading bytes", s);
		else if (!memcmp(mmio, pio + s, len - s))
			snprintf(pat, sizeof(pat), "window skips first %d bytes", s);
	}
	dev_warn(rtspi->dev, "dirmap: diag at 0x%x op 0x%02x/%u/%u, window %pa at 0x%lx, release %d\n",
		 a, op->cmd.opcode, op->addr.nbytes, op->dummy.nbytes, &rtspi->win_phys,
		 (unsigned long)rtspi->win_uc, rel);
	dev_warn(rtspi->dev, "dirmap: pio  %16ph\n", pio);
	dev_warn(rtspi->dev, "dirmap: mmio %16ph\n", mmio);
	dev_warn(rtspi->dev, "dirmap: mmio bytes %16ph\n", b);
	dev_warn(rtspi->dev,
		 "dirmap: SFCR 0x%08x SFCR2 0x%08x SFCSR 0x%08x, boot SFCR 0x%08x SFCSR 0x%08x\n",
		 __raw_readl(REG(RTL_SPI_SFCR)), __raw_readl(REG(RTL_SPI_SFCR2)), sfcsr,
		 rtspi->boot_sfcr, rtspi->boot_sfcsr);
	dev_warn(rtspi->dev, "dirmap: pattern %s, mio_pio_err %u idle %u cs_sts %u%u\n", pat,
		 !!(sfcsr & RTL_SPI_SFCSR_MIO_PIO_ERR), !!(sfcsr & RTL_SPI_SFCSR_IDLE),
		 !!(sfcsr & RTL_SPI_SFCSR_CSB0_STS), !!(sfcsr & RTL_SPI_SFCSR_CSB1_STS));
}

/* PIO vs window at @a; 0 = equal, 1 = differ (diag logged), <0 = PIO error */
static int rtspi_dirmap_check(struct rtspi *rtspi, const struct spi_mem_op *op, u32 a)
{
	u8 pio[64], mmio[64];
	int ret, rel;

	ret = rtspi_pio_read(rtspi, op, a, pio, sizeof(pio));
	if (ret)
		return ret;
	rel = rtspi_mmio_read(rtspi, a, mmio, sizeof(mmio), false);
	if (rel)	/* not released: read anyway, the diag shows what the window returns */
		memcpy_fromio(mmio, rtspi->win_uc + a, sizeof(mmio));
	if (!rel && !memcmp(pio, mmio, sizeof(pio)))
		return 0;
	rtspi_dirmap_diag(rtspi, op, a, pio, mmio, sizeof(pio), rel);
	return 1;
}

static int rtspi_dirmap_create(struct spi_mem_dirmap_desc *desc)
{
	struct rtspi *rtspi = spi_controller_get_devdata(desc->mem->spi->controller);
	const struct spi_mem_op *op = desc->info.op_tmpl;
	u64 offs[] = { 0, desc->info.length / 2 };
	int i, ret;

	if (!rtspi->win_uc || op->data.dir != SPI_MEM_DATA_IN)
		return -EOPNOTSUPP;
	/* the window sends 4 address bytes (SFCR2 bit 9): only valid if the op does too */
	if (op->cmd.nbytes != 1 || op->addr.nbytes != 4 || op->dummy.nbytes > 4 ||
	    op->cmd.buswidth != 1 || op->addr.buswidth != 1 || op->data.buswidth != 1 ||
	    (op->dummy.nbytes && op->dummy.buswidth != 1) || op->cmd.dtr || op->addr.dtr ||
	    op->dummy.dtr || op->data.dtr)
		return -EOPNOTSUPP;
	if (desc->info.offset + desc->info.length > rtspi->win_size)
		return -EOPNOTSUPP;

	/* self-check: window == PIO at the start and in the upper half (4-byte addressing) */
	for (i = 0; i < ARRAY_SIZE(offs); i++) {
		u32 a = desc->info.offset + offs[i];

		ret = rtspi_dirmap_check(rtspi, op, a);
		if (ret > 0 && !rtspi->mmio_sfcsr) {
			/* second try: SFCSR as the bootloader left it (init_hw() rewrites it) */
			rtspi->mmio_sfcsr = rtspi->boot_sfcsr |
					    RTL_SPI_SFCSR_CSB0 | RTL_SPI_SFCSR_CSB1;
			ret = rtspi_dirmap_check(rtspi, op, a);
			if (!ret)
				dev_info(rtspi->dev, "dirmap: window == PIO with boot SFCSR 0x%08x\n",
					 rtspi->mmio_sfcsr);
		}
		if (ret < 0)
			return ret;
		if (ret) {
			rtspi->mmio_sfcsr = 0;
			dev_warn(rtspi->dev, "dirmap: window != PIO at 0x%x (SFCR2 0x%08x), using PIO\n",
				 a, __raw_readl(REG(RTL_SPI_SFCR2)));
			return -EOPNOTSUPP;
		}
	}
	dev_info(rtspi->dev, "dirmap: 0x%llx+0x%llx via window, SFCR2 0x%08x, op 0x%02x, mode %d\n",
		 desc->info.offset, desc->info.length, __raw_readl(REG(RTL_SPI_SFCR2)),
		 op->cmd.opcode, READ_ONCE(dirmap));
	return 0;
}

static ssize_t rtspi_dirmap_read(struct spi_mem_dirmap_desc *desc, u64 offs, size_t len,
				 void *buf)
{
	struct rtspi *rtspi = spi_controller_get_devdata(desc->mem->spi->controller);
	u32 addr = desc->info.offset + offs;
	int mode = READ_ONCE(dirmap), ret;

	if (mode <= 0)
		ret = rtspi_pio_read(rtspi, desc->info.op_tmpl, addr, buf, len);
	else
		ret = rtspi_mmio_read(rtspi, addr, buf, len, mode >= 2 && rtspi->win_wb);
	return ret ?: len;
}

static const struct spi_controller_mem_ops rtspi_mem_ops = {
	.dirmap_create = rtspi_dirmap_create,
	.dirmap_read = rtspi_dirmap_read,
};

static void init_hw(struct rtspi *rtspi)
{
	u32 value;

	/* Turn on big-endian byte ordering */
	value = __raw_readl(REG(RTL_SPI_SFCR));
	value |= RTL_SPI_SFCR_RBO | RTL_SPI_SFCR_WBO;
	__raw_writel(value, REG(RTL_SPI_SFCR));

	/* CHIP_SEL is only used in MMIO mode; CSB0/CSB1 are active low. */
	value = 0;
	value |= RTL_SPI_SFCSR_CSB0 | RTL_SPI_SFCSR_CSB1;
	__raw_writel(value, REG(RTL_SPI_SFCSR));
}

static int realtek_rtl_spi_probe(struct platform_device *pdev)
{
	struct spi_controller *ctrl;
	struct rtspi *rtspi;
	struct resource *res;
	int err;

	ctrl = devm_spi_alloc_host(&pdev->dev, sizeof(*rtspi));
	if (!ctrl) {
		dev_err(&pdev->dev, "Error allocating SPI controller\n");
		return -ENOMEM;
	}
	platform_set_drvdata(pdev, ctrl);
	rtspi = spi_controller_get_devdata(ctrl);

	rtspi->base = devm_platform_get_and_ioremap_resource(pdev, 0, NULL);
	if (IS_ERR(rtspi->base)) {
		dev_err(&pdev->dev, "Could not map SPI register address");
		return -ENOMEM;
	}

	rtspi->dev = &pdev->dev;
	rtspi->boot_sfcr = __raw_readl(REG(RTL_SPI_SFCR));
	rtspi->boot_sfcsr = __raw_readl(REG(RTL_SPI_SFCSR));

	/* optional flash read window: spi-mem dirmap reads; writes/erase stay on PIO */
	res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "dirmap");
	if (res) {
		rtspi->win_uc = devm_ioremap_resource(&pdev->dev, res);
		if (IS_ERR(rtspi->win_uc))
			return PTR_ERR(rtspi->win_uc);
		rtspi->win_size = resource_size(res);
		rtspi->win_phys = res->start;
		rtspi->win_wb = devm_memremap(&pdev->dev, res->start, rtspi->win_size, MEMREMAP_WB);
		if (IS_ERR(rtspi->win_wb)) {
			dev_warn(&pdev->dev, "dirmap: no cached mapping, uncached only\n");
			rtspi->win_wb = NULL;
		}
		ctrl->mem_ops = &rtspi_mem_ops;
	}

	init_hw(rtspi);

	ctrl->flags = SPI_CONTROLLER_HALF_DUPLEX;
	ctrl->set_cs = rt_set_cs;
	ctrl->transfer_one = transfer_one;
	ctrl->num_chipselect = 2;

	err = devm_spi_register_controller(&pdev->dev, ctrl);
	if (err) {
		dev_err(&pdev->dev, "Could not register SPI controller\n");
		return -ENODEV;
	}

	return 0;
}


static const struct of_device_id realtek_rtl_spi_of_ids[] = {
	{ .compatible = "realtek,rtl8380-spi" },
	{ .compatible = "realtek,rtl8382-spi" },
	{ .compatible = "realtek,rtl8391-spi" },
	{ .compatible = "realtek,rtl8392-spi" },
	{ .compatible = "realtek,rtl8393-spi" },
	{ /* sentinel */ }
};
MODULE_DEVICE_TABLE(of, realtek_rtl_spi_of_ids);

static struct platform_driver realtek_rtl_spi_driver = {
	.probe = realtek_rtl_spi_probe,
	.driver = {
		.name = "realtek-rtl-spi",
		.of_match_table = realtek_rtl_spi_of_ids,
	},
};

module_platform_driver(realtek_rtl_spi_driver);

MODULE_LICENSE("GPL v2");
MODULE_AUTHOR("Bert Vermeulen <bert@biot.com>");
MODULE_DESCRIPTION("Realtek RTL SPI driver");
