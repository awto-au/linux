// SPDX-License-Identifier: GPL-2.0
/*
 * seq_buf_rs_shim.c
 *
 * seq_buf_printf() has a genuine C-variadic signature (`...`) and is
 * EXPORT_SYMBOL_GPL'd for real callers throughout the kernel (kernel/panic.c,
 * kernel/trace/trace_seq.c, lib/alloc_tag.c, ~38 call sites total). Rust
 * cannot *define* a C-variadic function without the `c_variadic` feature,
 * which is both unstable and outside this kernel's rust_allowed_features
 * allow-list (scripts/Makefile.build) — unlike raw_ref_op/extern_types/
 * strict_provenance, there is no stable substitute for defining (as opposed
 * to calling) one. This thin C shim provides the real variadic ABI entry
 * point and forwards to seq_buf_vprintf(), which lib/seq_buf_rs.rs defines
 * taking a plain __builtin_va_list (ABI-identical to C's va_list on this
 * arch) — no unstable Rust feature required on the Rust side.
 */

#include <linux/export.h>
#include <linux/seq_buf.h>
#include <linux/stdarg.h>

extern int seq_buf_vprintf(struct seq_buf *s, const char *fmt, va_list args);

int seq_buf_printf(struct seq_buf *s, const char *fmt, ...)
{
	va_list ap;
	int ret;

	va_start(ap, fmt);
	ret = seq_buf_vprintf(s, fmt, ap);
	va_end(ap);

	return ret;
}
EXPORT_SYMBOL_GPL(seq_buf_printf);
