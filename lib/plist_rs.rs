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
use ::core::arch::asm;
use ::kernel::warn_on;
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const r#true: C2Rust_Unnamed = 1;
pub const r#false: C2Rust_Unnamed = 0;
pub type bool_0 = bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plist_head {
    pub node_list: list_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plist_node {
    pub prio: ::core::ffi::c_int,
    pub prio_list: list_head,
    pub node_list: list_head,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn INIT_LIST_HEAD(mut list: *mut list_head) {
    unsafe {
        extern "C" {
            #[link_name = "__compiletime_assert_3"]
            fn __compiletime_assert_3_0() -> !;
        }
        if !(::core::mem::size_of::<*mut list_head>()
            == ::core::mem::size_of::<::core::ffi::c_char>()
            || ::core::mem::size_of::<*mut list_head>()
                == ::core::mem::size_of::<::core::ffi::c_short>()
            || ::core::mem::size_of::<*mut list_head>()
                == ::core::mem::size_of::<::core::ffi::c_int>()
            || ::core::mem::size_of::<*mut list_head>()
                == ::core::mem::size_of::<::core::ffi::c_long>()
            || ::core::mem::size_of::<*mut list_head>()
                == ::core::mem::size_of::<::core::ffi::c_longlong>())
        {
            __compiletime_assert_3_0();
        }
        ::core::ptr::write_volatile(
            (&raw mut (*list).next as *mut *mut list_head),
            list as *mut list_head,
        );
        extern "C" {
            #[link_name = "__compiletime_assert_4"]
            fn __compiletime_assert_4_0() -> !;
        }
        if !(::core::mem::size_of::<*mut list_head>()
            == ::core::mem::size_of::<::core::ffi::c_char>()
            || ::core::mem::size_of::<*mut list_head>()
                == ::core::mem::size_of::<::core::ffi::c_short>()
            || ::core::mem::size_of::<*mut list_head>()
                == ::core::mem::size_of::<::core::ffi::c_int>()
            || ::core::mem::size_of::<*mut list_head>()
                == ::core::mem::size_of::<::core::ffi::c_long>()
            || ::core::mem::size_of::<*mut list_head>()
                == ::core::mem::size_of::<::core::ffi::c_longlong>())
        {
            __compiletime_assert_4_0();
        }
        ::core::ptr::write_volatile(
            (&raw mut (*list).prev as *mut *mut list_head),
            list as *mut list_head,
        );
    }
}
#[inline]
unsafe extern "C" fn __list_add_valid(
    mut new: *mut list_head,
    mut prev: *mut list_head,
    mut next: *mut list_head,
) -> bool_0 {
    unsafe {
        return r#true as ::core::ffi::c_int != 0;
    }
}
#[inline]
unsafe extern "C" fn __list_del_entry_valid(mut entry: *mut list_head) -> bool_0 {
    unsafe {
        return r#true as ::core::ffi::c_int != 0;
    }
}
#[inline]
unsafe extern "C" fn __list_add(
    mut new: *mut list_head,
    mut prev: *mut list_head,
    mut next: *mut list_head,
) {
    unsafe {
        if !__list_add_valid(new, prev, next) {
            return;
        }
        (*next).prev = new;
        (*new).next = next;
        (*new).prev = prev;
        extern "C" {
            #[link_name = "__compiletime_assert_5"]
            fn __compiletime_assert_5_0() -> !;
        }
        if !(::core::mem::size_of::<*mut list_head>()
            == ::core::mem::size_of::<::core::ffi::c_char>()
            || ::core::mem::size_of::<*mut list_head>()
                == ::core::mem::size_of::<::core::ffi::c_short>()
            || ::core::mem::size_of::<*mut list_head>()
                == ::core::mem::size_of::<::core::ffi::c_int>()
            || ::core::mem::size_of::<*mut list_head>()
                == ::core::mem::size_of::<::core::ffi::c_long>()
            || ::core::mem::size_of::<*mut list_head>()
                == ::core::mem::size_of::<::core::ffi::c_longlong>())
        {
            __compiletime_assert_5_0();
        }
        ::core::ptr::write_volatile(
            (&raw mut (*prev).next as *mut *mut list_head),
            new as *mut list_head,
        );
    }
}
#[inline]
unsafe extern "C" fn list_add(mut new: *mut list_head, mut head: *mut list_head) {
    unsafe {
        __list_add(new, head, (*head).next);
    }
}
#[inline]
unsafe extern "C" fn list_add_tail(mut new: *mut list_head, mut head: *mut list_head) {
    unsafe {
        __list_add(new, (*head).prev, head);
    }
}
#[inline]
unsafe extern "C" fn __list_del(mut prev: *mut list_head, mut next: *mut list_head) {
    unsafe {
        (*next).prev = prev;
        extern "C" {
            #[link_name = "__compiletime_assert_7"]
            fn __compiletime_assert_7_0() -> !;
        }
        if !(::core::mem::size_of::<*mut list_head>()
            == ::core::mem::size_of::<::core::ffi::c_char>()
            || ::core::mem::size_of::<*mut list_head>()
                == ::core::mem::size_of::<::core::ffi::c_short>()
            || ::core::mem::size_of::<*mut list_head>()
                == ::core::mem::size_of::<::core::ffi::c_int>()
            || ::core::mem::size_of::<*mut list_head>()
                == ::core::mem::size_of::<::core::ffi::c_long>()
            || ::core::mem::size_of::<*mut list_head>()
                == ::core::mem::size_of::<::core::ffi::c_longlong>())
        {
            __compiletime_assert_7_0();
        }
        ::core::ptr::write_volatile(
            (&raw mut (*prev).next as *mut *mut list_head),
            next as *mut list_head,
        );
    }
}
#[inline]
unsafe extern "C" fn __list_del_entry(mut entry: *mut list_head) {
    unsafe {
        if !__list_del_entry_valid(entry) {
            return;
        }
        __list_del((*entry).prev, (*entry).next);
    }
}
#[inline]
unsafe extern "C" fn list_del_init(mut entry: *mut list_head) {
    unsafe {
        __list_del_entry(entry);
        INIT_LIST_HEAD(entry);
    }
}
#[inline]
unsafe extern "C" fn list_is_head(
    mut list: *const list_head,
    mut head: *const list_head,
) -> ::core::ffi::c_int {
    unsafe {
        return (list == head) as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn list_empty(mut head: *const list_head) -> ::core::ffi::c_int {
    unsafe {
        return (({
            extern "C" {
                #[link_name = "__compiletime_assert_8"]
                fn __compiletime_assert_8_0() -> !;
            }
            if !(::core::mem::size_of::<*mut list_head>()
                == ::core::mem::size_of::<::core::ffi::c_char>()
                || ::core::mem::size_of::<*mut list_head>()
                    == ::core::mem::size_of::<::core::ffi::c_short>()
                || ::core::mem::size_of::<*mut list_head>()
                    == ::core::mem::size_of::<::core::ffi::c_int>()
                || ::core::mem::size_of::<*mut list_head>()
                    == ::core::mem::size_of::<::core::ffi::c_long>()
                || ::core::mem::size_of::<*mut list_head>()
                    == ::core::mem::size_of::<::core::ffi::c_longlong>())
            {
                __compiletime_assert_8_0();
            }
            ::core::ptr::read_volatile::<*mut list_head>(
                (&raw const (*head).next as *const *mut list_head),
            )
        }) == head as *mut list_head) as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn plist_head_empty(mut head: *const plist_head) -> ::core::ffi::c_int {
    unsafe {
        return list_empty(&raw const (*head).node_list);
    }
}
#[inline]
unsafe extern "C" fn plist_node_empty(mut node: *const plist_node) -> ::core::ffi::c_int {
    unsafe {
        return list_empty(&raw const (*node).node_list);
    }
}
#[inline]
unsafe extern "C" fn plist_first(mut head: *const plist_head) -> *mut plist_node {
    unsafe {
        return ({
            let mut __mptr: *mut ::core::ffi::c_void =
                (*head).node_list.next as *mut ::core::ffi::c_void;
            __mptr.offset(-(24 as usize as isize)) as *mut plist_node
        });
    }
}
#[inline]
unsafe extern "C" fn plist_last(mut head: *const plist_head) -> *mut plist_node {
    unsafe {
        return ({
            let mut __mptr: *mut ::core::ffi::c_void =
                (*head).node_list.prev as *mut ::core::ffi::c_void;
            __mptr.offset(-(24 as usize as isize)) as *mut plist_node
        });
    }
}
#[no_mangle]
pub unsafe extern "C" fn plist_add(mut node: *mut plist_node, mut head: *mut plist_head) {
    unsafe {
        let mut first: *mut plist_node = ::core::ptr::null_mut::<plist_node>();
        let mut iter: *mut plist_node = ::core::ptr::null_mut::<plist_node>();
        let mut prev: *mut plist_node = ::core::ptr::null_mut::<plist_node>();
        let mut last: *mut plist_node = ::core::ptr::null_mut::<plist_node>();
        let mut reverse_iter: *mut plist_node = ::core::ptr::null_mut::<plist_node>();
        let mut node_next: *mut list_head = &raw mut (*head).node_list;
        kernel::warn_on!(plist_node_empty(node) == 0);
        kernel::warn_on!(list_empty(&raw mut (*node).prio_list) == 0);
        if plist_head_empty(head) == 0 {
            iter = plist_first(head);
            first = iter;
            reverse_iter = ({
                let mut __mptr: *mut ::core::ffi::c_void =
                    (*first).prio_list.prev as *mut ::core::ffi::c_void;
                __mptr.offset(-(8 as usize as isize)) as *mut plist_node
            });
            last = reverse_iter;
            loop {
                if (*node).prio < (*iter).prio {
                    node_next = &raw mut (*iter).node_list;
                    break;
                } else if (*node).prio >= (*reverse_iter).prio {
                    prev = reverse_iter;
                    iter = ({
                        let mut __mptr: *mut ::core::ffi::c_void =
                            (*reverse_iter).prio_list.next as *mut ::core::ffi::c_void;
                        __mptr.offset(-(8 as usize as isize)) as *mut plist_node
                    });
                    if (reverse_iter != last) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                        node_next = &raw mut (*iter).node_list;
                    }
                    break;
                } else {
                    prev = iter;
                    iter = ({
                        let mut __mptr: *mut ::core::ffi::c_void =
                            (*iter).prio_list.next as *mut ::core::ffi::c_void;
                        __mptr.offset(-(8 as usize as isize)) as *mut plist_node
                    });
                    reverse_iter = ({
                        let mut __mptr: *mut ::core::ffi::c_void =
                            (*reverse_iter).prio_list.prev as *mut ::core::ffi::c_void;
                        __mptr.offset(-(8 as usize as isize)) as *mut plist_node
                    });
                    if iter == first {
                        break;
                    }
                }
            }
            if prev.is_null() || (*prev).prio != (*node).prio {
                list_add_tail(&raw mut (*node).prio_list, &raw mut (*iter).prio_list);
            }
        }
        list_add_tail(&raw mut (*node).node_list, node_next);
    }
}
#[no_mangle]
pub unsafe extern "C" fn plist_del(mut node: *mut plist_node, mut head: *mut plist_head) {
    unsafe {
        if list_empty(&raw mut (*node).prio_list) == 0 {
            if (*node).node_list.next != &raw mut (*head).node_list {
                let mut next: *mut plist_node = ::core::ptr::null_mut::<plist_node>();
                next = ({
                    let mut __mptr: *mut ::core::ffi::c_void =
                        (*node).node_list.next as *mut ::core::ffi::c_void;
                    __mptr.offset(-(24 as usize as isize)) as *mut plist_node
                });
                if list_empty(&raw mut (*next).prio_list) != 0 {
                    list_add(&raw mut (*next).prio_list, &raw mut (*node).prio_list);
                }
            }
            list_del_init(&raw mut (*node).prio_list);
        }
        list_del_init(&raw mut (*node).node_list);
    }
}
#[no_mangle]
pub unsafe extern "C" fn plist_requeue(mut node: *mut plist_node, mut head: *mut plist_head) {
    unsafe {
        let mut iter: *mut plist_node = ::core::ptr::null_mut::<plist_node>();
        let mut node_next: *mut list_head = &raw mut (*head).node_list;
        if (plist_head_empty(head) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            asm!("ebreak\n", "\n", options(preserves_flags));
            unreachable!();
        }
        if (plist_node_empty(node) != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            asm!("ebreak\n", "\n", options(preserves_flags));
            unreachable!();
        }
        if node == plist_last(head) {
            return;
        }
        iter = ({
            let mut __mptr: *mut ::core::ffi::c_void =
                (*node).node_list.next as *mut ::core::ffi::c_void;
            __mptr.offset(-(24 as usize as isize)) as *mut plist_node
        }) as *mut plist_node;
        if (*node).prio != (*iter).prio {
            return;
        }
        plist_del(node, head);
        if list_empty(&raw mut (*iter).prio_list) == 0 {
            iter = ({
                let mut __mptr: *mut ::core::ffi::c_void =
                    (*iter).prio_list.next as *mut ::core::ffi::c_void;
                __mptr.offset(-(8 as usize as isize)) as *mut plist_node
            });
            node_next = &raw mut (*iter).node_list;
        } else {
            iter = ({
                let mut __mptr: *mut ::core::ffi::c_void =
                    (*iter).node_list.next as *mut ::core::ffi::c_void;
                __mptr.offset(-(24 as usize as isize)) as *mut plist_node
            }) as *mut plist_node;
            while list_is_head(&raw mut (*iter).node_list, &raw mut (*head).node_list) == 0 {
                if (*node).prio != (*iter).prio {
                    node_next = &raw mut (*iter).node_list;
                    break;
                } else {
                    iter = ({
                        let mut __mptr: *mut ::core::ffi::c_void =
                            (*iter).node_list.next as *mut ::core::ffi::c_void;
                        __mptr.offset(-(24 as usize as isize)) as *mut plist_node
                    }) as *mut plist_node;
                }
            }
        }
        list_add_tail(&raw mut (*node).node_list, node_next);
    }
}
