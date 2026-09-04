// SPDX-License-Identifier: GPL-2.0-only
/*
 * Amazon Annapurna Labs Alpine AHCI SATA driver.
 *
 * Copyright 2020 Pierre Bourdon <delroth@gmail.com>
 * Copyright 2026 Daniel Tyrrell <dan@awto.au> (Awto)
 *
 * Based on AHCI driver modifications in Annapurna Labs' 4.2.8 kernel fork
 * released by QNAP.
 *
 * Exists because ahci.c's board_ahci_al sets AHCI_HFLAG_NO_MSI, putting every
 * port of a controller on one shared INTx line.
 */

#include <linux/bits.h>
#include <linux/device.h>
#include <linux/kernel.h>
#include <linux/libata.h>
#include <linux/module.h>
#include <linux/msi.h>
#include <linux/pci.h>
#include <linux/suspend.h>
#include <scsi/scsi_host.h>
#include "ahci.h"

#define DRV_NAME "ahci-alpine"
#define DRV_VERSION "1.0"

enum {
	AHCI_PCI_BAR			= 5,

	/* MSI-X entries 0-2 are the IOFIC group summaries. */
	AL_INT_MSIX_RX_COMPLETION_START	= 3,

	/* IOFIC group B: cause bit N == AHCI port N. */
	IOFIC_BASE			= 0x2000,
	IOFIC_GROUP_B_INT_MASK_CLEAR	= 0x58,
	IOFIC_GROUP_B_INT_CTRL		= 0x68,

	INT_CTRL_CLEAR_ON_READ		= BIT(0),
	INT_CTRL_AUTO_MASK		= BIT(1),
	INT_CTRL_AUTO_CLEAR		= BIT(2),
	INT_CTRL_SET_ON_POSEDGE		= BIT(3),
	INT_CTRL_MOD_RES_SHIFT		= 24,

	/* Moderation-timer resolution only; the interval stays 0 (no
	 * coalescing), so this never delays a completion.
	 */
	MOD_RES_VAL			= 15,
};

static unsigned int mask_port_map;
module_param(mask_port_map, uint, 0444);
MODULE_PARM_DESC(mask_port_map,
		 "32-bit port map mask applied to every Alpine AHCI controller; "
		 "ports whose bit is clear become dummy ports. 0 (default) "
		 "keeps the controller's own PORTS_IMPL.");

/* IOFIC unmask semantics: a 0 bit clears that cause bit's mask. */
static void ahci_alpine_unmask_interrupts(struct ahci_host_priv *hpriv,
					  int from, int to)
{
	writel(~GENMASK(to, from),
	       hpriv->mmio + IOFIC_BASE + IOFIC_GROUP_B_INT_MASK_CLEAR);
}

/*
 * Configure the unit adapter's IOFIC, then take the per-port MSI-X entries
 * that sit above the three reserved group-summary ones.
 */
static int ahci_alpine_init_msix(struct pci_dev *pdev, unsigned int nr_ports,
				 struct ahci_host_priv *hpriv)
{
	struct msix_entry msix_entries[AHCI_MAX_PORTS];
	unsigned int i, need;
	int rc, have;
	u32 ctrl;

	need = AL_INT_MSIX_RX_COMPLETION_START + nr_ports;
	have = pci_msix_vec_count(pdev);
	if (have < (int)need) {
		dev_err(&pdev->dev, "MSI-X table has %d entries, need %u\n",
			have, need);
		return have < 0 ? have : -ENOSPC;
	}

	ctrl = INT_CTRL_CLEAR_ON_READ | INT_CTRL_AUTO_MASK |
	       INT_CTRL_AUTO_CLEAR | INT_CTRL_SET_ON_POSEDGE |
	       (MOD_RES_VAL << INT_CTRL_MOD_RES_SHIFT);
	writel(ctrl, hpriv->mmio + IOFIC_BASE + IOFIC_GROUP_B_INT_CTRL);

	for (i = 0; i < nr_ports; i++)
		msix_entries[i].entry = AL_INT_MSIX_RX_COMPLETION_START + i;

	rc = pci_enable_msix_exact(pdev, msix_entries, nr_ports);
	if (rc) {
		dev_err(&pdev->dev, "failed to enable MSI-X, err %d\n", rc);
		return rc;
	}

	ahci_alpine_unmask_interrupts(hpriv, 0, nr_ports - 1);
	return 0;
}

static int ahci_alpine_get_irq_vector(struct ata_host *host, int port)
{
	return pci_irq_vector(to_pci_dev(host->dev),
			      AL_INT_MSIX_RX_COMPLETION_START + port);
}

/*
 * AUTO_MASK makes the IOFIC mask the cause bit on delivery, and nothing else
 * clears the host-level IS bit on a per-port vector, so do both here.
 */
static irqreturn_t ahci_alpine_irq_handler(int irq, void *dev_instance)
{
	struct ata_port *ap = dev_instance;
	struct ahci_host_priv *hpriv = ap->host->private_data;

	spin_lock(ap->lock);
	ahci_handle_port_intr(ap->host, 1 << ap->port_no);
	spin_unlock(ap->lock);

	writel(1 << ap->port_no, hpriv->mmio + HOST_IRQ_STAT);
	ahci_alpine_unmask_interrupts(hpriv, ap->port_no, ap->port_no);

	return IRQ_HANDLED;
}

/*
 * Same shape as libahci's ahci_host_activate_multi_irqs(), which hardcodes its
 * own handler.
 */
static int ahci_alpine_host_activate(struct ata_host *host,
				     const struct scsi_host_template *sht)
{
	int i, rc;

	rc = ata_host_start(host);
	if (rc)
		return rc;

	for (i = 0; i < host->n_ports; i++) {
		struct ahci_port_priv *pp = host->ports[i]->private_data;
		int irq = ahci_alpine_get_irq_vector(host, i);

		/* Do not receive interrupts sent by dummy ports */
		if (!pp) {
			disable_irq(irq);
			continue;
		}

		rc = devm_request_irq(host->dev, irq, ahci_alpine_irq_handler,
				      0, pp->irq_desc, host->ports[i]);
		if (rc)
			return rc;
		ata_port_desc_misc(host->ports[i], irq);
	}

	return ata_host_register(host, sht);
}

/* Verbatim from ahci.c: hotplug-capable ports must not get LPM. */
static void ahci_alpine_mark_external_port(struct ata_port *ap)
{
	struct ahci_host_priv *hpriv = ap->host->private_data;
	void __iomem *port_mmio = ahci_port_base(ap);
	u32 tmp;

	tmp = readl(port_mmio + PORT_CMD);
	if (((tmp & PORT_CMD_ESP) && (hpriv->cap & HOST_CAP_SXS)) ||
	    (tmp & PORT_CMD_HPCP))
		ap->pflags |= ATA_PFLAG_EXTERNAL;
}

/* Verbatim from ahci.c, minus its mobile_lpm_policy module param. */
static void ahci_alpine_update_initial_lpm_policy(struct ata_port *ap)
{
	struct ahci_host_priv *hpriv = ap->host->private_data;
	int policy = CONFIG_SATA_MOBILE_LPM_POLICY;

	if (ap->pflags & ATA_PFLAG_EXTERNAL) {
		ap->flags |= ATA_FLAG_NO_LPM;
		ap->target_lpm_policy = ATA_LPM_MAX_POWER;
		return;
	}

	/* If no Partial or no Slumber, we cannot support DIPM. */
	if ((ap->host->flags & ATA_HOST_NO_PART) ||
	    (ap->host->flags & ATA_HOST_NO_SSC))
		ap->flags |= ATA_FLAG_NO_DIPM;

	if ((ap->host->flags & ATA_HOST_NO_PART) &&
	    (ap->host->flags & ATA_HOST_NO_SSC) &&
	    (ap->host->flags & ATA_HOST_NO_DEVSLP)) {
		ap->flags |= ATA_FLAG_NO_LPM;
		ap->target_lpm_policy = ATA_LPM_MAX_POWER;
		return;
	}

	if (policy > ATA_LPM_MED_POWER && pm_suspend_default_s2idle()) {
		if (hpriv->cap & HOST_CAP_PART)
			policy = ATA_LPM_MIN_POWER_WITH_PARTIAL;
		else if (hpriv->cap & HOST_CAP_SSC)
			policy = ATA_LPM_MIN_POWER;
	}

	if (policy >= ATA_LPM_UNKNOWN && policy <= ATA_LPM_MIN_POWER)
		ap->target_lpm_policy = policy;
}

static const struct ata_port_info ahci_alpine_ata_port_info = {
	AHCI_HFLAGS(AHCI_HFLAG_NO_PMP | AHCI_HFLAG_MULTI_MSI),
	.flags		= AHCI_FLAG_COMMON,
	.pio_mask	= ATA_PIO4,
	.udma_mask	= ATA_UDMA6,
	.port_ops	= &ahci_ops,
};

static const struct scsi_host_template ahci_alpine_sht = {
	AHCI_SHT(DRV_NAME),
};

static int ahci_alpine_init_one(struct pci_dev *pdev,
				const struct pci_device_id *ent)
{
	struct device *dev = &pdev->dev;
	struct ata_port_info pi = ahci_alpine_ata_port_info;
	const struct ata_port_info *ppi[] = { &pi, NULL };
	struct ahci_host_priv *hpriv;
	struct ata_host *host;
	int rc, i, n_ports;

	ata_print_version_once(dev, DRV_VERSION);

	rc = pcim_enable_device(pdev);
	if (rc)
		return rc;

	/* AHCI controllers often implement an SFF compatible interface, so
	 * grab all PCI BARs just in case.
	 */
	rc = pcim_request_all_regions(pdev, DRV_NAME);
	if (rc == -EBUSY)
		pcim_pin_device(pdev);
	if (rc)
		return rc;

	hpriv = devm_kzalloc(dev, sizeof(*hpriv), GFP_KERNEL);
	if (!hpriv)
		return -ENOMEM;
	hpriv->flags |= (unsigned long)pi.private_data;
	hpriv->mask_port_map = mask_port_map;

	hpriv->mmio = pcim_iomap(pdev, AHCI_PCI_BAR, 0);
	if (!hpriv->mmio)
		return -ENOMEM;

	ahci_save_initial_config(dev, hpriv);

	if (hpriv->cap & HOST_CAP_NCQ)
		pi.flags |= ATA_FLAG_NCQ | ATA_FLAG_FPDMA_AA | ATA_FLAG_FPDMA_AUX;

	ahci_set_em_messages(hpriv, &pi);

	/* CAP.NP can be either the last enabled or the last possible port, so
	 * the real maximum needs both it and the port map.
	 */
	n_ports = max(ahci_nr_ports(hpriv->cap), fls(hpriv->port_map));

	host = ata_host_alloc_pinfo(dev, ppi, n_ports);
	if (!host)
		return -ENOMEM;
	host->private_data = hpriv;

	rc = ahci_alpine_init_msix(pdev, n_ports, hpriv);
	if (rc)
		return rc;
	hpriv->get_irq_vector = ahci_alpine_get_irq_vector;

	if (!(hpriv->cap & HOST_CAP_SSS) || ahci_ignore_sss)
		host->flags |= ATA_HOST_PARALLEL_SCAN;
	else
		dev_info(dev, "SSS flag set, parallel bus scan disabled\n");

	if (!(hpriv->cap & HOST_CAP_PART))
		host->flags |= ATA_HOST_NO_PART;
	if (!(hpriv->cap & HOST_CAP_SSC))
		host->flags |= ATA_HOST_NO_SSC;
	if (!(hpriv->cap2 & HOST_CAP2_SDS))
		host->flags |= ATA_HOST_NO_DEVSLP;

	if (pi.flags & ATA_FLAG_EM)
		ahci_reset_em(host);

	for (i = 0; i < host->n_ports; i++) {
		struct ata_port *ap = host->ports[i];

		ata_port_pbar_desc(ap, AHCI_PCI_BAR, -1, "abar");
		ata_port_pbar_desc(ap, AHCI_PCI_BAR,
				   0x100 + ap->port_no * 0x80, "port");

		if (ap->flags & ATA_FLAG_EM)
			ap->em_message_type = hpriv->em_msg_type;

		/* disabled/not-implemented port */
		if (!(hpriv->port_map & (1 << i))) {
			ap->ops = &ata_dummy_port_ops;
		} else {
			ahci_alpine_mark_external_port(ap);
			ahci_alpine_update_initial_lpm_policy(ap);
		}
	}

	rc = dma_set_mask_and_coherent(dev,
		DMA_BIT_MASK((hpriv->cap & HOST_CAP_64) ? 64 : 32));
	if (rc) {
		dev_err(dev, "DMA enable failed\n");
		return rc;
	}

	rc = ahci_reset_controller(host);
	if (rc)
		return rc;

	ahci_init_controller(host);
	ahci_print_info(host, "SATA");

	pci_set_master(pdev);

	return ahci_alpine_host_activate(host, &ahci_alpine_sht);
}

static const struct pci_device_id ahci_alpine_pci_tbl[] = {
	{ PCI_DEVICE(PCI_VENDOR_ID_AMAZON_ANNAPURNA_LABS, 0x0031),
	  .class = PCI_CLASS_STORAGE_SATA_AHCI, .class_mask = 0xffffff },

	{},
};

static struct pci_driver ahci_alpine_pci_driver = {
	.name		= DRV_NAME,
	.id_table	= ahci_alpine_pci_tbl,
	.probe		= ahci_alpine_init_one,
	.remove		= ata_pci_remove_one,
	.shutdown	= ata_pci_shutdown_one,
};

module_pci_driver(ahci_alpine_pci_driver);

MODULE_DESCRIPTION("Amazon Annapurna Labs Alpine AHCI SATA driver");
MODULE_AUTHOR("Pierre Bourdon <delroth@gmail.com>");
MODULE_LICENSE("GPL");
MODULE_DEVICE_TABLE(pci, ahci_alpine_pci_tbl);
MODULE_ALIAS("ahci:alpine");
