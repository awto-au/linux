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
use ::core::arch::global_asm;
extern "C" {
    static _ctype: [::core::ffi::c_uchar; 0];
    fn hex_to_bin(ch: ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    fn get_random_bytes(buf: *mut ::core::ffi::c_void, len: size_t);
}
pub type __u8 = ::core::ffi::c_uchar;
pub type u8_0 = __u8;
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
pub struct guid_t {
    pub b: [__u8; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuid_t {
    pub b: [__u8; 16],
}
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const _D: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const _X: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const UUID_STRING_LEN: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
#[no_mangle]
pub static mut guid_null: guid_t = guid_t { b: [0; 16] };
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_guid_null: ; .asciz \"\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad guid_null ; .previous"
);
#[no_mangle]
pub static mut uuid_null: uuid_t = uuid_t { b: [0; 16] };
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_uuid_null: ; .asciz \"\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad uuid_null ; .previous"
);
#[no_mangle]
pub static mut guid_index: [u8_0; 16] = [
    3 as u8_0, 2 as u8_0, 1 as u8_0, 0 as u8_0, 5 as u8_0, 4 as u8_0, 7 as u8_0, 6 as u8_0,
    8 as u8_0, 9 as u8_0, 10 as u8_0, 11 as u8_0, 12 as u8_0, 13 as u8_0, 14 as u8_0, 15 as u8_0,
];
#[no_mangle]
pub static mut uuid_index: [u8_0; 16] = [
    0 as u8_0, 1 as u8_0, 2 as u8_0, 3 as u8_0, 4 as u8_0, 5 as u8_0, 6 as u8_0, 7 as u8_0,
    8 as u8_0, 9 as u8_0, 10 as u8_0, 11 as u8_0, 12 as u8_0, 13 as u8_0, 14 as u8_0, 15 as u8_0,
];
#[no_mangle]
pub unsafe extern "C" fn generate_random_uuid(mut uuid: *mut ::core::ffi::c_uchar) {
    unsafe {
        get_random_bytes(uuid as *mut ::core::ffi::c_void, 16 as size_t);
        *uuid.offset(6 as ::core::ffi::c_int as isize) =
            (*uuid.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int
                | 0x40 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        *uuid.offset(8 as ::core::ffi::c_int as isize) =
            (*uuid.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x3f as ::core::ffi::c_int
                | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_generate_random_uuid: ; .asciz \"\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad generate_random_uuid ; .previous"
);
#[no_mangle]
pub unsafe extern "C" fn generate_random_guid(mut guid: *mut ::core::ffi::c_uchar) {
    unsafe {
        get_random_bytes(guid as *mut ::core::ffi::c_void, 16 as size_t);
        *guid.offset(7 as ::core::ffi::c_int as isize) =
            (*guid.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int
                | 0x40 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        *guid.offset(8 as ::core::ffi::c_int as isize) =
            (*guid.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x3f as ::core::ffi::c_int
                | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_generate_random_guid: ; .asciz \"\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad generate_random_guid ; .previous"
);
unsafe extern "C" fn __uuid_gen_common(mut b: *mut __u8) {
    unsafe {
        get_random_bytes(b as *mut ::core::ffi::c_void, 16 as size_t);
        *b.offset(8 as ::core::ffi::c_int as isize) =
            (*b.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x3f as ::core::ffi::c_int
                | 0x80 as ::core::ffi::c_int) as __u8;
    }
}
#[no_mangle]
pub unsafe extern "C" fn guid_gen(mut lu: *mut guid_t) {
    unsafe {
        __uuid_gen_common(&raw mut (*lu).b as *mut __u8);
        (*lu).b[7 as ::core::ffi::c_int as usize] =
            ((*lu).b[7 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int
                | 0x40 as ::core::ffi::c_int) as __u8;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_guid_gen: ; .asciz \"GPL\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad guid_gen ; .previous"
);
#[no_mangle]
pub unsafe extern "C" fn uuid_gen(mut bu: *mut uuid_t) {
    unsafe {
        __uuid_gen_common(&raw mut (*bu).b as *mut __u8);
        (*bu).b[6 as ::core::ffi::c_int as usize] =
            ((*bu).b[6 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int
                | 0x40 as ::core::ffi::c_int) as __u8;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_uuid_gen: ; .asciz \"GPL\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad uuid_gen ; .previous"
);
#[no_mangle]
pub unsafe extern "C" fn uuid_is_valid(mut uuid: *const ::core::ffi::c_char) -> bool_0 {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        i = 0 as ::core::ffi::c_uint;
        while i < UUID_STRING_LEN as ::core::ffi::c_uint {
            if i == 8 as ::core::ffi::c_uint
                || i == 13 as ::core::ffi::c_uint
                || i == 18 as ::core::ffi::c_uint
                || i == 23 as ::core::ffi::c_uint
            {
                if *uuid.offset(i as isize) as ::core::ffi::c_int != '-' as ::core::ffi::c_int {
                    return r#false as ::core::ffi::c_int != 0;
                }
            } else if !(*(&raw const _ctype as *const ::core::ffi::c_uchar).offset(
                *uuid.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize,
            ) as ::core::ffi::c_int
                & (_D | _X)
                != 0 as ::core::ffi::c_int)
            {
                return r#false as ::core::ffi::c_int != 0;
            }
            i = i.wrapping_add(1);
        }
        return r#true as ::core::ffi::c_int != 0;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_uuid_is_valid: ; .asciz \"\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad uuid_is_valid ; .previous"
);
unsafe extern "C" fn __uuid_parse(
    mut uuid: *const ::core::ffi::c_char,
    mut b: *mut __u8,
    mut ei: *const u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        static mut si: [u8_0; 16] = [
            0 as u8_0, 2 as u8_0, 4 as u8_0, 6 as u8_0, 9 as u8_0, 11 as u8_0, 14 as u8_0,
            16 as u8_0, 19 as u8_0, 21 as u8_0, 24 as u8_0, 26 as u8_0, 28 as u8_0, 30 as u8_0,
            32 as u8_0, 34 as u8_0,
        ];
        let mut i: ::core::ffi::c_uint = 0;
        if !uuid_is_valid(uuid) {
            return -EINVAL;
        }
        i = 0 as ::core::ffi::c_uint;
        while i < 16 as ::core::ffi::c_uint {
            let mut hi: ::core::ffi::c_int =
                hex_to_bin(*uuid.offset(
                    (si[i as usize] as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_uchar);
            let mut lo: ::core::ffi::c_int =
                hex_to_bin(*uuid.offset(
                    (si[i as usize] as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_uchar);
            *b.offset(*ei.offset(i as isize) as isize) =
                (hi << 4 as ::core::ffi::c_int | lo) as __u8;
            i = i.wrapping_add(1);
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn guid_parse(
    mut uuid: *const ::core::ffi::c_char,
    mut u: *mut guid_t,
) -> ::core::ffi::c_int {
    unsafe {
        return __uuid_parse(
            uuid,
            &raw mut (*u).b as *mut __u8,
            &raw const guid_index as *const u8_0,
        );
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_guid_parse: ; .asciz \"\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad guid_parse ; .previous"
);
#[no_mangle]
pub unsafe extern "C" fn uuid_parse(
    mut uuid: *const ::core::ffi::c_char,
    mut u: *mut uuid_t,
) -> ::core::ffi::c_int {
    unsafe {
        return __uuid_parse(
            uuid,
            &raw mut (*u).b as *mut __u8,
            &raw const uuid_index as *const u8_0,
        );
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_uuid_parse: ; .asciz \"\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad uuid_parse ; .previous"
);
