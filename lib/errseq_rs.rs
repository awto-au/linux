#![deny(unsafe_op_in_unsafe_fn)]
#![allow(
    clippy::missing_safety_doc,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
use ::core::arch::{asm, global_asm};
use ::kernel::warn_on;
extern "C" {
    fn _printk(fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn __riscv_isa_extension_available(
        isa_bitmap: *const ::core::ffi::c_ulong,
        bit: ::core::ffi::c_uint,
    ) -> bool_0;
}
pub type __u32 = ::core::ffi::c_uint;
pub type __u64 = ::core::ffi::c_ulonglong;
pub type u32_0 = __u32;
pub type u64_0 = __u64;
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const r#true: C2Rust_Unnamed = 1;
pub const r#false: C2Rust_Unnamed = 0;
pub type __kernel_ulong_t = ::core::ffi::c_ulong;
pub type __kernel_size_t = __kernel_ulong_t;
pub type bool_0 = bool;
pub type uintptr_t = usize;
pub type size_t = __kernel_size_t;
pub type ulong = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct C2Rust_Unnamed_0(pub C2Rust_C2Rust_Unnamed_0_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_C2Rust_Unnamed_0_Inner {
    pub x: ::core::ffi::c_long,
}
#[allow(dead_code, non_upper_case_globals)]
const C2Rust_C2Rust_Unnamed_0_PADDING: usize = ::core::mem::size_of::<C2Rust_Unnamed_0>()
    - ::core::mem::size_of::<C2Rust_C2Rust_Unnamed_0_Inner>();
pub type errseq_t = u32_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {}
pub const BITS_PER_LONG: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn kasan_check_write(
    mut p: *const ::core::ffi::c_void,
    mut size: ::core::ffi::c_uint,
) -> bool_0 {
    unsafe {
        return r#true as ::core::ffi::c_int != 0;
    }
}
pub const KCSAN_ACCESS_WRITE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const KCSAN_ACCESS_COMPOUND: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const KCSAN_ACCESS_ATOMIC: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn kcsan_check_access(
    mut ptr: *const ::core::ffi::c_void,
    mut size: size_t,
    mut r#type: ::core::ffi::c_int,
) {
}
pub const BITS_PER_BYTE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const RISCV_ISA_EXT_ZBB: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const RISCV_ISA_EXT_ZACAS: ::core::ffi::c_int = 73 as ::core::ffi::c_int;
pub const RISCV_ISA_EXT_ZABHA: ::core::ffi::c_int = 90 as ::core::ffi::c_int;
pub const STANDARD_EXT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
// linux-rs: `__riscv_has_extension_{likely,unlikely}` are `static
// __always_inline` C functions built entirely from `asm goto(ALTERNATIVE(...))`
// (boot-time code-patching of a `j`/`nop` branch in the `.alternative`
// section) — c2rust cannot translate GNU asm-goto with label operands
// ("Cannot translate GNU asm goto (extended asm with label operands)",
// see transpile.log) and falls back to an unresolvable `extern "C"`
// declaration with no real linkable symbol anywhere in the kernel (every
// C call site is inlined). Routed to the same real, globally-linkable
// `__riscv_isa_extension_available()` the C header itself falls back to
// when `CONFIG_RISCV_ALTERNATIVE` is off — semantically identical query
// (same true/false answer; "likely"/"unlikely" was only ever a branch-
// prediction hint for the patched-branch fast path, not a correctness
// difference), just without the boot-patched-branch fast path.
#[inline(always)]
unsafe extern "C" fn riscv_has_extension_unlikely(ext: ::core::ffi::c_ulong) -> bool_0 {
    unsafe {
        extern "C" {
            #[link_name = "__compiletime_assert_0"]
            fn __compiletime_assert_0_0() -> !;
        }
        if !(ext < 128 as ::core::ffi::c_ulong) {
            __compiletime_assert_0_0();
        }
        return __riscv_isa_extension_available(::core::ptr::null(), ext as ::core::ffi::c_uint);
    }
}
#[inline(always)]
unsafe extern "C" fn riscv_has_extension_likely(ext: ::core::ffi::c_ulong) -> bool_0 {
    unsafe {
        extern "C" {
            #[link_name = "__compiletime_assert_1"]
            fn __compiletime_assert_1_0() -> !;
        }
        if !(ext < 128 as ::core::ffi::c_ulong) {
            __compiletime_assert_1_0();
        }
        return __riscv_isa_extension_available(::core::ptr::null(), ext as ::core::ffi::c_uint);
    }
}
#[inline(always)]
unsafe extern "C" fn instrument_atomic_check_alignment(
    mut v: *const ::core::ffi::c_void,
    mut size: size_t,
) {
}
#[inline(always)]
unsafe extern "C" fn instrument_atomic_read_write(
    mut v: *const ::core::ffi::c_void,
    mut size: size_t,
) {
    unsafe {
        kasan_check_write(v, size as ::core::ffi::c_uint);
        kcsan_check_access(
            v,
            size,
            KCSAN_ACCESS_ATOMIC | KCSAN_ACCESS_WRITE | KCSAN_ACCESS_COMPOUND,
        );
        instrument_atomic_check_alignment(v, size);
    }
}
#[inline(always)]
unsafe extern "C" fn generic___fls(mut word: ::core::ffi::c_ulong) -> ::core::ffi::c_uint {
    unsafe {
        return u64::BITS - 1 - word.leading_zeros();
    }
}
#[inline(always)]
unsafe extern "C" fn generic_fls(mut x: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    unsafe {
        return (u32::BITS - x.leading_zeros()) as ::core::ffi::c_int;
    }
}
#[inline(always)]
unsafe extern "C" fn variable__fls(mut word: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    unsafe {
        if !riscv_has_extension_likely(RISCV_ISA_EXT_ZBB as ::core::ffi::c_ulong) {
            return generic___fls(word) as ::core::ffi::c_ulong;
        }
        asm!(
            ".option push\n", ".option arch,+zbb\n", "clz {0}, {1}\n", ".option pop\n",
            "\n", lateout(reg) word, inlateout(reg) word => _, options(preserves_flags)
        );
        return ((BITS_PER_LONG - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
            .wrapping_sub(word);
    }
}
#[inline(always)]
unsafe extern "C" fn variable_fls(mut x: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    unsafe {
        if !riscv_has_extension_likely(RISCV_ISA_EXT_ZBB as ::core::ffi::c_ulong) {
            return generic_fls(x);
        }
        if x == 0 {
            return 0 as ::core::ffi::c_int;
        }
        asm!(
            ".option push\n", ".option arch,+zbb\n", "clzw {0}, {1}\n", ".option pop\n",
            "\n", lateout(reg) x, inlateout(reg) x => _, options(preserves_flags)
        );
        return (32 as ::core::ffi::c_uint).wrapping_sub(x) as ::core::ffi::c_int;
    }
}
#[inline(always)]
unsafe extern "C" fn fls64(mut x: __u64) -> ::core::ffi::c_int {
    unsafe {
        return (u64::BITS - x.leading_zeros()) as ::core::ffi::c_int;
    }
}
#[inline(always)]
unsafe extern "C" fn __ilog2_u32(mut n: u32_0) -> ::core::ffi::c_int {
    unsafe {
        return ({
            let mut x_: u32_0 = n as u32_0;
            (if false {
                (if x_ != 0 as u32_0 {
                    32 as ::core::ffi::c_int - x_.leading_zeros() as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })
            } else {
                variable_fls(x_ as ::core::ffi::c_uint)
            })
        }) - 1 as ::core::ffi::c_int;
    }
}
#[inline(always)]
unsafe extern "C" fn __ilog2_u64(mut n: u64_0) -> ::core::ffi::c_int {
    unsafe {
        return fls64(n as __u64) - 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn errseq_set(
    mut eseq: *mut errseq_t,
    mut err: ::core::ffi::c_int,
) -> errseq_t {
    unsafe {
        let mut cur: errseq_t = 0;
        let mut old: errseq_t = 0;
        old = ({
            extern "C" {
                #[link_name = "__compiletime_assert_28"]
                fn __compiletime_assert_28_0() -> !;
            }
            if !(::core::mem::size_of::<errseq_t>()
                == ::core::mem::size_of::<::core::ffi::c_char>()
                || ::core::mem::size_of::<errseq_t>()
                    == ::core::mem::size_of::<::core::ffi::c_short>()
                || ::core::mem::size_of::<errseq_t>()
                    == ::core::mem::size_of::<::core::ffi::c_int>()
                || ::core::mem::size_of::<errseq_t>()
                    == ::core::mem::size_of::<::core::ffi::c_long>()
                || ::core::mem::size_of::<errseq_t>()
                    == ::core::mem::size_of::<::core::ffi::c_longlong>())
            {
                __compiletime_assert_28_0();
            }
            ::core::ptr::read_volatile::<errseq_t>((eseq as *const errseq_t))
        }) as errseq_t;
        if kernel::warn_on!(
            (err == 0 as ::core::ffi::c_int
                || -err as ::core::ffi::c_uint > 4095 as ::core::ffi::c_uint)
                as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
        ) {
            return old;
        }
        loop {
            let mut new: errseq_t = 0;
            new = old
                & !(((1 as ::core::ffi::c_int)
                    << (if false {
                        (if (4095 as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int
                        } else {
                            63 as ::core::ffi::c_int
                                - (4095 as ::core::ffi::c_ulonglong).leading_zeros()
                                    as ::core::ffi::c_int
                        })
                    } else {
                        (if ::core::mem::size_of::<::core::ffi::c_int>() <= 4 as usize {
                            __ilog2_u32(4095 as u32_0)
                        } else {
                            __ilog2_u64(4095 as u64_0)
                        })
                    }) + 1 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int)
                        << (if false {
                            (if (4095 as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int
                            } else {
                                63 as ::core::ffi::c_int
                                    - (4095 as ::core::ffi::c_ulonglong).leading_zeros()
                                        as ::core::ffi::c_int
                            })
                        } else {
                            (if ::core::mem::size_of::<::core::ffi::c_int>() <= 4 as usize {
                                __ilog2_u32(4095 as u32_0)
                            } else {
                                __ilog2_u64(4095 as u64_0)
                            })
                        }) + 1 as ::core::ffi::c_int) as errseq_t
                | -err as errseq_t;
            if old
                & ((1 as ::core::ffi::c_int)
                    << (if false {
                        (if (4095 as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int
                        } else {
                            63 as ::core::ffi::c_int
                                - (4095 as ::core::ffi::c_ulonglong).leading_zeros()
                                    as ::core::ffi::c_int
                        })
                    } else {
                        (if ::core::mem::size_of::<::core::ffi::c_int>() <= 4 as usize {
                            __ilog2_u32(4095 as u32_0)
                        } else {
                            __ilog2_u64(4095 as u64_0)
                        })
                    }) + 1 as ::core::ffi::c_int) as errseq_t
                != 0
            {
                new = new.wrapping_add(
                    ((1 as ::core::ffi::c_int)
                        << (if false {
                            (if (4095 as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int
                            } else {
                                63 as ::core::ffi::c_int
                                    - (4095 as ::core::ffi::c_ulonglong).leading_zeros()
                                        as ::core::ffi::c_int
                            })
                        } else {
                            (if ::core::mem::size_of::<::core::ffi::c_int>() <= 4 as usize {
                                __ilog2_u32(4095 as u32_0)
                            } else {
                                __ilog2_u64(4095 as u64_0)
                            })
                        }) + 1 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as errseq_t,
                );
            }
            if new == old {
                cur = new;
                break;
            } else {
                cur = ({
                    let mut __ai_ptr: *mut errseq_t = eseq as *mut errseq_t;
                    instrument_atomic_read_write(
                        __ai_ptr as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<errseq_t>() as size_t,
                    );
                    ({
                        let mut __ptr: *mut errseq_t = __ai_ptr as *mut errseq_t;
                        let mut __old: errseq_t = old as errseq_t;
                        let mut __new: errseq_t = new as errseq_t;
                        let mut __ret: errseq_t = 0;
                        match ::core::mem::size_of::<errseq_t>() {
                            1 => {
                                '_c2rust_label_2: {
                                    if true
                                        && true
                                        && true
                                        && riscv_has_extension_unlikely(
                                            RISCV_ISA_EXT_ZABHA as ::core::ffi::c_ulong,
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                        && riscv_has_extension_unlikely(
                                            RISCV_ISA_EXT_ZACAS as ::core::ffi::c_ulong,
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        __ret = __old as errseq_t;
                                        asm!(
                                            "\tamocas.b.aqrl {0}, {2:}, 0({1})\n", "\n", inout(reg)
                                            __ret, in (reg) & raw mut * __ptr, inlateout(reg) __new =>
                                            _, options(preserves_flags)
                                        );
                                    } else {
                                        let mut __ptr32b: *mut u32_0 =
                                            ::core::ptr::with_exposed_provenance_mut::<u32_0>(
                                                (__ptr.expose_provenance() as ulong
                                                    & !(0x3 as ::core::ffi::c_int) as ulong)
                                                    as usize,
                                            );
                                        let mut __s: ulong =
                                            (__ptr.expose_provenance() as ulong
                                                & (0x4 as ulong).wrapping_sub(
                                                    ::core::mem::size_of::<errseq_t>() as ulong,
                                                ))
                                            .wrapping_mul(BITS_PER_BYTE as ulong);
                                        let mut __mask: ulong = (::core::mem::size_of::<
                                            C2Rust_Unnamed_2,
                                        >(
                                        )
                                            as ::core::ffi::c_int
                                            as ::core::ffi::c_ulong)
                                            .wrapping_add(
                                                ((1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                    << (8 as usize)
                                                        .wrapping_mul(::core::mem::size_of::<
                                                            ::core::ffi::c_ulong,
                                                        >(
                                                        ))
                                                        .wrapping_sub(1 as usize)
                                                        .wrapping_sub(
                                                            ((-1 as ::core::ffi::c_int
                                                                as ::core::ffi::c_ulong)
                                                                < 1 as ::core::ffi::c_int
                                                                    as ::core::ffi::c_ulong)
                                                                as ::core::ffi::c_int
                                                                as usize,
                                                        ))
                                                .wrapping_sub(1 as ::core::ffi::c_ulong)
                                                .wrapping_add(
                                                    (1 as ::core::ffi::c_int
                                                        as ::core::ffi::c_ulong)
                                                        << (8 as usize)
                                                            .wrapping_mul(::core::mem::size_of::<
                                                                ::core::ffi::c_ulong,
                                                            >(
                                                            ))
                                                            .wrapping_sub(1 as usize)
                                                            .wrapping_sub(
                                                                ((-1 as ::core::ffi::c_int
                                                                    as ::core::ffi::c_ulong)
                                                                    < 1 as ::core::ffi::c_int
                                                                        as ::core::ffi::c_ulong)
                                                                    as ::core::ffi::c_int
                                                                    as usize,
                                                            ),
                                                ) << 0 as ::core::ffi::c_int
                                                    & ((1 as ::core::ffi::c_int
                                                        as ::core::ffi::c_ulong)
                                                        << (8 as usize)
                                                            .wrapping_mul(::core::mem::size_of::<
                                                                ::core::ffi::c_ulong,
                                                            >(
                                                            ))
                                                            .wrapping_sub(1 as usize)
                                                            .wrapping_sub(
                                                                ((-1 as ::core::ffi::c_int
                                                                    as ::core::ffi::c_ulong)
                                                                    < 1 as ::core::ffi::c_int
                                                                        as ::core::ffi::c_ulong)
                                                                    as ::core::ffi::c_int
                                                                    as usize,
                                                            ))
                                                    .wrapping_sub(1 as ::core::ffi::c_ulong)
                                                    .wrapping_add(
                                                        (1 as ::core::ffi::c_int
                                                            as ::core::ffi::c_ulong)
                                                            << (8 as usize)
                                                                .wrapping_mul(
                                                                    ::core::mem::size_of::<
                                                                        ::core::ffi::c_ulong,
                                                                    >(
                                                                    ),
                                                                )
                                                                .wrapping_sub(1 as usize)
                                                                .wrapping_sub(
                                                                    ((-1 as ::core::ffi::c_int
                                                                        as ::core::ffi::c_ulong)
                                                                        < 1 as ::core::ffi::c_int
                                                                            as ::core::ffi::c_ulong)
                                                                        as ::core::ffi::c_int
                                                                        as usize,
                                                                ),
                                                    ) >> ::core::mem::size_of::<
                                                        ::core::ffi::c_ulong,
                                                    >(
                                                    )
                                                    .wrapping_mul(BITS_PER_BYTE as usize)
                                                    .wrapping_sub(1 as usize)
                                                    .wrapping_sub(
                                                        ::core::mem::size_of::<errseq_t>()
                                                            .wrapping_mul(8 as usize)
                                                            .wrapping_sub(1 as usize),
                                                    ),
                                            )
                                            << __s;
                                        let mut __newx: ulong = (__new as ulong) << __s;
                                        let mut __oldx: ulong = (__old as ulong) << __s;
                                        let mut __retx: ulong = 0;
                                        let mut __rc: ulong = 0;
                                        asm!(
                                            "0:\tlr.w {0}, 0({2})\n", "\tand  {1}, {0}, {5:}\n",
                                            "\tbne  {1}, {3:}, 1f\n", "\tand  {1}, {0}, {6:}\n",
                                            "\tor   {1}, {1}, {4:}\n", "\tsc.w.rl {1}, {1}, 0({2})\n",
                                            "\tbnez {1}, 0b\n", "1:\n", "\n", out(reg) __retx, out(reg)
                                            __rc, in (reg) & raw mut * __ptr32b, inlateout(reg) __oldx
                                            as ::core::ffi::c_long => _, inlateout(reg) __newx => _,
                                            inlateout(reg) __mask => _, inlateout(reg) ! __mask => _,
                                            options(preserves_flags)
                                        );
                                        __ret = ((__retx & __mask) >> __s) as errseq_t as errseq_t;
                                    }
                                };
                            }
                            2 => {
                                '_c2rust_label_3: {
                                    if true
                                        && true
                                        && true
                                        && riscv_has_extension_unlikely(
                                            RISCV_ISA_EXT_ZABHA as ::core::ffi::c_ulong,
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                        && riscv_has_extension_unlikely(
                                            RISCV_ISA_EXT_ZACAS as ::core::ffi::c_ulong,
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        __ret = __old as errseq_t;
                                        asm!(
                                            "\tamocas.h.aqrl {0}, {2:}, 0({1})\n", "\n", inout(reg)
                                            __ret, in (reg) & raw mut * __ptr, inlateout(reg) __new =>
                                            _, options(preserves_flags)
                                        );
                                    } else {
                                        let mut __ptr32b: *mut u32_0 =
                                            ::core::ptr::with_exposed_provenance_mut::<u32_0>(
                                                (__ptr.expose_provenance() as ulong
                                                    & !(0x3 as ::core::ffi::c_int) as ulong)
                                                    as usize,
                                            );
                                        let mut __s: ulong =
                                            (__ptr.expose_provenance() as ulong
                                                & (0x4 as ulong).wrapping_sub(
                                                    ::core::mem::size_of::<errseq_t>() as ulong,
                                                ))
                                            .wrapping_mul(BITS_PER_BYTE as ulong);
                                        let mut __mask: ulong = (::core::mem::size_of::<
                                            C2Rust_Unnamed_1,
                                        >(
                                        )
                                            as ::core::ffi::c_int
                                            as ::core::ffi::c_ulong)
                                            .wrapping_add(
                                                ((1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                    << (8 as usize)
                                                        .wrapping_mul(::core::mem::size_of::<
                                                            ::core::ffi::c_ulong,
                                                        >(
                                                        ))
                                                        .wrapping_sub(1 as usize)
                                                        .wrapping_sub(
                                                            ((-1 as ::core::ffi::c_int
                                                                as ::core::ffi::c_ulong)
                                                                < 1 as ::core::ffi::c_int
                                                                    as ::core::ffi::c_ulong)
                                                                as ::core::ffi::c_int
                                                                as usize,
                                                        ))
                                                .wrapping_sub(1 as ::core::ffi::c_ulong)
                                                .wrapping_add(
                                                    (1 as ::core::ffi::c_int
                                                        as ::core::ffi::c_ulong)
                                                        << (8 as usize)
                                                            .wrapping_mul(::core::mem::size_of::<
                                                                ::core::ffi::c_ulong,
                                                            >(
                                                            ))
                                                            .wrapping_sub(1 as usize)
                                                            .wrapping_sub(
                                                                ((-1 as ::core::ffi::c_int
                                                                    as ::core::ffi::c_ulong)
                                                                    < 1 as ::core::ffi::c_int
                                                                        as ::core::ffi::c_ulong)
                                                                    as ::core::ffi::c_int
                                                                    as usize,
                                                            ),
                                                ) << 0 as ::core::ffi::c_int
                                                    & ((1 as ::core::ffi::c_int
                                                        as ::core::ffi::c_ulong)
                                                        << (8 as usize)
                                                            .wrapping_mul(::core::mem::size_of::<
                                                                ::core::ffi::c_ulong,
                                                            >(
                                                            ))
                                                            .wrapping_sub(1 as usize)
                                                            .wrapping_sub(
                                                                ((-1 as ::core::ffi::c_int
                                                                    as ::core::ffi::c_ulong)
                                                                    < 1 as ::core::ffi::c_int
                                                                        as ::core::ffi::c_ulong)
                                                                    as ::core::ffi::c_int
                                                                    as usize,
                                                            ))
                                                    .wrapping_sub(1 as ::core::ffi::c_ulong)
                                                    .wrapping_add(
                                                        (1 as ::core::ffi::c_int
                                                            as ::core::ffi::c_ulong)
                                                            << (8 as usize)
                                                                .wrapping_mul(
                                                                    ::core::mem::size_of::<
                                                                        ::core::ffi::c_ulong,
                                                                    >(
                                                                    ),
                                                                )
                                                                .wrapping_sub(1 as usize)
                                                                .wrapping_sub(
                                                                    ((-1 as ::core::ffi::c_int
                                                                        as ::core::ffi::c_ulong)
                                                                        < 1 as ::core::ffi::c_int
                                                                            as ::core::ffi::c_ulong)
                                                                        as ::core::ffi::c_int
                                                                        as usize,
                                                                ),
                                                    ) >> ::core::mem::size_of::<
                                                        ::core::ffi::c_ulong,
                                                    >(
                                                    )
                                                    .wrapping_mul(BITS_PER_BYTE as usize)
                                                    .wrapping_sub(1 as usize)
                                                    .wrapping_sub(
                                                        ::core::mem::size_of::<errseq_t>()
                                                            .wrapping_mul(8 as usize)
                                                            .wrapping_sub(1 as usize),
                                                    ),
                                            )
                                            << __s;
                                        let mut __newx: ulong = (__new as ulong) << __s;
                                        let mut __oldx: ulong = (__old as ulong) << __s;
                                        let mut __retx: ulong = 0;
                                        let mut __rc: ulong = 0;
                                        asm!(
                                            "0:\tlr.w {0}, 0({2})\n", "\tand  {1}, {0}, {5:}\n",
                                            "\tbne  {1}, {3:}, 1f\n", "\tand  {1}, {0}, {6:}\n",
                                            "\tor   {1}, {1}, {4:}\n", "\tsc.w.rl {1}, {1}, 0({2})\n",
                                            "\tbnez {1}, 0b\n", "1:\n", "\n", out(reg) __retx, out(reg)
                                            __rc, in (reg) & raw mut * __ptr32b, inlateout(reg) __oldx
                                            as ::core::ffi::c_long => _, inlateout(reg) __newx => _,
                                            inlateout(reg) __mask => _, inlateout(reg) ! __mask => _,
                                            options(preserves_flags)
                                        );
                                        __ret = ((__retx & __mask) >> __s) as errseq_t as errseq_t;
                                    }
                                };
                            }
                            4 => {
                                '_c2rust_label_4: {
                                    if true
                                        && true
                                        && riscv_has_extension_unlikely(
                                            RISCV_ISA_EXT_ZACAS as ::core::ffi::c_ulong,
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        __ret = __old as errseq_t;
                                        asm!(
                                            "\tamocas.w.aqrl {0}, {2:}, 0({1})\n", "\n", inout(reg)
                                            __ret, in (reg) & raw mut * __ptr, inlateout(reg) __new =>
                                            _, options(preserves_flags)
                                        );
                                    } else {
                                        let mut __rc: ::core::ffi::c_uint = 0;
                                        asm!(
                                            "0:\tlr.w {0}, 0({2})\n", "\tbne  {0}, {3:}, 1f\n",
                                            "\tsc.w.rl {1}, {4:}, 0({2})\n", "\tbnez {1}, 0b\n", "1:\n",
                                            "\n", out(reg) __ret, out(reg) __rc, in (reg) & raw mut *
                                            __ptr, inlateout(reg) __old as ::core::ffi::c_long as
                                            ::core::ffi::c_int as ::core::ffi::c_long => _,
                                            inlateout(reg) __new => _, options(preserves_flags)
                                        );
                                    }
                                };
                            }
                            8 => {
                                '_c2rust_label_5: {
                                    if true
                                        && true
                                        && riscv_has_extension_unlikely(
                                            RISCV_ISA_EXT_ZACAS as ::core::ffi::c_ulong,
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        __ret = __old as errseq_t;
                                        asm!(
                                            "\tamocas.d.aqrl {0}, {2:}, 0({1})\n", "\n", inout(reg)
                                            __ret, in (reg) & raw mut * __ptr, inlateout(reg) __new =>
                                            _, options(preserves_flags)
                                        );
                                    } else {
                                        let mut __rc: ::core::ffi::c_uint = 0;
                                        asm!(
                                            "0:\tlr.d {0}, 0({2})\n", "\tbne  {0}, {3:}, 1f\n",
                                            "\tsc.d.rl {1}, {4:}, 0({2})\n", "\tbnez {1}, 0b\n", "1:\n",
                                            "\n", out(reg) __ret, out(reg) __rc, in (reg) & raw mut *
                                            __ptr, inlateout(reg) __old => _, inlateout(reg) __new => _,
                                            options(preserves_flags)
                                        );
                                    }
                                };
                            }
                            _ => {
                                extern "C" {
                                    #[link_name = "__compiletime_assert_29"]
                                    fn __compiletime_assert_29_0() -> !;
                                }
                                if true {
                                    __compiletime_assert_29_0();
                                }
                            }
                        }
                        __ret
                    })
                }) as errseq_t;
                if (cur == old || cur == new) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                    break;
                }
                old = cur;
            }
        }
        return cur;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_errseq_set: ; .asciz \"\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad errseq_set ; .previous"
);
#[no_mangle]
pub unsafe extern "C" fn errseq_sample(mut eseq: *mut errseq_t) -> errseq_t {
    unsafe {
        let mut old: errseq_t = ({
            extern "C" {
                #[link_name = "__compiletime_assert_31"]
                fn __compiletime_assert_31_0() -> !;
            }
            if !(::core::mem::size_of::<errseq_t>()
                == ::core::mem::size_of::<::core::ffi::c_char>()
                || ::core::mem::size_of::<errseq_t>()
                    == ::core::mem::size_of::<::core::ffi::c_short>()
                || ::core::mem::size_of::<errseq_t>()
                    == ::core::mem::size_of::<::core::ffi::c_int>()
                || ::core::mem::size_of::<errseq_t>()
                    == ::core::mem::size_of::<::core::ffi::c_long>()
                || ::core::mem::size_of::<errseq_t>()
                    == ::core::mem::size_of::<::core::ffi::c_longlong>())
            {
                __compiletime_assert_31_0();
            }
            ::core::ptr::read_volatile::<errseq_t>((eseq as *const errseq_t))
        });
        if old
            & ((1 as ::core::ffi::c_int)
                << (if false {
                    (if (4095 as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int
                    } else {
                        63 as ::core::ffi::c_int
                            - (4095 as ::core::ffi::c_ulonglong).leading_zeros()
                                as ::core::ffi::c_int
                    })
                } else {
                    (if ::core::mem::size_of::<::core::ffi::c_int>() <= 4 as usize {
                        __ilog2_u32(4095 as u32_0)
                    } else {
                        __ilog2_u64(4095 as u64_0)
                    })
                }) + 1 as ::core::ffi::c_int) as errseq_t
            == 0
        {
            old = 0 as errseq_t;
        }
        return old;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_errseq_sample: ; .asciz \"\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad errseq_sample ; .previous"
);
#[no_mangle]
pub unsafe extern "C" fn errseq_check(
    mut eseq: *mut errseq_t,
    mut since: errseq_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut cur: errseq_t = ({
            extern "C" {
                #[link_name = "__compiletime_assert_33"]
                fn __compiletime_assert_33_0() -> !;
            }
            if !(::core::mem::size_of::<errseq_t>()
                == ::core::mem::size_of::<::core::ffi::c_char>()
                || ::core::mem::size_of::<errseq_t>()
                    == ::core::mem::size_of::<::core::ffi::c_short>()
                || ::core::mem::size_of::<errseq_t>()
                    == ::core::mem::size_of::<::core::ffi::c_int>()
                || ::core::mem::size_of::<errseq_t>()
                    == ::core::mem::size_of::<::core::ffi::c_long>()
                || ::core::mem::size_of::<errseq_t>()
                    == ::core::mem::size_of::<::core::ffi::c_longlong>())
            {
                __compiletime_assert_33_0();
            }
            ::core::ptr::read_volatile::<errseq_t>((eseq as *const errseq_t))
        });
        if (cur == since) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            return 0 as ::core::ffi::c_int;
        }
        return (cur
            & (((1 as ::core::ffi::c_int)
                << (if false {
                    (if (4095 as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int
                    } else {
                        63 as ::core::ffi::c_int
                            - (4095 as ::core::ffi::c_ulonglong).leading_zeros()
                                as ::core::ffi::c_int
                    })
                } else {
                    (if ::core::mem::size_of::<::core::ffi::c_int>() <= 4 as usize {
                        __ilog2_u32(4095 as u32_0)
                    } else {
                        __ilog2_u64(4095 as u64_0)
                    })
                }) + 1 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as errseq_t)
            .wrapping_neg() as ::core::ffi::c_int;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_errseq_check: ; .asciz \"\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad errseq_check ; .previous"
);
#[no_mangle]
pub unsafe extern "C" fn errseq_check_and_advance(
    mut eseq: *mut errseq_t,
    mut since: *mut errseq_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut err: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut old: errseq_t = 0;
        let mut new: errseq_t = 0;
        old = ({
            extern "C" {
                #[link_name = "__compiletime_assert_35"]
                fn __compiletime_assert_35_0() -> !;
            }
            if !(::core::mem::size_of::<errseq_t>()
                == ::core::mem::size_of::<::core::ffi::c_char>()
                || ::core::mem::size_of::<errseq_t>()
                    == ::core::mem::size_of::<::core::ffi::c_short>()
                || ::core::mem::size_of::<errseq_t>()
                    == ::core::mem::size_of::<::core::ffi::c_int>()
                || ::core::mem::size_of::<errseq_t>()
                    == ::core::mem::size_of::<::core::ffi::c_long>()
                || ::core::mem::size_of::<errseq_t>()
                    == ::core::mem::size_of::<::core::ffi::c_longlong>())
            {
                __compiletime_assert_35_0();
            }
            ::core::ptr::read_volatile::<errseq_t>((eseq as *const errseq_t))
        }) as errseq_t;
        if old != *since {
            new = old
                | ((1 as ::core::ffi::c_int)
                    << (if false {
                        (if (4095 as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int
                        } else {
                            63 as ::core::ffi::c_int
                                - (4095 as ::core::ffi::c_ulonglong).leading_zeros()
                                    as ::core::ffi::c_int
                        })
                    } else {
                        (if ::core::mem::size_of::<::core::ffi::c_int>() <= 4 as usize {
                            __ilog2_u32(4095 as u32_0)
                        } else {
                            __ilog2_u64(4095 as u64_0)
                        })
                    }) + 1 as ::core::ffi::c_int) as errseq_t;
            if new != old {
                ({
                    let mut __ai_ptr: *mut errseq_t = eseq as *mut errseq_t;
                    instrument_atomic_read_write(
                        __ai_ptr as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<errseq_t>() as size_t,
                    );
                    ({
                        let mut __ptr: *mut errseq_t = __ai_ptr as *mut errseq_t;
                        let mut __old: errseq_t = old as errseq_t;
                        let mut __new: errseq_t = new as errseq_t;
                        let mut __ret: errseq_t = 0;
                        match ::core::mem::size_of::<errseq_t>() {
                            1 => {
                                '_c2rust_label_2: {
                                    if true
                                        && true
                                        && true
                                        && riscv_has_extension_unlikely(
                                            RISCV_ISA_EXT_ZABHA as ::core::ffi::c_ulong,
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                        && riscv_has_extension_unlikely(
                                            RISCV_ISA_EXT_ZACAS as ::core::ffi::c_ulong,
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        __ret = __old as errseq_t;
                                        asm!(
                                            "\tamocas.b.aqrl {0}, {2:}, 0({1})\n", "\n", inout(reg)
                                            __ret, in (reg) & raw mut * __ptr, inlateout(reg) __new =>
                                            _, options(preserves_flags)
                                        );
                                    } else {
                                        let mut __ptr32b: *mut u32_0 =
                                            ::core::ptr::with_exposed_provenance_mut::<u32_0>(
                                                (__ptr.expose_provenance() as ulong
                                                    & !(0x3 as ::core::ffi::c_int) as ulong)
                                                    as usize,
                                            );
                                        let mut __s: ulong =
                                            (__ptr.expose_provenance() as ulong
                                                & (0x4 as ulong).wrapping_sub(
                                                    ::core::mem::size_of::<errseq_t>() as ulong,
                                                ))
                                            .wrapping_mul(BITS_PER_BYTE as ulong);
                                        let mut __mask: ulong = (::core::mem::size_of::<
                                            C2Rust_Unnamed_4,
                                        >(
                                        )
                                            as ::core::ffi::c_int
                                            as ::core::ffi::c_ulong)
                                            .wrapping_add(
                                                ((1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                    << (8 as usize)
                                                        .wrapping_mul(::core::mem::size_of::<
                                                            ::core::ffi::c_ulong,
                                                        >(
                                                        ))
                                                        .wrapping_sub(1 as usize)
                                                        .wrapping_sub(
                                                            ((-1 as ::core::ffi::c_int
                                                                as ::core::ffi::c_ulong)
                                                                < 1 as ::core::ffi::c_int
                                                                    as ::core::ffi::c_ulong)
                                                                as ::core::ffi::c_int
                                                                as usize,
                                                        ))
                                                .wrapping_sub(1 as ::core::ffi::c_ulong)
                                                .wrapping_add(
                                                    (1 as ::core::ffi::c_int
                                                        as ::core::ffi::c_ulong)
                                                        << (8 as usize)
                                                            .wrapping_mul(::core::mem::size_of::<
                                                                ::core::ffi::c_ulong,
                                                            >(
                                                            ))
                                                            .wrapping_sub(1 as usize)
                                                            .wrapping_sub(
                                                                ((-1 as ::core::ffi::c_int
                                                                    as ::core::ffi::c_ulong)
                                                                    < 1 as ::core::ffi::c_int
                                                                        as ::core::ffi::c_ulong)
                                                                    as ::core::ffi::c_int
                                                                    as usize,
                                                            ),
                                                ) << 0 as ::core::ffi::c_int
                                                    & ((1 as ::core::ffi::c_int
                                                        as ::core::ffi::c_ulong)
                                                        << (8 as usize)
                                                            .wrapping_mul(::core::mem::size_of::<
                                                                ::core::ffi::c_ulong,
                                                            >(
                                                            ))
                                                            .wrapping_sub(1 as usize)
                                                            .wrapping_sub(
                                                                ((-1 as ::core::ffi::c_int
                                                                    as ::core::ffi::c_ulong)
                                                                    < 1 as ::core::ffi::c_int
                                                                        as ::core::ffi::c_ulong)
                                                                    as ::core::ffi::c_int
                                                                    as usize,
                                                            ))
                                                    .wrapping_sub(1 as ::core::ffi::c_ulong)
                                                    .wrapping_add(
                                                        (1 as ::core::ffi::c_int
                                                            as ::core::ffi::c_ulong)
                                                            << (8 as usize)
                                                                .wrapping_mul(
                                                                    ::core::mem::size_of::<
                                                                        ::core::ffi::c_ulong,
                                                                    >(
                                                                    ),
                                                                )
                                                                .wrapping_sub(1 as usize)
                                                                .wrapping_sub(
                                                                    ((-1 as ::core::ffi::c_int
                                                                        as ::core::ffi::c_ulong)
                                                                        < 1 as ::core::ffi::c_int
                                                                            as ::core::ffi::c_ulong)
                                                                        as ::core::ffi::c_int
                                                                        as usize,
                                                                ),
                                                    ) >> ::core::mem::size_of::<
                                                        ::core::ffi::c_ulong,
                                                    >(
                                                    )
                                                    .wrapping_mul(BITS_PER_BYTE as usize)
                                                    .wrapping_sub(1 as usize)
                                                    .wrapping_sub(
                                                        ::core::mem::size_of::<errseq_t>()
                                                            .wrapping_mul(8 as usize)
                                                            .wrapping_sub(1 as usize),
                                                    ),
                                            )
                                            << __s;
                                        let mut __newx: ulong = (__new as ulong) << __s;
                                        let mut __oldx: ulong = (__old as ulong) << __s;
                                        let mut __retx: ulong = 0;
                                        let mut __rc: ulong = 0;
                                        asm!(
                                            "0:\tlr.w {0}, 0({2})\n", "\tand  {1}, {0}, {5:}\n",
                                            "\tbne  {1}, {3:}, 1f\n", "\tand  {1}, {0}, {6:}\n",
                                            "\tor   {1}, {1}, {4:}\n", "\tsc.w.rl {1}, {1}, 0({2})\n",
                                            "\tbnez {1}, 0b\n", "1:\n", "\n", out(reg) __retx, out(reg)
                                            __rc, in (reg) & raw mut * __ptr32b, inlateout(reg) __oldx
                                            as ::core::ffi::c_long => _, inlateout(reg) __newx => _,
                                            inlateout(reg) __mask => _, inlateout(reg) ! __mask => _,
                                            options(preserves_flags)
                                        );
                                        __ret = ((__retx & __mask) >> __s) as errseq_t as errseq_t;
                                    }
                                };
                            }
                            2 => {
                                '_c2rust_label_3: {
                                    if true
                                        && true
                                        && true
                                        && riscv_has_extension_unlikely(
                                            RISCV_ISA_EXT_ZABHA as ::core::ffi::c_ulong,
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                        && riscv_has_extension_unlikely(
                                            RISCV_ISA_EXT_ZACAS as ::core::ffi::c_ulong,
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        __ret = __old as errseq_t;
                                        asm!(
                                            "\tamocas.h.aqrl {0}, {2:}, 0({1})\n", "\n", inout(reg)
                                            __ret, in (reg) & raw mut * __ptr, inlateout(reg) __new =>
                                            _, options(preserves_flags)
                                        );
                                    } else {
                                        let mut __ptr32b: *mut u32_0 =
                                            ::core::ptr::with_exposed_provenance_mut::<u32_0>(
                                                (__ptr.expose_provenance() as ulong
                                                    & !(0x3 as ::core::ffi::c_int) as ulong)
                                                    as usize,
                                            );
                                        let mut __s: ulong =
                                            (__ptr.expose_provenance() as ulong
                                                & (0x4 as ulong).wrapping_sub(
                                                    ::core::mem::size_of::<errseq_t>() as ulong,
                                                ))
                                            .wrapping_mul(BITS_PER_BYTE as ulong);
                                        let mut __mask: ulong = (::core::mem::size_of::<
                                            C2Rust_Unnamed_3,
                                        >(
                                        )
                                            as ::core::ffi::c_int
                                            as ::core::ffi::c_ulong)
                                            .wrapping_add(
                                                ((1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                    << (8 as usize)
                                                        .wrapping_mul(::core::mem::size_of::<
                                                            ::core::ffi::c_ulong,
                                                        >(
                                                        ))
                                                        .wrapping_sub(1 as usize)
                                                        .wrapping_sub(
                                                            ((-1 as ::core::ffi::c_int
                                                                as ::core::ffi::c_ulong)
                                                                < 1 as ::core::ffi::c_int
                                                                    as ::core::ffi::c_ulong)
                                                                as ::core::ffi::c_int
                                                                as usize,
                                                        ))
                                                .wrapping_sub(1 as ::core::ffi::c_ulong)
                                                .wrapping_add(
                                                    (1 as ::core::ffi::c_int
                                                        as ::core::ffi::c_ulong)
                                                        << (8 as usize)
                                                            .wrapping_mul(::core::mem::size_of::<
                                                                ::core::ffi::c_ulong,
                                                            >(
                                                            ))
                                                            .wrapping_sub(1 as usize)
                                                            .wrapping_sub(
                                                                ((-1 as ::core::ffi::c_int
                                                                    as ::core::ffi::c_ulong)
                                                                    < 1 as ::core::ffi::c_int
                                                                        as ::core::ffi::c_ulong)
                                                                    as ::core::ffi::c_int
                                                                    as usize,
                                                            ),
                                                ) << 0 as ::core::ffi::c_int
                                                    & ((1 as ::core::ffi::c_int
                                                        as ::core::ffi::c_ulong)
                                                        << (8 as usize)
                                                            .wrapping_mul(::core::mem::size_of::<
                                                                ::core::ffi::c_ulong,
                                                            >(
                                                            ))
                                                            .wrapping_sub(1 as usize)
                                                            .wrapping_sub(
                                                                ((-1 as ::core::ffi::c_int
                                                                    as ::core::ffi::c_ulong)
                                                                    < 1 as ::core::ffi::c_int
                                                                        as ::core::ffi::c_ulong)
                                                                    as ::core::ffi::c_int
                                                                    as usize,
                                                            ))
                                                    .wrapping_sub(1 as ::core::ffi::c_ulong)
                                                    .wrapping_add(
                                                        (1 as ::core::ffi::c_int
                                                            as ::core::ffi::c_ulong)
                                                            << (8 as usize)
                                                                .wrapping_mul(
                                                                    ::core::mem::size_of::<
                                                                        ::core::ffi::c_ulong,
                                                                    >(
                                                                    ),
                                                                )
                                                                .wrapping_sub(1 as usize)
                                                                .wrapping_sub(
                                                                    ((-1 as ::core::ffi::c_int
                                                                        as ::core::ffi::c_ulong)
                                                                        < 1 as ::core::ffi::c_int
                                                                            as ::core::ffi::c_ulong)
                                                                        as ::core::ffi::c_int
                                                                        as usize,
                                                                ),
                                                    ) >> ::core::mem::size_of::<
                                                        ::core::ffi::c_ulong,
                                                    >(
                                                    )
                                                    .wrapping_mul(BITS_PER_BYTE as usize)
                                                    .wrapping_sub(1 as usize)
                                                    .wrapping_sub(
                                                        ::core::mem::size_of::<errseq_t>()
                                                            .wrapping_mul(8 as usize)
                                                            .wrapping_sub(1 as usize),
                                                    ),
                                            )
                                            << __s;
                                        let mut __newx: ulong = (__new as ulong) << __s;
                                        let mut __oldx: ulong = (__old as ulong) << __s;
                                        let mut __retx: ulong = 0;
                                        let mut __rc: ulong = 0;
                                        asm!(
                                            "0:\tlr.w {0}, 0({2})\n", "\tand  {1}, {0}, {5:}\n",
                                            "\tbne  {1}, {3:}, 1f\n", "\tand  {1}, {0}, {6:}\n",
                                            "\tor   {1}, {1}, {4:}\n", "\tsc.w.rl {1}, {1}, 0({2})\n",
                                            "\tbnez {1}, 0b\n", "1:\n", "\n", out(reg) __retx, out(reg)
                                            __rc, in (reg) & raw mut * __ptr32b, inlateout(reg) __oldx
                                            as ::core::ffi::c_long => _, inlateout(reg) __newx => _,
                                            inlateout(reg) __mask => _, inlateout(reg) ! __mask => _,
                                            options(preserves_flags)
                                        );
                                        __ret = ((__retx & __mask) >> __s) as errseq_t as errseq_t;
                                    }
                                };
                            }
                            4 => {
                                '_c2rust_label_4: {
                                    if true
                                        && true
                                        && riscv_has_extension_unlikely(
                                            RISCV_ISA_EXT_ZACAS as ::core::ffi::c_ulong,
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        __ret = __old as errseq_t;
                                        asm!(
                                            "\tamocas.w.aqrl {0}, {2:}, 0({1})\n", "\n", inout(reg)
                                            __ret, in (reg) & raw mut * __ptr, inlateout(reg) __new =>
                                            _, options(preserves_flags)
                                        );
                                    } else {
                                        let mut __rc: ::core::ffi::c_uint = 0;
                                        asm!(
                                            "0:\tlr.w {0}, 0({2})\n", "\tbne  {0}, {3:}, 1f\n",
                                            "\tsc.w.rl {1}, {4:}, 0({2})\n", "\tbnez {1}, 0b\n", "1:\n",
                                            "\n", out(reg) __ret, out(reg) __rc, in (reg) & raw mut *
                                            __ptr, inlateout(reg) __old as ::core::ffi::c_long as
                                            ::core::ffi::c_int as ::core::ffi::c_long => _,
                                            inlateout(reg) __new => _, options(preserves_flags)
                                        );
                                    }
                                };
                            }
                            8 => {
                                '_c2rust_label_5: {
                                    if true
                                        && true
                                        && riscv_has_extension_unlikely(
                                            RISCV_ISA_EXT_ZACAS as ::core::ffi::c_ulong,
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        __ret = __old as errseq_t;
                                        asm!(
                                            "\tamocas.d.aqrl {0}, {2:}, 0({1})\n", "\n", inout(reg)
                                            __ret, in (reg) & raw mut * __ptr, inlateout(reg) __new =>
                                            _, options(preserves_flags)
                                        );
                                    } else {
                                        let mut __rc: ::core::ffi::c_uint = 0;
                                        asm!(
                                            "0:\tlr.d {0}, 0({2})\n", "\tbne  {0}, {3:}, 1f\n",
                                            "\tsc.d.rl {1}, {4:}, 0({2})\n", "\tbnez {1}, 0b\n", "1:\n",
                                            "\n", out(reg) __ret, out(reg) __rc, in (reg) & raw mut *
                                            __ptr, inlateout(reg) __old => _, inlateout(reg) __new => _,
                                            options(preserves_flags)
                                        );
                                    }
                                };
                            }
                            _ => {
                                extern "C" {
                                    #[link_name = "__compiletime_assert_36"]
                                    fn __compiletime_assert_36_0() -> !;
                                }
                                if true {
                                    __compiletime_assert_36_0();
                                }
                            }
                        }
                    });
                });
            }
            *since = new;
            err = (new
                & (((1 as ::core::ffi::c_int)
                    << (if false {
                        (if (4095 as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int
                        } else {
                            63 as ::core::ffi::c_int
                                - (4095 as ::core::ffi::c_ulonglong).leading_zeros()
                                    as ::core::ffi::c_int
                        })
                    } else {
                        (if ::core::mem::size_of::<::core::ffi::c_int>() <= 4 as usize {
                            __ilog2_u32(4095 as u32_0)
                        } else {
                            __ilog2_u64(4095 as u64_0)
                        })
                    }) + 1 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as errseq_t)
                .wrapping_neg() as ::core::ffi::c_int;
        }
        return err;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_errseq_check_and_advance: ; .asciz \"\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad errseq_check_and_advance ; .previous"
);
