# SPDX-License-Identifier: GPL-2.0-only
ccflags-y += -O3

obj-$(CONFIG_LZ4_COMPRESS) += lz4_compress.o
obj-$(CONFIG_LZ4HC_COMPRESS) += lz4hc_compress.o
ifdef CONFIG_RUST_C2RUST_BOOT_TEST
obj-$(CONFIG_LZ4_DECOMPRESS) += lz4_decompress_rs.o
else
obj-$(CONFIG_LZ4_DECOMPRESS) += lz4_decompress.o
endif
