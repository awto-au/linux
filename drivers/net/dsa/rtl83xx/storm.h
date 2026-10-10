/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef _RTL83XX_STORM_H
#define _RTL83XX_STORM_H

struct rtl838x_switch_priv;

/* RTL930x storm control defaults + loop detection (awto-tl-st1008f #111) */
void rtldsa_930x_storm_init(struct rtl838x_switch_priv *priv);
void rtldsa_930x_storm_remove(struct rtl838x_switch_priv *priv);

#endif
