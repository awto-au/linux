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
extern "C" {
    fn console_verbose();
    fn __riscv_isa_extension_available(
        isa_bitmap: *const ::core::ffi::c_ulong,
        bit: ::core::ffi::c_uint,
    ) -> bool_0;
    fn __riscv_has_extension_unlikely(
        vendor: ::core::ffi::c_ulong,
        ext: ::core::ffi::c_ulong,
    ) -> bool_0;
}
pub type __u32 = ::core::ffi::c_uint;
pub type u32_0 = __u32;
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
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {}
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
pub const RISCV_ISA_EXT_ZABHA: ::core::ffi::c_int = 90 as ::core::ffi::c_int;
pub const STANDARD_EXT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
unsafe extern "C" fn __debug_locks_off() -> ::core::ffi::c_int {
    unsafe {
        return ({
            let mut __ai_ptr: *mut ::core::ffi::c_int = &raw mut debug_locks;
            instrument_atomic_read_write(
                __ai_ptr as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            );
            ({
                let mut __ptr: *mut ::core::ffi::c_int = __ai_ptr as *mut ::core::ffi::c_int;
                let mut __new: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut __ret: ::core::ffi::c_int = 0;
                match ::core::mem::size_of::<::core::ffi::c_int>() {
                    1 => {
                        '_c2rust_label_1: {
                            if true
                                && riscv_has_extension_unlikely(
                                    RISCV_ISA_EXT_ZABHA as ::core::ffi::c_ulong,
                                ) as ::core::ffi::c_int
                                    != 0
                            {
                                asm!(
                                    "\tamoswap.b.aqrl {0}, {2:}, 0({1})\n", "\n", out(reg) __ret,
                                    in (reg) & raw mut * __ptr, inlateout(reg) __new => _,
                                    options(preserves_flags)
                                );
                            } else {
                                let mut __ptr32b: *mut u32_0 =
                                    ::core::ptr::with_exposed_provenance_mut::<u32_0>(
                                        (__ptr.expose_provenance() as ulong
                                            & !(0x3 as ::core::ffi::c_int) as ulong)
                                            as usize,
                                    );
                                let mut __s: ulong = (__ptr.expose_provenance() as ulong
                                    & (0x4 as ulong).wrapping_sub(::core::mem::size_of::<
                                        ::core::ffi::c_int,
                                    >(
                                    )
                                        as ulong))
                                .wrapping_mul(BITS_PER_BYTE as ulong);
                                let mut __mask: ulong = (::core::mem::size_of::<C2Rust_Unnamed_2>()
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
                                            (1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
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
                                            & ((1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
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
                                                (1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
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
                                            ) >> ::core::mem::size_of::<::core::ffi::c_ulong>()
                                                .wrapping_mul(BITS_PER_BYTE as usize)
                                                .wrapping_sub(1 as usize)
                                                .wrapping_sub(
                                                    ::core::mem::size_of::<::core::ffi::c_int>()
                                                        .wrapping_mul(8 as usize)
                                                        .wrapping_sub(1 as usize),
                                                ),
                                    )
                                    << __s;
                                let mut __newx: ulong = (__new as ulong) << __s;
                                let mut __retx: ulong = 0;
                                let mut __rc: ulong = 0;
                                asm!(
                                    "886 :\n", ".option push\n", ".option norvc\n",
                                    ".option norelax\n", ".rept  1\n", "nop\n", ".endr\n", "\n",
                                    ".option pop\n", "887 :\n", ".if 1 == 1\n",
                                    ".pushsection .alternative, \"a\"\n",
                                    ".4byte\t((886b) - .) \n", ".4byte\t((888f) - .) \n",
                                    ".2byte\t0\n", ".2byte 889f - 888f\n", ".4byte\t99\n",
                                    ".popsection\n", ".subsection 1\n", "888 :\n",
                                    ".option push\n", ".option norvc\n", ".option norelax\n",
                                    ".insn\ts 19, 6, x3, (0) & 0xfe0({5})\n", "\n",
                                    ".option pop\n", "889 :\n",
                                    ".org\t. - (887b - 886b) + (889b - 888b)\n",
                                    ".org\t. - (889b - 888b) + (887b - 886b)\n", ".previous\n",
                                    ".endif\n", "0:\tlr.w {0}, 0({2})\n",
                                    "\tand  {1}, {0}, {4:}\n", "\tor   {1}, {1}, {3:}\n",
                                    "\tsc.w.rl {1}, {1}, 0({2})\n", "\tbnez {1}, 0b\n", "\n",
                                    out(reg) __retx, out(reg) __rc, in (reg) & raw mut *
                                    __ptr32b, inlateout(reg) __newx => _, inlateout(reg) !
                                    __mask => _, inlateout(reg) __ptr32b => _,
                                    options(preserves_flags)
                                );
                                __ret = ((__retx & __mask) >> __s) as ::core::ffi::c_int
                                    as ::core::ffi::c_int;
                            }
                        };
                    }
                    2 => {
                        '_c2rust_label_2: {
                            if true
                                && riscv_has_extension_unlikely(
                                    RISCV_ISA_EXT_ZABHA as ::core::ffi::c_ulong,
                                ) as ::core::ffi::c_int
                                    != 0
                            {
                                asm!(
                                    "\tamoswap.h.aqrl {0}, {2:}, 0({1})\n", "\n", out(reg) __ret,
                                    in (reg) & raw mut * __ptr, inlateout(reg) __new => _,
                                    options(preserves_flags)
                                );
                            } else {
                                let mut __ptr32b: *mut u32_0 =
                                    ::core::ptr::with_exposed_provenance_mut::<u32_0>(
                                        (__ptr.expose_provenance() as ulong
                                            & !(0x3 as ::core::ffi::c_int) as ulong)
                                            as usize,
                                    );
                                let mut __s: ulong = (__ptr.expose_provenance() as ulong
                                    & (0x4 as ulong).wrapping_sub(::core::mem::size_of::<
                                        ::core::ffi::c_int,
                                    >(
                                    )
                                        as ulong))
                                .wrapping_mul(BITS_PER_BYTE as ulong);
                                let mut __mask: ulong = (::core::mem::size_of::<C2Rust_Unnamed_1>()
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
                                            (1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
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
                                            & ((1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
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
                                                (1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
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
                                            ) >> ::core::mem::size_of::<::core::ffi::c_ulong>()
                                                .wrapping_mul(BITS_PER_BYTE as usize)
                                                .wrapping_sub(1 as usize)
                                                .wrapping_sub(
                                                    ::core::mem::size_of::<::core::ffi::c_int>()
                                                        .wrapping_mul(8 as usize)
                                                        .wrapping_sub(1 as usize),
                                                ),
                                    )
                                    << __s;
                                let mut __newx: ulong = (__new as ulong) << __s;
                                let mut __retx: ulong = 0;
                                let mut __rc: ulong = 0;
                                asm!(
                                    "886 :\n", ".option push\n", ".option norvc\n",
                                    ".option norelax\n", ".rept  1\n", "nop\n", ".endr\n", "\n",
                                    ".option pop\n", "887 :\n", ".if 1 == 1\n",
                                    ".pushsection .alternative, \"a\"\n",
                                    ".4byte\t((886b) - .) \n", ".4byte\t((888f) - .) \n",
                                    ".2byte\t0\n", ".2byte 889f - 888f\n", ".4byte\t99\n",
                                    ".popsection\n", ".subsection 1\n", "888 :\n",
                                    ".option push\n", ".option norvc\n", ".option norelax\n",
                                    ".insn\ts 19, 6, x3, (0) & 0xfe0({5})\n", "\n",
                                    ".option pop\n", "889 :\n",
                                    ".org\t. - (887b - 886b) + (889b - 888b)\n",
                                    ".org\t. - (889b - 888b) + (887b - 886b)\n", ".previous\n",
                                    ".endif\n", "0:\tlr.w {0}, 0({2})\n",
                                    "\tand  {1}, {0}, {4:}\n", "\tor   {1}, {1}, {3:}\n",
                                    "\tsc.w.rl {1}, {1}, 0({2})\n", "\tbnez {1}, 0b\n", "\n",
                                    out(reg) __retx, out(reg) __rc, in (reg) & raw mut *
                                    __ptr32b, inlateout(reg) __newx => _, inlateout(reg) !
                                    __mask => _, inlateout(reg) __ptr32b => _,
                                    options(preserves_flags)
                                );
                                __ret = ((__retx & __mask) >> __s) as ::core::ffi::c_int
                                    as ::core::ffi::c_int;
                            }
                        };
                    }
                    4 => {
                        '_c2rust_label_3: {
                            asm!(
                                "\tamoswap.w.aqrl {0}, {2}, 0({1})\n", "\n", lateout(reg)
                                __ret, in (reg) & raw mut * __ptr, inlateout(reg) __new =>
                                _, options(preserves_flags)
                            );
                        };
                    }
                    8 => {
                        '_c2rust_label_4: {
                            asm!(
                                "\tamoswap.d.aqrl {0}, {2}, 0({1})\n", "\n", lateout(reg)
                                __ret, in (reg) & raw mut * __ptr, inlateout(reg) __new =>
                                _, options(preserves_flags)
                            );
                        };
                    }
                    _ => {
                        extern "C" {
                            #[link_name = "__compiletime_assert_75"]
                            fn __compiletime_assert_75_0() -> !;
                        }
                        if true {
                            __compiletime_assert_75_0();
                        }
                    }
                }
                __ret
            })
        });
    }
}
#[no_mangle]
pub static mut debug_locks: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_debug_locks: ; .asciz \"GPL\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad debug_locks ; .previous"
);
#[no_mangle]
pub static mut debug_locks_silent: ::core::ffi::c_int = 0;
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_debug_locks_silent: ; .asciz \"GPL\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad debug_locks_silent ; .previous"
);
#[no_mangle]
pub unsafe extern "C" fn debug_locks_off() -> ::core::ffi::c_int {
    unsafe {
        if debug_locks != 0 && __debug_locks_off() != 0 {
            if debug_locks_silent == 0 {
                console_verbose();
                return 1 as ::core::ffi::c_int;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_debug_locks_off: ; .asciz \"GPL\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad debug_locks_off ; .previous"
);
