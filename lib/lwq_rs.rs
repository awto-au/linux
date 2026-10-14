#![allow(
    clippy::missing_safety_doc,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use ::core::arch::{asm, global_asm};
macro_rules! opaque_marker {
    ($name:ident) => {
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct $name {
            _private: [u8; 0],
        }
    };
}
opaque_marker!(task_struct);

extern "C" {
    fn __riscv_isa_extension_available(
        isa_bitmap: *const ::core::ffi::c_ulong,
        bit: ::core::ffi::c_uint,
    ) -> bool_0;
    fn __riscv_has_extension_unlikely(
        vendor: ::core::ffi::c_ulong,
        ext: ::core::ffi::c_ulong,
    ) -> bool_0;
    fn preempt_schedule();
    fn llist_reverse_order(head: *mut llist_node) -> *mut llist_node;
}
pub type __u8 = ::core::ffi::c_uchar;
pub type __u16 = ::core::ffi::c_ushort;
pub type __u32 = ::core::ffi::c_uint;
pub type __s64 = ::core::ffi::c_longlong;
pub type __u64 = ::core::ffi::c_ulonglong;
pub type u8_0 = __u8;
pub type u16_0 = __u16;
pub type u32_0 = __u32;
pub type s64 = __s64;
pub type u64_0 = __u64;
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const r#true: C2Rust_Unnamed = 1;
pub const r#false: C2Rust_Unnamed = 0;
pub type __kernel_long_t = ::core::ffi::c_long;
pub type __kernel_ulong_t = ::core::ffi::c_ulong;
pub type __kernel_pid_t = ::core::ffi::c_int;
pub type __kernel_uid32_t = ::core::ffi::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_time64_t = ::core::ffi::c_longlong;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = ::core::ffi::c_int;
pub type __kernel_clockid_t = ::core::ffi::c_int;
pub type pid_t = __kernel_pid_t;
pub type clockid_t = __kernel_clockid_t;
pub type bool_0 = bool;
pub type uintptr_t = usize;
pub type size_t = __kernel_size_t;
pub type ulong = ::core::ffi::c_ulong;
pub type ktime_t = s64;
opaque_marker!(sched_dl_entity);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread_info {
    pub flags: ::core::ffi::c_ulong,
    pub preempt_count: ::core::ffi::c_int,
    pub kernel_sp: ::core::ffi::c_long,
    pub user_sp: ::core::ffi::c_long,
    pub cpu: ::core::ffi::c_int,
    pub syscall_work: ::core::ffi::c_ulong,
    pub a0: ::core::ffi::c_ulong,
    pub a1: ::core::ffi::c_ulong,
    pub a2: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct raw_spinlock {
    pub raw_lock: arch_spinlock_t,
}
pub type raw_spinlock_t = raw_spinlock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arch_spinlock_t {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spinlock {
    pub c2rust_unnamed: C2Rust_Unnamed_12,
}
pub type spinlock_t = spinlock;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_12 {
    pub rlock: raw_spinlock,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct llist_node {
    pub next: *mut llist_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct llist_head {
    pub first: *mut llist_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lwq {
    pub lock: spinlock_t,
    pub ready: *mut llist_node,
    pub new: llist_head,
}
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
    unsafe {
}
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
    return __riscv_has_extension_unlikely(STANDARD_EXT as ::core::ffi::c_ulong, ext);
}
}
#[inline(always)]
unsafe extern "C" fn instrument_atomic_check_alignment(
    mut v: *const ::core::ffi::c_void,
    mut size: size_t,
) {
    unsafe {
}
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
#[no_mangle]
pub static mut riscv_current_is_tp: *mut task_struct = ::core::ptr::null_mut::<task_struct>();
#[inline(always)]
unsafe extern "C" fn get_current() -> *mut task_struct {
    unsafe {
    return riscv_current_is_tp;
}
}
#[inline(always)]
unsafe extern "C" fn generic_test_bit(
    mut nr: ::core::ffi::c_ulong,
    mut addr: *const ::core::ffi::c_ulong,
) -> bool_0 {
    unsafe {
    return 1 as ::core::ffi::c_ulong
        & ::core::ptr::read_volatile::<::core::ffi::c_ulong>(
            addr.offset(nr.wrapping_div(BITS_PER_LONG as ::core::ffi::c_ulong) as isize),
        ) >> (nr & (BITS_PER_LONG - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
        != 0;
}
}
#[inline(always)]
unsafe extern "C" fn const_test_bit(
    mut nr: ::core::ffi::c_ulong,
    mut addr: *const ::core::ffi::c_ulong,
) -> bool_0 {
    unsafe {
    let mut p: *const ::core::ffi::c_ulong = (addr as *const ::core::ffi::c_ulong)
        .offset(nr.wrapping_div(BITS_PER_LONG as ::core::ffi::c_ulong) as isize);
    let mut mask: ::core::ffi::c_ulong =
        (1 as ::core::ffi::c_ulong) << nr.wrapping_rem(BITS_PER_LONG as ::core::ffi::c_ulong);
    let mut val: ::core::ffi::c_ulong = *p;
    return val & mask != 0;
}
}
pub const TIF_NEED_RESCHED: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn tif_test_bit(mut bit: ::core::ffi::c_int) -> bool_0 {
    unsafe {
    return if false
        && false
        && (&raw mut (*((get_current as unsafe extern "C" fn() -> *mut task_struct)()
            as *mut thread_info))
            .flags)
            .expose_provenance() as uintptr_t
            != NULL.expose_provenance() as uintptr_t
        && false
    {
        const_test_bit(
            bit as ::core::ffi::c_ulong,
            &raw mut (*((get_current as unsafe extern "C" fn() -> *mut task_struct)()
                as *mut thread_info))
                .flags,
        ) as ::core::ffi::c_int
    } else {
        generic_test_bit(
            bit as ::core::ffi::c_ulong,
            &raw mut (*((get_current as unsafe extern "C" fn() -> *mut task_struct)()
                as *mut thread_info))
                .flags,
        ) as ::core::ffi::c_int
    } != 0;
}
}
#[inline(always)]
unsafe extern "C" fn tif_need_resched() -> bool_0 {
    unsafe {
    return tif_test_bit(TIF_NEED_RESCHED);
}
}
#[inline(always)]
unsafe extern "C" fn preempt_count_ptr() -> *mut ::core::ffi::c_int {
    unsafe {
    return &raw mut (*((get_current as unsafe extern "C" fn() -> *mut task_struct)()
        as *mut thread_info))
        .preempt_count as *mut ::core::ffi::c_int;
}
}
#[inline(always)]
unsafe extern "C" fn __preempt_count_add(mut val: ::core::ffi::c_int) {
    unsafe {
    let c2rust_lvalue_ptr = &raw mut *preempt_count_ptr();
    ::core::ptr::write_volatile(
        c2rust_lvalue_ptr,
        ::core::ptr::read_volatile::<::core::ffi::c_int>(c2rust_lvalue_ptr) + val,
    );
}
}
#[inline(always)]
unsafe extern "C" fn __preempt_count_dec_and_test() -> bool_0 {
    unsafe {
    let c2rust_lvalue_ptr = &raw mut *preempt_count_ptr();
    ::core::ptr::write_volatile(
        c2rust_lvalue_ptr,
        ::core::ptr::read_volatile::<::core::ffi::c_int>(c2rust_lvalue_ptr) - 1,
    );
    return ::core::ptr::read_volatile::<::core::ffi::c_int>(c2rust_lvalue_ptr) == 0
        && tif_need_resched() as ::core::ffi::c_int != 0;
}
}
#[inline]
unsafe extern "C" fn llist_empty(mut head: *const llist_head) -> bool_0 {
    unsafe {
    return ({
        extern "C" {
            #[link_name = "__compiletime_assert_32"]
            fn __compiletime_assert_32_0() -> !;
        }
        if !(::core::mem::size_of::<*mut llist_node>()
            == ::core::mem::size_of::<::core::ffi::c_char>()
            || ::core::mem::size_of::<*mut llist_node>()
                == ::core::mem::size_of::<::core::ffi::c_short>()
            || ::core::mem::size_of::<*mut llist_node>()
                == ::core::mem::size_of::<::core::ffi::c_int>()
            || ::core::mem::size_of::<*mut llist_node>()
                == ::core::mem::size_of::<::core::ffi::c_long>()
            || ::core::mem::size_of::<*mut llist_node>()
                == ::core::mem::size_of::<::core::ffi::c_longlong>())
        {
            __compiletime_assert_32_0();
        }
        ::core::ptr::read_volatile::<*mut llist_node>(
            (&raw const (*head).first as *const *mut llist_node),
        )
    })
    .is_null();
}
}
#[inline]
unsafe extern "C" fn llist_next(mut node: *mut llist_node) -> *mut llist_node {
    unsafe {
    return ({
        extern "C" {
            #[link_name = "__compiletime_assert_33"]
            fn __compiletime_assert_33_0() -> !;
        }
        if !(::core::mem::size_of::<*mut llist_node>()
            == ::core::mem::size_of::<::core::ffi::c_char>()
            || ::core::mem::size_of::<*mut llist_node>()
                == ::core::mem::size_of::<::core::ffi::c_short>()
            || ::core::mem::size_of::<*mut llist_node>()
                == ::core::mem::size_of::<::core::ffi::c_int>()
            || ::core::mem::size_of::<*mut llist_node>()
                == ::core::mem::size_of::<::core::ffi::c_long>()
            || ::core::mem::size_of::<*mut llist_node>()
                == ::core::mem::size_of::<::core::ffi::c_longlong>())
        {
            __compiletime_assert_33_0();
        }
        ::core::ptr::read_volatile::<*mut llist_node>(
            (&raw mut (*node).next as *const *mut llist_node),
        )
    });
}
}
#[inline]
unsafe extern "C" fn llist_del_all(mut head: *mut llist_head) -> *mut llist_node {
    unsafe {
    return ({
        let mut __ai_ptr: *mut *mut llist_node = &raw mut (*head).first;
        instrument_atomic_read_write(
            __ai_ptr as *const ::core::ffi::c_void,
            ::core::mem::size_of::<*mut llist_node>() as size_t,
        );
        ({
            let mut __ptr: *mut *mut llist_node = __ai_ptr as *mut *mut llist_node;
            let mut __new: *mut llist_node = ::core::ptr::null_mut::<llist_node>();
            let mut __ret: *mut llist_node = ::core::ptr::null_mut::<llist_node>();
            match ::core::mem::size_of::<*mut llist_node>() {
                1 => {
                    '_c2rust_label_1: {
                            let mut __ptr32b: *mut u32_0 =
                                ::core::ptr::with_exposed_provenance_mut::<u32_0>(
                                    (__ptr.expose_provenance() as ulong
                                        & !(0x3 as ::core::ffi::c_int) as ulong)
                                        as usize,
                                );
                            let mut __s: ulong = (__ptr.expose_provenance() as ulong
                                & (0x4 as ulong).wrapping_sub(
                                    ::core::mem::size_of::<*mut llist_node>() as ulong,
                                ))
                            .wrapping_mul(BITS_PER_BYTE as ulong);
                            let mut __mask: ulong = (::core::mem::size_of::<C2Rust_Unnamed_21>()
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
                                                ((-1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
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
                                                ::core::mem::size_of::<*mut llist_node>()
                                                    .wrapping_mul(8 as usize)
                                                    .wrapping_sub(1 as usize),
                                            ),
                                )
                                << __s;
                            let mut __newx: ulong = (__new.expose_provenance() as ulong) << __s;
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
                                ".endif\n", "0:\tlr.w {0}, [{2}]\n",
                                "\tand  {1}, {0}, {4:}\n", "\tor   {1}, {1}, {3:}\n",
                                "\tsc.w.rl {1}, {1}, [{2}]\n", "\tbnez {1}, 0b\n", "\n",
                                out(reg) __retx, out(reg) __rc, in (reg) & raw mut *
                                __ptr32b, inlateout(reg) __newx => _, inlateout(reg) !
                                __mask => _, inlateout(reg) __ptr32b => _,
                                options(preserves_flags)
                            );
                            __ret = ::core::ptr::with_exposed_provenance_mut::<llist_node>(
                                ((__retx & __mask) >> __s) as usize,
                            ) as *mut llist_node;
                    };
                }
                2 => {
                    '_c2rust_label_2: {
                            let mut __ptr32b: *mut u32_0 =
                                ::core::ptr::with_exposed_provenance_mut::<u32_0>(
                                    (__ptr.expose_provenance() as ulong
                                        & !(0x3 as ::core::ffi::c_int) as ulong)
                                        as usize,
                                );
                            let mut __s: ulong = (__ptr.expose_provenance() as ulong
                                & (0x4 as ulong).wrapping_sub(
                                    ::core::mem::size_of::<*mut llist_node>() as ulong,
                                ))
                            .wrapping_mul(BITS_PER_BYTE as ulong);
                            let mut __mask: ulong = (::core::mem::size_of::<C2Rust_Unnamed_20>()
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
                                                ((-1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
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
                                                ::core::mem::size_of::<*mut llist_node>()
                                                    .wrapping_mul(8 as usize)
                                                    .wrapping_sub(1 as usize),
                                            ),
                                )
                                << __s;
                            let mut __newx: ulong = (__new.expose_provenance() as ulong) << __s;
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
                                ".endif\n", "0:\tlr.w {0}, [{2}]\n",
                                "\tand  {1}, {0}, {4:}\n", "\tor   {1}, {1}, {3:}\n",
                                "\tsc.w.rl {1}, {1}, [{2}]\n", "\tbnez {1}, 0b\n", "\n",
                                out(reg) __retx, out(reg) __rc, in (reg) & raw mut *
                                __ptr32b, inlateout(reg) __newx => _, inlateout(reg) !
                                __mask => _, inlateout(reg) __ptr32b => _,
                                options(preserves_flags)
                            );
                            __ret = ::core::ptr::with_exposed_provenance_mut::<llist_node>(
                                ((__retx & __mask) >> __s) as usize,
                            ) as *mut llist_node;
                    };
                }
                4 => {
                    '_c2rust_label_3: {
                        asm!(
                            "\tamoswap.w.aqrl {0}, {2}, 0({1})\n", "\n", lateout(reg)
                            __ret, in (reg) & raw mut * __ptr, inlateout(reg) __new => _,
                            options(preserves_flags)
                        );
                    };
                }
                8 => {
                    '_c2rust_label_4: {
                        asm!(
                            "\tamoswap.d.aqrl {0}, {2}, 0({1})\n", "\n", lateout(reg)
                            __ret, in (reg) & raw mut * __ptr, inlateout(reg) __new => _,
                            options(preserves_flags)
                        );
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
            __ret
        })
    });
}
}
#[inline(always)]
unsafe extern "C" fn spin_lock(mut lock: *mut spinlock_t) {
    unsafe {
    __preempt_count_add(1 as ::core::ffi::c_int);
    asm!("\n", options(preserves_flags));
    &raw mut (*lock).c2rust_unnamed.rlock;
}
}
#[inline(always)]
unsafe extern "C" fn spin_unlock(mut lock: *mut spinlock_t) {
    unsafe {
    asm!("\n", options(preserves_flags));
    if __preempt_count_dec_and_test() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        preempt_schedule();
    }
    &raw mut (*lock).c2rust_unnamed.rlock;
}
}
#[inline]
unsafe extern "C" fn lwq_empty(mut q: *mut lwq) -> bool_0 {
    unsafe {
    return ({
        let mut ___p1: *mut llist_node = ({
            extern "C" {
                #[link_name = "__compiletime_assert_84"]
                fn __compiletime_assert_84_0() -> !;
            }
            if !(::core::mem::size_of::<*mut llist_node>()
                == ::core::mem::size_of::<::core::ffi::c_char>()
                || ::core::mem::size_of::<*mut llist_node>()
                    == ::core::mem::size_of::<::core::ffi::c_short>()
                || ::core::mem::size_of::<*mut llist_node>()
                    == ::core::mem::size_of::<::core::ffi::c_int>()
                || ::core::mem::size_of::<*mut llist_node>()
                    == ::core::mem::size_of::<::core::ffi::c_long>()
                || ::core::mem::size_of::<*mut llist_node>()
                    == ::core::mem::size_of::<::core::ffi::c_longlong>())
            {
                __compiletime_assert_84_0();
            }
            ::core::ptr::read_volatile::<*mut llist_node>(
                (&raw mut (*q).ready as *const *mut llist_node),
            )
        });
        asm!("\n", options(preserves_flags));
        ___p1
    })
    .is_null()
        && llist_empty(&raw mut (*q).new) as ::core::ffi::c_int != 0;
}
}
#[no_mangle]
pub unsafe extern "C" fn __lwq_dequeue(mut q: *mut lwq) -> *mut llist_node {
    unsafe {
        let mut this: *mut llist_node = ::core::ptr::null_mut::<llist_node>();
        if lwq_empty(q) {
            return ::core::ptr::null_mut::<llist_node>();
        }
        spin_lock(&raw mut (*q).lock);
        this = (*q).ready;
        if this.is_null() && !llist_empty(&raw mut (*q).new) {
            asm!("\n", options(preserves_flags));
            extern "C" {
                #[link_name = "__compiletime_assert_87"]
                fn __compiletime_assert_87_0() -> !;
            }
            if !(::core::mem::size_of::<*mut llist_node>()
                == ::core::mem::size_of::<::core::ffi::c_char>()
                || ::core::mem::size_of::<*mut llist_node>()
                    == ::core::mem::size_of::<::core::ffi::c_short>()
                || ::core::mem::size_of::<*mut llist_node>()
                    == ::core::mem::size_of::<::core::ffi::c_int>()
                || ::core::mem::size_of::<*mut llist_node>()
                    == ::core::mem::size_of::<::core::ffi::c_long>()
                || ::core::mem::size_of::<*mut llist_node>()
                    == ::core::mem::size_of::<::core::ffi::c_longlong>())
            {
                __compiletime_assert_87_0();
            }
            ::core::ptr::write_volatile(
                (&raw mut (*q).ready as *mut *mut llist_node),
                ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    1 as ::core::ffi::c_int as usize,
                ) as *mut llist_node,
            );
            this = llist_reverse_order(llist_del_all(&raw mut (*q).new));
            if this.is_null() {
                (*q).ready = ::core::ptr::null_mut::<llist_node>();
            }
        }
        if !this.is_null() {
            (*q).ready = llist_next(this);
        }
        spin_unlock(&raw mut (*q).lock);
        return this;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol___lwq_dequeue: ; .asciz \"GPL\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad __lwq_dequeue ; .previous"
);
#[no_mangle]
pub unsafe extern "C" fn lwq_dequeue_all(mut q: *mut lwq) -> *mut llist_node {
    unsafe {
        let mut r: *mut llist_node = ::core::ptr::null_mut::<llist_node>();
        let mut t: *mut llist_node = ::core::ptr::null_mut::<llist_node>();
        let mut ep: *mut *mut llist_node = ::core::ptr::null_mut::<*mut llist_node>();
        if lwq_empty(q) {
            return ::core::ptr::null_mut::<llist_node>();
        }
        spin_lock(&raw mut (*q).lock);
        r = (*q).ready;
        (*q).ready = ::core::ptr::null_mut::<llist_node>();
        t = llist_del_all(&raw mut (*q).new);
        spin_unlock(&raw mut (*q).lock);
        ep = &raw mut r;
        while !(*ep).is_null() {
            ep = &raw mut (**ep).next;
        }
        *ep = llist_reverse_order(t);
        return r;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_lwq_dequeue_all: ; .asciz \"GPL\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad lwq_dequeue_all ; .previous"
);
