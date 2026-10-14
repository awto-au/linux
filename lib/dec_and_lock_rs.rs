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
    fn preempt_schedule();
}
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const r#true: C2Rust_Unnamed = 1;
pub const r#false: C2Rust_Unnamed = 0;
pub type __kernel_ulong_t = ::core::ffi::c_ulong;
pub type __kernel_size_t = __kernel_ulong_t;
pub type bool_0 = bool;
pub type uintptr_t = usize;
pub type size_t = __kernel_size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct atomic_t {
    pub counter: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arch_spinlock_t {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct raw_spinlock {
    pub raw_lock: arch_spinlock_t,
}
pub type raw_spinlock_t = raw_spinlock;
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
pub struct spinlock {
    pub c2rust_unnamed: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub rlock: raw_spinlock,
}
pub type spinlock_t = spinlock;
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
#[inline(always)]
unsafe extern "C" fn get_current() -> *mut task_struct {
    unsafe {
        return unsafe {
            let c2rust_result: *mut task_struct;
            asm!(
                "mv {0}, tp", out(reg) c2rust_result, options(nomem, nostack,
                preserves_flags)
            );
            c2rust_result
        };
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
unsafe extern "C" fn arch_local_irq_save() -> ::core::ffi::c_ulong {
    unsafe {
        return ({
            let mut __v: ::core::ffi::c_ulong = 0x2 as ::core::ffi::c_ulong;
            asm!(
                "csrrc {0}, 0x100, {1}\n", lateout(reg) __v, inlateout(reg) __v => _,
                options(preserves_flags)
            );
            __v
        });
    }
}
#[inline]
unsafe extern "C" fn arch_local_irq_restore(mut flags: ::core::ffi::c_ulong) {
    unsafe {
        '_c2rust_label: {
            let mut __v: ::core::ffi::c_ulong = flags & 0x2 as ::core::ffi::c_ulong;
            asm!("csrs 0x100, {0}\n", inlateout(reg) __v => _, options(preserves_flags));
        };
    }
}
#[inline(always)]
unsafe extern "C" fn arch_atomic_fetch_sub(
    mut i: ::core::ffi::c_int,
    mut v: *mut atomic_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ret: ::core::ffi::c_int = 0;
        asm!(
            "\tamoadd.w.aqrl  {1}, {2}, 0({0})\n", in (reg) & raw mut (* v).counter,
            lateout(reg) ret, inlateout(reg) - i => _, options(preserves_flags)
        );
        return ret;
    }
}
#[inline(always)]
unsafe extern "C" fn arch_atomic_sub_return(
    mut i: ::core::ffi::c_int,
    mut v: *mut atomic_t,
) -> ::core::ffi::c_int {
    unsafe {
        return arch_atomic_fetch_sub(i, v) + -i;
    }
}
#[inline(always)]
unsafe extern "C" fn arch_atomic_fetch_add_unless(
    mut v: *mut atomic_t,
    mut a: ::core::ffi::c_int,
    mut u: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut prev: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        '_c2rust_label: {
            asm!(
                "0:\tlr.w     {0},  0({2})\n", "\tbeq\t       {0},  {4}, 1f\n",
                "\tadd            {1}, {0}, {3}\n", "\tsc.w.rl  {1}, {1}, 0({2})\n",
                "\tbnez           {1}, 0b\n", "1:\n", "\n", out(reg) prev, out(reg) rc,
                in (reg) & raw mut (* v).counter, inlateout(reg) a => _, inlateout(reg) u
                => _, options(preserves_flags)
            );
        };
        return prev;
    }
}
#[inline(always)]
unsafe extern "C" fn raw_atomic_sub_return(
    mut i: ::core::ffi::c_int,
    mut v: *mut atomic_t,
) -> ::core::ffi::c_int {
    unsafe {
        return arch_atomic_sub_return(i, v);
    }
}
#[inline(always)]
unsafe extern "C" fn raw_atomic_dec_return(mut v: *mut atomic_t) -> ::core::ffi::c_int {
    unsafe {
        return raw_atomic_sub_return(1 as ::core::ffi::c_int, v);
    }
}
#[inline(always)]
unsafe extern "C" fn raw_atomic_dec_and_test(mut v: *mut atomic_t) -> bool_0 {
    unsafe {
        return raw_atomic_dec_return(v) == 0 as ::core::ffi::c_int;
    }
}
#[inline(always)]
unsafe extern "C" fn raw_atomic_fetch_add_unless(
    mut v: *mut atomic_t,
    mut a: ::core::ffi::c_int,
    mut u: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return arch_atomic_fetch_add_unless(v, a, u);
    }
}
#[inline(always)]
unsafe extern "C" fn raw_atomic_add_unless(
    mut v: *mut atomic_t,
    mut a: ::core::ffi::c_int,
    mut u: ::core::ffi::c_int,
) -> bool_0 {
    unsafe {
        return raw_atomic_fetch_add_unless(v, a, u) != u;
    }
}
#[inline(always)]
unsafe extern "C" fn atomic_dec_and_test(mut v: *mut atomic_t) -> bool_0 {
    unsafe {
        instrument_atomic_read_write(
            v as *const ::core::ffi::c_void,
            ::core::mem::size_of::<atomic_t>() as size_t,
        );
        return raw_atomic_dec_and_test(v);
    }
}
#[inline(always)]
unsafe extern "C" fn atomic_add_unless(
    mut v: *mut atomic_t,
    mut a: ::core::ffi::c_int,
    mut u: ::core::ffi::c_int,
) -> bool_0 {
    unsafe {
        instrument_atomic_read_write(
            v as *const ::core::ffi::c_void,
            ::core::mem::size_of::<atomic_t>() as size_t,
        );
        return raw_atomic_add_unless(v, a, u);
    }
}
#[inline(always)]
unsafe extern "C" fn spinlock_check(mut lock: *mut spinlock_t) -> *mut raw_spinlock_t {
    unsafe {
        return &raw mut (*lock).c2rust_unnamed.rlock;
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
#[inline(always)]
unsafe extern "C" fn spin_unlock_irqrestore(
    mut lock: *mut spinlock_t,
    mut flags: ::core::ffi::c_ulong,
) {
    unsafe {
        ({
            let mut __dummy: ::core::ffi::c_ulong = 0;
            let mut __dummy2: ::core::ffi::c_ulong = 0;
            &raw mut __dummy;
            &raw mut __dummy2;
        });
        ({
            let mut __dummy: ::core::ffi::c_ulong = 0;
            let mut __dummy2: ::core::ffi::c_ulong = 0;
            &raw mut __dummy;
            &raw mut __dummy2;
        });
        arch_local_irq_restore(flags);
        asm!("\n", options(preserves_flags));
        if __preempt_count_dec_and_test() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            preempt_schedule();
        }
        &raw mut (*lock).c2rust_unnamed.rlock;
    }
}
#[no_mangle]
pub unsafe extern "C" fn atomic_dec_and_lock(
    mut atomic: *mut atomic_t,
    mut lock: *mut spinlock_t,
) -> ::core::ffi::c_int {
    unsafe {
        if atomic_add_unless(atomic, -1 as ::core::ffi::c_int, 1 as ::core::ffi::c_int) {
            return 0 as ::core::ffi::c_int;
        }
        spin_lock(lock);
        if atomic_dec_and_test(atomic) {
            return 1 as ::core::ffi::c_int;
        }
        spin_unlock(lock);
        return 0 as ::core::ffi::c_int;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_atomic_dec_and_lock: ; .asciz \"\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad atomic_dec_and_lock ; .previous"
);
#[no_mangle]
pub unsafe extern "C" fn _atomic_dec_and_lock_irqsave(
    mut atomic: *mut atomic_t,
    mut lock: *mut spinlock_t,
    mut flags: *mut ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    unsafe {
        if atomic_add_unless(atomic, -1 as ::core::ffi::c_int, 1 as ::core::ffi::c_int) {
            return 0 as ::core::ffi::c_int;
        }
        ({
            let mut __dummy: ::core::ffi::c_ulong = 0;
            let mut __dummy2: ::core::ffi::c_ulong = 0;
            &raw mut __dummy;
            &raw mut __dummy2;
        });
        ({
            let mut __dummy: ::core::ffi::c_ulong = 0;
            let mut __dummy2: ::core::ffi::c_ulong = 0;
            &raw mut __dummy;
            &raw mut __dummy2;
        });
        *flags = arch_local_irq_save();
        __preempt_count_add(1 as ::core::ffi::c_int);
        asm!("\n", options(preserves_flags));
        spinlock_check(lock);
        if atomic_dec_and_test(atomic) {
            return 1 as ::core::ffi::c_int;
        }
        spin_unlock_irqrestore(lock, *flags);
        return 0 as ::core::ffi::c_int;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol__atomic_dec_and_lock_irqsave: ; .asciz \"\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad _atomic_dec_and_lock_irqsave ; .previous"
);
#[no_mangle]
pub unsafe extern "C" fn atomic_dec_and_raw_lock(
    mut atomic: *mut atomic_t,
    mut lock: *mut raw_spinlock_t,
) -> ::core::ffi::c_int {
    unsafe {
        if atomic_add_unless(atomic, -1 as ::core::ffi::c_int, 1 as ::core::ffi::c_int) {
            return 0 as ::core::ffi::c_int;
        }
        __preempt_count_add(1 as ::core::ffi::c_int);
        asm!("\n", options(preserves_flags));
        if atomic_dec_and_test(atomic) {
            return 1 as ::core::ffi::c_int;
        }
        asm!("\n", options(preserves_flags));
        if __preempt_count_dec_and_test() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            preempt_schedule();
        }
        return 0 as ::core::ffi::c_int;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_atomic_dec_and_raw_lock: ; .asciz \"\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad atomic_dec_and_raw_lock ; .previous"
);
#[no_mangle]
pub unsafe extern "C" fn _atomic_dec_and_raw_lock_irqsave(
    mut atomic: *mut atomic_t,
    mut lock: *mut raw_spinlock_t,
    mut flags: *mut ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    unsafe {
        if atomic_add_unless(atomic, -1 as ::core::ffi::c_int, 1 as ::core::ffi::c_int) {
            return 0 as ::core::ffi::c_int;
        }
        ({
            let mut __dummy: ::core::ffi::c_ulong = 0;
            let mut __dummy2: ::core::ffi::c_ulong = 0;
            &raw mut __dummy;
            &raw mut __dummy2;
        });
        ({
            let mut __dummy: ::core::ffi::c_ulong = 0;
            let mut __dummy2: ::core::ffi::c_ulong = 0;
            &raw mut __dummy;
            &raw mut __dummy2;
        });
        *flags = arch_local_irq_save();
        __preempt_count_add(1 as ::core::ffi::c_int);
        asm!("\n", options(preserves_flags));
        if atomic_dec_and_test(atomic) {
            return 1 as ::core::ffi::c_int;
        }
        ({
            let mut __dummy: ::core::ffi::c_ulong = 0;
            let mut __dummy2: ::core::ffi::c_ulong = 0;
            &raw mut __dummy;
            &raw mut __dummy2;
        });
        ({
            let mut __dummy: ::core::ffi::c_ulong = 0;
            let mut __dummy2: ::core::ffi::c_ulong = 0;
            &raw mut __dummy;
            &raw mut __dummy2;
        });
        arch_local_irq_restore(*flags);
        asm!("\n", options(preserves_flags));
        if __preempt_count_dec_and_test() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            preempt_schedule();
        }
        return 0 as ::core::ffi::c_int;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol__atomic_dec_and_raw_lock_irqsave: ; .asciz \"\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad _atomic_dec_and_raw_lock_irqsave ; .previous"
);
