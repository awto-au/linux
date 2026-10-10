// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2024 Allied Telesis
 */

#include <linux/errno.h>
#include <linux/libfdt.h>
#include <linux/printk.h>
#include <linux/types.h>

#include <asm/fw/fw.h>
#include <asm/machine.h>

#include <mach-rtl-otto.h>

#define RTL_SOC_BASE			((volatile void *) 0xB8000000)

#define RTL83XX_DRAM_CONFIG		0x1004

#define RTL9300_SRAMSAR0		0x4000
#define RTL9300_SRAMSAR1		0x4010
#define RTL9300_SRAMSAR2		0x4020
#define RTL9300_SRAMSAR3		0x4030
#define RTL9300_UMSAR0			0x1300
#define RTL9300_UMSAR1			0x1310
#define RTL9300_UMSAR2			0x1320
#define RTL9300_UMSAR3			0x1330
#define RTL9300_O0DOR2			0x4220
#define RTL9300_O0DMAR2			0x4224
#define RTL9300_MCR			0x1000
#define RTL9300_MCR_IPREF		BIT(23)
#define RTL9300_MCR_DPREF		BIT(22)
#define RTL9300_SRAM_BUS		0x1f000000
#define RTL9300_SEG_EN			BIT(0)
#define RTL9300_SEG_SIZE_64K		9
#define RTL9300_SRAMSBR0_PLR		0x00030000

#define RTL931X_DRAM_CONFIG		0x14304c

#define soc_r32(reg)			__raw_readl(RTL_SOC_BASE + reg)
#define soc_w32(val, reg)		__raw_writel(val, RTL_SOC_BASE + reg)

struct realtek_soc_data {
	u32 model_info_reg;
	u32 family;
	int cpu_port;
	void (*read_details)(u32 model);
	void (*apply_quirks)(void);
	void (*get_memory)(void);
	void (*prepare_highmem)(void);
};

struct rtl83xx_soc_info soc_info;

#define RTL930X_PLL_CPU_CTRL0		0xe208	/* CPU_CMU_NCODE_IN 11:4, CPU_CMU_SEL_DIV4 2, CPU_CMU_SEL_PREDIV 1:0 */
#define RTL930X_PLL_CPU_MISC_CTRL	0xe210	/* REG_CMU_DIVN2_CPU 6:4 */

/* CPU clock from the CPU PLL: 25 MHz x (4 if SEL_DIV4) x (NCODE + 2) x 2 / 2^PREDIV / (DIVN2 + 2) */
static u32 __init rtl930x_cpu_freq(void)
{
	u32 ctrl0 = sw_r32(RTL930X_PLL_CPU_CTRL0), ctrl2 = sw_r32(RTL930X_PLL_CPU_MISC_CTRL);
	u32 khz = 25000 * ((ctrl0 & BIT(2)) ? 4 : 1) * (((ctrl0 >> 4) & 0xff) + 2) * 2;

	khz >>= ctrl0 & 3;
	khz /= ((ctrl2 >> 4) & 7) + 2;
	return khz * 1000;
}

static unsigned int __init realtek_measure_hpt_freq(void)
{
	struct device_node *np;
	u32 freq = 500000000;

	/* the bootloader may have changed the PLL: the DT value would be stale */
	if (soc_info.family == RTL9300_FAMILY_ID) {
		freq = rtl930x_cpu_freq();
		pr_info("CPU frequency from PLL: %uMHz", freq / 1000000);
		return freq / 2;
	}

	np = of_find_node_by_name(NULL, "cpus");
	if (!np) {
		pr_err("Missing 'cpus' DT node, using default frequency.");
	} else {
		if (of_property_read_u32(np, "frequency", &freq) < 0)
			pr_err("No 'frequency' property in DT, using default.");
		else
			pr_info("CPU frequency from device tree: %dMHz", freq / 1000000);
		of_node_put(np);
	}

	return freq / 2;
}

static int __init realtek_apply_dts_quirks(void)
{
	struct device_node *node;

	node = of_find_compatible_node(NULL, NULL, "diodes,pt7a75xx-wdt");
	if (node) {
		if (soc_info.family == RTL9310_FAMILY_ID) {
			pr_info("apply quirk for diodes pt7a75xx watchdog\n");
			sw_w32_mask(GENMASK(13, 12), BIT(12), RTL931X_LED_GLB_CTRL);
			sw_w32_mask(0x0, BIT(8), RTL931X_MAC_L2_GLOBAL_CTRL2);
		};
		of_node_put(node);
	}

	return 0;
}

arch_initcall(realtek_apply_dts_quirks);

static void __init rtl838x_apply_early_quirks(void)
{
	/*
	 * Open up write protected registers. SDK opens/closes this whenever needed. For
	 * simplicity always work with an "open" register set.
	 */
	sw_w32(0x3, RTL838X_INT_RW_CTRL);
	/*
	 * Disable 4 byte address mode of flash controller. If this bit is not cleared
	 * the watchdog cannot reset the SoC. The SDK changes this short before restart.
	 * Until this quirk was implemented all RTL838x devices ran with this disabled
	 * because of a coding error. As no issues were detected keep the behaviour
	 * until more details are known.
	 */
	sw_w32_mask(BIT(30), 0, RTL838X_PLL_CML_CTRL);
}

static void __init rtl838x_read_details(u32 model)
{
	u32 chip_info, ext_version, tmp;

	sw_w32(0xa << 28, RTL838X_CHIP_INFO);

	chip_info = sw_r32(RTL838X_CHIP_INFO);
	soc_info.cpu = chip_info & 0xffff;

	ext_version = sw_r32(RTL838X_EXT_VERSION);
	tmp = ext_version & 0x1f;

	if (tmp == 2) {
		soc_info.revision = 1;
	} else {
		tmp = (chip_info >> 16) & 0x1f;
		if (soc_info.cpu == 0x0477) {
			soc_info.revision = tmp;
			soc_info.testchip = true;
		} else {
			soc_info.revision = tmp - 1;
		}
	}
}

static void __init rtl839x_read_details(u32 model)
{
	u32 chip_info;

	sw_w32(0xa << 28, RTL839X_CHIP_INFO);

	chip_info = sw_r32(RTL839X_CHIP_INFO);
	soc_info.cpu = chip_info & 0xffff;

	soc_info.revision = (model >> 1) & 0x1f;

	if (!(model & 0x3e))
		soc_info.testchip = true;
}

static void __init rtl93xx_read_details(u32 model)
{
	u32 chip_info;

	sw_w32(0xa << 16, RTL93XX_CHIP_INFO);

	chip_info = sw_r32(RTL93XX_CHIP_INFO);
	soc_info.cpu = chip_info & 0xffff;

	soc_info.revision = model & 0xf;

	if (model & 0x30)
		soc_info.testchip = true;
}

static void __init rtl96xx_read_details(u32 model)
{
	u32 chip_info, chip_subtype;

	sw_w32(0xa << 28, RTL96XX_CHIP_INFO);

	chip_info = sw_r32(RTL96XX_CHIP_INFO);
	soc_info.cpu = chip_info & 0xffff;

	sw_w32(0xb << 28, RTL96XX_CHIP_SUB_INFO);

	chip_subtype = sw_r32(RTL96XX_CHIP_SUB_INFO);
	soc_info.subtype = chip_subtype & 0x1f;

	soc_info.revision = model & 0xf;
}

static void __init realtek_parse_model(u32 model)
{
	int val;
	char suffix = 0;

	val = (model >> 11) & 0x1f;
	if (val > 0 && val <= 26)
		suffix = 'A' + (val - 1);

	snprintf(soc_info.name, sizeof(soc_info.name), "RTL%04X%c",
		 soc_info.id, suffix);
}

static void __init realtek_set_system_type(void)
{
	char revision = '?';
	char *es = "";
	char subtype[12] = "";

	if (soc_info.revision >= 0 && soc_info.revision < 26)
		revision = 'A' + soc_info.revision;

	if (soc_info.testchip)
		es = " ES";

	if (soc_info.subtype)
		snprintf(subtype, sizeof(subtype), " subtype %02X", soc_info.subtype);

	snprintf(soc_info.system_type, sizeof(soc_info.system_type),
		 "Realtek %s%s%s rev %c (%04X)",
		 soc_info.name, es, subtype, revision, soc_info.cpu);

	system_type = soc_info.system_type;
}

static void realtek_get_system_memory(void)
{
	unsigned int dcr, bits;

	dcr = soc_r32(RTL83XX_DRAM_CONFIG);
	bits = ((dcr >> 28) & 0x3) + ((dcr >> 24) & 0x3) +
	       ((dcr >> 20) & 0xf) + ((dcr >> 16) & 0xf) + 20;

	soc_info.memory_size = 1 << bits;
}

static void rtl931x_get_system_memory(void)
{
	unsigned int dcr, bits;

	dcr = soc_r32(RTL931X_DRAM_CONFIG);
	bits = (dcr >> 12) + ((dcr >> 6) & 0x3f) + (dcr & 0x3f);

	soc_info.memory_size = 1 << bits;
}

/*
 * The DRAM controller's instruction/data prefetch (MCR IP_REF/DP_REF) is left off by the
 * bootloader. With it on, cached reads and writes stop waiting on one line fill at a time.
 * Same enable sequence as the Realtek DRAM test: clear, then set.
 */
static void __init rtl930x_apply_quirks(void)
{
	u32 mcr = soc_r32(RTL9300_MCR) & ~(RTL9300_MCR_IPREF | RTL9300_MCR_DPREF);

	soc_w32(mcr, RTL9300_MCR);
	soc_w32(mcr | RTL9300_MCR_IPREF | RTL9300_MCR_DPREF, RTL9300_MCR);
	__sync();
	pr_info("RTL930x DRAM controller prefetch on (MCR %08x)\n", soc_r32(RTL9300_MCR));
}

static void rtl930x_prepare_highmem(void)
{
	if ((soc_info.memory_size <= 256 * 1024 * 1024) ||
	    !IS_ENABLED(CONFIG_HIGHMEM))
		return;

	/*
	 * The RTL930x provides 3 logical adressing zones that can be configured individually
	 * and offer an additional memory access indirection. Memory is accessed via the OCP
	 * bus that checks these regions and maps logical addresses to physical ones. They are
	 *
	 * zone 1: logical address 0x00000000-0x0fffffff (256 MB) - map register 0xb8004200
	 * zone 2: logical address 0x10000000-0x13ffffff (64 MB)  - map register 0xb8004210
	 * zone 3: logical address 0x20000000-0x9fffffff (2 GB)   - map register 0xb8004220
	 *
	 * Whenever CPU accesses memory the normal MIPS translation is applied and afterwards
	 * the bus adds the zone mapping. E.g. a read to 0x81230000 is converted to an cached
	 * memory access to logical address 0x01230000. It is issued to the OCP bus and the
	 * mapping from zone 1 register is added. That allows for two memory topologies:
	 *
	 * Linear memory with a maximum of 320 MB:
	 *
	 * Zone | map content | logical               | physical
	 * -------------------+-----------------------+-----------------------
	 *    1 | 0x00000000  | 0x00000000-0x0fffffff | 0x00000000-0x0fffffff
	 *    2 | 0x00000000  | 0x10000000-0x13ffffff | 0x10000000-0x13ffffff
	 *
	 * 256MB low memory plus up to 2GB high memory:
	 *
	 * Zone | map content | logical               | physical
	 * -------------------+-----------------------+-----------------------
	 *    1 | 0x00000000  | 0x00000000-0x0fffffff | 0x00000000-0x0fffffff
	 *    3 | 0x70000000  | 0x20000000-0x9fffffff | 0x10000000-0x7fffffff
	 */

	pr_info("highmem kernel with > 256 MB RAM, adapt SoC memory mapping\n");

	soc_w32(0, RTL9300_UMSAR0);
	soc_w32(0, RTL9300_UMSAR1);
	soc_w32(0, RTL9300_UMSAR2);
	soc_w32(0, RTL9300_UMSAR3);
	soc_w32(0, RTL9300_SRAMSAR0);
	soc_w32(0, RTL9300_SRAMSAR1);
	soc_w32(0, RTL9300_SRAMSAR2);
	soc_w32(0, RTL9300_SRAMSAR3);
	__sync();

	soc_w32(0x70000000, RTL9300_O0DOR2);
	soc_w32(0x7fffffff, RTL9300_O0DMAR2);
	__sync();
}

/*
 * 64 KiB on-chip SRAM at bus 0x1f000000 (DT mmio-sram node), outside zones 1-3; re-set after
 * the highmem remap above, which clears it. SRAM segment only: the PLR's unmap segment
 * (UMSAR0) matches the DRAM address after zone translation, so on a 512 MiB board it would
 * punch a hole at phys 0x2f000000 whose accesses hang (#107). Enable last.
 */
static void __init rtl930x_map_sram(void)
{
	soc_w32(RTL9300_SEG_SIZE_64K, RTL9300_SRAMSAR0 + 4);
	soc_w32(RTL9300_SRAMSBR0_PLR, RTL9300_SRAMSAR0 + 8);
	soc_w32(RTL9300_SRAM_BUS | RTL9300_SEG_EN, RTL9300_SRAMSAR0);
	__sync();
}

static void __init rtl930x_prepare_memory(void)
{
	rtl930x_prepare_highmem();
	rtl930x_map_sram();
}

static void __init realtek_prom_init(const void *match_data)
{
	const struct realtek_soc_data *cfg = match_data;
	u32 model;

	if (!cfg)
		panic("No match_data found!");

	model = sw_r32(cfg->model_info_reg);

	soc_info.id = model >> 16 & 0xffff;
	soc_info.family = cfg->family;
	soc_info.cpu_port = cfg->cpu_port;

	if (cfg->apply_quirks)
		cfg->apply_quirks();

	cfg->read_details(model);
	realtek_parse_model(model);
	realtek_set_system_type();
	cfg->get_memory();

	if (cfg->prepare_highmem)
		cfg->prepare_highmem();

	pr_info("%s SoC with %d MB\n", get_system_type(), soc_info.memory_size >> 20);

	/*
	 * fw_arg2 is the pointer to the environment. Some devices (e.g. HP JG924A) hand
	 * over other than expected kernel boot arguments. Something like 0xfffdffff looks
	 * suspicous. Do extra cleanup for fw_init_cmdline() to avoid a hang during boot.
	 */
	if (fw_arg2 >= CKSEG2)
		fw_arg2 = 0;
}


static __init int realtek_add_initrd(void *fdt)
{
	int node, err;
	u32 start, size;

	node = fdt_path_offset(fdt, "/chosen");
	if (node < 0) {
		pr_err("/chosen node not found\n");
		return -ENOENT;
	}

	start = fw_getenvl("initrd_start");
	size = fw_getenvl("initrd_size");

	if (start == 0 && size == 0)
		return 0;

	pr_info("Adding initrd info from environment\n");

	err = fdt_setprop_u32(fdt, node, "linux,initrd-start", start);
	if (err) {
		pr_err("unable to set initrd-start: %d\n", err);
		return err;
	}

	err = fdt_setprop_u32(fdt, node, "linux,initrd-end", start + size);
	if (err) {
		pr_err("unable to set initrd-end: %d\n", err);
		return err;
	}

	return 0;
}

#define RTL930X_PLL_SW_CTRL0		0xe214	/* same layout as CPU_CTRL0 */
#define RTL930X_PLL_SW_DIV_CTRL		0xe220	/* REG_CMU_DIVN2_LXB 12:9 */

/* LX bus clock: switch PLL VCO / (2 x (DIVN2_LXB + 2)); 175 MHz stock */
static u32 __init rtl930x_lx_freq(void)
{
	u32 ctrl0 = sw_r32(RTL930X_PLL_SW_CTRL0);
	u32 khz = 25000 * ((ctrl0 & BIT(2)) ? 4 : 1) * (((ctrl0 >> 4) & 0xff) + 2) * 2;

	khz >>= ctrl0 & 3;
	return khz / (2 * (((sw_r32(RTL930X_PLL_SW_DIV_CTRL) >> 9) & 0xf) + 2)) * 1000;
}

/*
 * The bootloader may have changed the LX/CPU PLLs (st1008f `turbo`): the DT fixed-clocks are stale.
 * Patch the LX clock (UART, timer, watchdog) and the CPU clock to the live rates, and give the
 * UARTs clock-frequency so earlycon divides right too.
 */
static __init int realtek_fixup_clocks(void *fdt)
{
	u32 lx, cpu;
	int node, err;

	if (soc_info.family != RTL9300_FAMILY_ID)
		return 0;
	lx = rtl930x_lx_freq();
	cpu = rtl930x_cpu_freq();
	node = fdt_path_offset(fdt, "/clock-175mhz");
	if (node >= 0 && (err = fdt_setprop_u32(fdt, node, "clock-frequency", lx)))
		return err;
	node = fdt_path_offset(fdt, "/clock-800mhz");
	if (node >= 0 && (err = fdt_setprop_u32(fdt, node, "clock-frequency", cpu)))
		return err;
	for (node = fdt_node_offset_by_compatible(fdt, -1, "ns16550a"); node >= 0;
	     node = fdt_node_offset_by_compatible(fdt, node, "ns16550a"))
		if ((err = fdt_setprop_u32(fdt, node, "clock-frequency", lx)))
			return err;
	pr_info("LX frequency from PLL: %u kHz, CPU %u kHz\n", lx / 1000, cpu / 1000);
	return 0;
}

static const struct mips_fdt_fixup realtek_fdt_fixups[] __initconst = {
	{ realtek_fixup_clocks, "LX/CPU clocks from PLL" },
	{ realtek_add_initrd, "add initrd" },
	{},
};

static __init const void *realtek_fixup_fdt(const void *fdt, const void *match_data)
{
	static unsigned char fdt_buf[32 << 10] __initdata;
	int err;

	/*
	 * The RTL9607C PCIe controller has 2 io regions, with the 2nd one
	 * being at an offset from the 1st one. This does not work with
	 * MIPS specific pci_remap_iospace() as it expects the region start
	 * address to be 0, which is not the case for 2nd io region of
	 * RTL9607C PCIe controller. Thus set the io port base to KSEG1,
	 * a common base for majority of MIPS platforms, for the generic
	 * pci_remap_iospace() to work here.
	 */
	set_io_port_base(KSEG1);

	realtek_prom_init(match_data);

	if (fdt_check_header(fdt))
		panic("Corrupt DT");

	fw_init_cmdline();

	err = apply_mips_fdt_fixups(fdt_buf, sizeof(fdt_buf), fdt, realtek_fdt_fixups);
	if (err)
		panic("Unable to fixup FDT: %d", err);

	arcs_cmdline[0] = '\0';	/* plat_mem_setup() runs fw_init_cmdline() again */

	return fdt_buf;

}

static const struct realtek_soc_data rtl838x_soc __initconst = {
	.model_info_reg = RTL838X_MODEL_NAME_INFO,
	.family = RTL8380_FAMILY_ID,
	.cpu_port = RTL838X_CPU_PORT,
	.read_details = rtl838x_read_details,
	.apply_quirks = rtl838x_apply_early_quirks,
	.get_memory = realtek_get_system_memory,
};

static const struct realtek_soc_data rtl839x_soc __initconst = {
	.model_info_reg = RTL839X_MODEL_NAME_INFO,
	.family = RTL8390_FAMILY_ID,
	.cpu_port = RTL839X_CPU_PORT,
	.read_details = rtl839x_read_details,
	.get_memory = realtek_get_system_memory,
};

static const struct realtek_soc_data rtl930x_soc __initconst = {
	.model_info_reg = RTL93XX_MODEL_NAME_INFO,
	.family = RTL9300_FAMILY_ID,
	.cpu_port = RTL930X_CPU_PORT,
	.apply_quirks = rtl930x_apply_quirks,
	.read_details = rtl93xx_read_details,
	.get_memory = realtek_get_system_memory,
	.prepare_highmem = rtl930x_prepare_memory,
};

static const struct realtek_soc_data rtl931x_soc __initconst = {
	.model_info_reg = RTL93XX_MODEL_NAME_INFO,
	.family = RTL9310_FAMILY_ID,
	.cpu_port = RTL931X_CPU_PORT,
	.read_details = rtl93xx_read_details,
	.get_memory = rtl931x_get_system_memory,
};

static const struct realtek_soc_data rtl960x_soc __initconst = {
	.model_info_reg = RTL96XX_MODEL_NAME_INFO,
	.family = RTL9607_FAMILY_ID,
	.cpu_port = RTL9607_CPU_PORT,
	.read_details = rtl96xx_read_details,
	.get_memory = realtek_get_system_memory,
	.prepare_highmem = rtl930x_prepare_highmem,
};

static const struct of_device_id realtek_of_match[] __initconst = {
	{ .compatible = "realtek,rtl8380-soc", .data = &rtl838x_soc },
	{ .compatible = "realtek,rtl8381-soc", .data = &rtl838x_soc },
	{ .compatible = "realtek,rtl8382-soc", .data = &rtl838x_soc },
	{ .compatible = "realtek,rtl838x-soc", .data = &rtl838x_soc },
	{ .compatible = "realtek,rtl8391-soc", .data = &rtl839x_soc },
	{ .compatible = "realtek,rtl8392-soc", .data = &rtl839x_soc },
	{ .compatible = "realtek,rtl8393-soc", .data = &rtl839x_soc },
	{ .compatible = "realtek,rtl839x-soc", .data = &rtl839x_soc },
	{ .compatible = "realtek,rtl9301-soc", .data = &rtl930x_soc },
	{ .compatible = "realtek,rtl9302-soc", .data = &rtl930x_soc },
	{ .compatible = "realtek,rtl9302b-soc", .data = &rtl930x_soc },
	{ .compatible = "realtek,rtl9302c-soc", .data = &rtl930x_soc },
	{ .compatible = "realtek,rtl9303-soc", .data = &rtl930x_soc },
	{ .compatible = "realtek,rtl930x-soc", .data = &rtl930x_soc },
	{ .compatible = "realtek,rtl9311-soc", .data = &rtl931x_soc },
	{ .compatible = "realtek,rtl9312-soc", .data = &rtl931x_soc },
	{ .compatible = "realtek,rtl9313-soc", .data = &rtl931x_soc },
	{ .compatible = "realtek,rtl931x-soc", .data = &rtl931x_soc },
	{ .compatible = "realtek,rtl8198d-soc", .data = &rtl960x_soc },
	{ .compatible = "realtek,rtl9607-soc", .data = &rtl960x_soc },
	{ .compatible = "realtek,rtl9607c-soc", .data = &rtl960x_soc },
	{ .compatible = "realtek,rtl960x-soc", .data = &rtl960x_soc },
	{}
};

MIPS_MACHINE(realtek) = {
	.matches = realtek_of_match,
	.fixup_fdt = realtek_fixup_fdt,
	.measure_hpt_freq = realtek_measure_hpt_freq,
};
