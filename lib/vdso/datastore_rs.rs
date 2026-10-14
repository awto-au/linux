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

// task_struct/mmap_action/percpu_ref_data/signal_struct/sched_dl_entity carry
// c2rust_bitfields::BitfieldStruct derives (proc-macro crate unavailable in
// this build) but are grep-confirmed used only by-pointer, or not at all,
// anywhere in this TU's real translated functions (vdso_setup_data_pages,
// vvar_fault, vdso_install_vvar_mapping) -- opaqued rather than
// derive-stripped, per the standard idiom (issue #28, tracked at #55).
//
// [bisect] mm_struct/vm_area_struct/alloc_tag/nodemask_t added to the opaque
// list: grep-confirmed used only by-pointer (mm_struct, vm_area_struct,
// alloc_tag) or only as a null pointer (nodemask_t) anywhere in this TU's
// real functions -- zero field access, zero by-value use. Opaquing them
// removes their real bodies (and everything only THEY transitively pulled
// in) from the bisection reduction without touching any real logic.
macro_rules! opaque_marker {
    ($($name:ident),* $(,)?) => {
        $(
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct $name {
                _private: [u8; 0],
            }
        )*
    };
}
opaque_marker!(
    task_struct, mmap_action, percpu_ref_data, signal_struct, sched_dl_entity,
    mm_struct, vm_area_struct, alloc_tag, nodemask_t,
);
use ::core::arch::asm;
use ::kernel::warn_on;
extern "C" {
    fn panic(fmt: *const ::core::ffi::c_char, ...) -> !;
    fn _printk(fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn dump_stack();
    fn __riscv_isa_extension_available(
        isa_bitmap: *const ::core::ffi::c_ulong,
        bit: ::core::ffi::c_uint,
    ) -> bool_0;
    fn __riscv_has_extension_likely(
        vendor: ::core::ffi::c_ulong,
        ext: ::core::ffi::c_ulong,
    ) -> bool_0;
    static mut kernel_map: kernel_mapping;
    static mut phys_ram_base: phys_addr_t;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut mem_map: *mut page;
    fn __alloc_pages_noprof(
        gfp: gfp_t,
        order: ::core::ffi::c_uint,
        preferred_nid: ::core::ffi::c_int,
        nodemask: *mut nodemask_t,
    ) -> *mut page;
    static mut pgtable_l4_enabled: bool_0;
    static mut pgtable_l5_enabled: bool_0;
    fn split_page(page: *mut page, order: ::core::ffi::c_uint);
    fn _install_special_mapping(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        len: ::core::ffi::c_ulong,
        vm_flags: vm_flags_t,
        spec: *const vm_special_mapping,
    ) -> *mut vm_area_struct;
    fn vm_insert_page(
        _: *mut vm_area_struct,
        addr: ::core::ffi::c_ulong,
        _: *mut page,
    ) -> ::core::ffi::c_int;
}
pub type __u8 = ::core::ffi::c_uchar;
pub type __u16 = ::core::ffi::c_ushort;
pub type __s32 = ::core::ffi::c_int;
pub type __u32 = ::core::ffi::c_uint;
pub type __s64 = ::core::ffi::c_longlong;
pub type __u64 = ::core::ffi::c_ulonglong;
pub type u8_0 = __u8;
pub type u16_0 = __u16;
pub type s32 = __s32;
pub type u32_0 = __u32;
pub type s64 = __s64;
pub type u64_0 = __u64;
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const r#true: C2Rust_Unnamed = 1;
pub const r#false: C2Rust_Unnamed = 0;
pub type __kernel_ulong_t = ::core::ffi::c_ulong;
pub type __kernel_uid32_t = ::core::ffi::c_uint;
pub type __kernel_gid32_t = ::core::ffi::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_loff_t = ::core::ffi::c_longlong;
pub type __poll_t = ::core::ffi::c_uint;
pub type __kernel_dev_t = u32_0;
pub type dev_t = __kernel_dev_t;
pub type umode_t = ::core::ffi::c_ushort;
pub type bool_0 = bool;
pub type uid_t = __kernel_uid32_t;
pub type gid_t = __kernel_gid32_t;
pub type uintptr_t = usize;
pub type loff_t = __kernel_loff_t;
pub type size_t = __kernel_size_t;
pub type ssize_t = isize;
pub type uint32_t = u32;
pub type sector_t = u64_0;
pub type blkcnt_t = u64_0;
pub type gfp_t = ::core::ffi::c_uint;
pub type fmode_t = ::core::ffi::c_uint;
pub type phys_addr_t = u64_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct atomic_t {
    pub counter: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct atomic64_t {
    pub counter: s64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct callback_head(pub C2Rust_callback_head_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_callback_head_Inner {
    pub next: *mut callback_head,
    pub func: Option<unsafe extern "C" fn(*mut callback_head) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rcuwait {
    pub task: *mut task_struct,
}
pub type refcount_t = refcount_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refcount_struct {
    pub refs: atomic_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct page {
    pub flags: memdesc_flags_t,
    pub c2rust_unnamed: C2Rust_Unnamed_1,
    pub c2rust_unnamed_0: C2Rust_Unnamed_0,
    pub _refcount: atomic_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub page_type: ::core::ffi::c_uint,
    pub _mapcount: atomic_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub c2rust_unnamed: C2Rust_Unnamed_5,
    pub c2rust_unnamed_0: C2Rust_Unnamed_4,
    pub c2rust_unnamed_1: C2Rust_Unnamed_3,
    pub c2rust_unnamed_2: C2Rust_Unnamed_2,
    pub callback_head: callback_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub _unused_pgmap_compound_info: *mut ::core::ffi::c_void,
    pub zone_device_data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub compound_info: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub pp_magic: ::core::ffi::c_ulong,
    pub pp: *mut page_pool,
    pub _pp_mapping_pad: ::core::ffi::c_ulong,
    pub dma_addr: ::core::ffi::c_ulong,
    pub pp_ref_count: atomic_long_t,
}
pub type atomic_long_t = atomic64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct page_pool {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub c2rust_unnamed: C2Rust_Unnamed_59,
    pub mapping: *mut address_space,
    pub c2rust_unnamed_0: C2Rust_Unnamed_6,
    pub private: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_6 {
    pub __folio_index: ::core::ffi::c_ulong,
    pub share: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct address_space {
    pub host: *mut inode,
    pub i_pages: xarray,
    pub invalidate_lock: rw_semaphore,
    pub gfp_mask: gfp_t,
    pub i_mmap_writable: atomic_t,
    pub i_mmap: rb_root_cached,
    pub nrpages: ::core::ffi::c_ulong,
    pub writeback_index: ::core::ffi::c_ulong,
    pub a_ops: *const address_space_operations,
    pub flags: ::core::ffi::c_ulong,
    pub wb_err: errseq_t,
    pub i_private_lock: spinlock_t,
    pub i_mmap_rwsem: rw_semaphore,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rw_semaphore {
    pub count: atomic_long_t,
    pub owner: atomic_long_t,
    pub wait_lock: raw_spinlock_t,
    pub first_waiter: *mut rwsem_waiter,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rwsem_waiter {
    _private: [u8; 0],
}
pub type raw_spinlock_t = raw_spinlock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct raw_spinlock {
    pub raw_lock: arch_spinlock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arch_spinlock_t {}
pub type spinlock_t = spinlock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spinlock {
    pub c2rust_unnamed: C2Rust_Unnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_7 {
    pub rlock: raw_spinlock,
}
pub type errseq_t = u32_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct address_space_operations {
    pub read_folio: Option<unsafe extern "C" fn(*mut file, *mut folio) -> ::core::ffi::c_int>,
    pub writepages: Option<
        unsafe extern "C" fn(*mut address_space, *mut writeback_control) -> ::core::ffi::c_int,
    >,
    pub dirty_folio: Option<unsafe extern "C" fn(*mut address_space, *mut folio) -> bool_0>,
    pub readahead: Option<unsafe extern "C" fn(*mut readahead_control) -> ()>,
    pub write_begin: Option<
        unsafe extern "C" fn(
            *const kiocb,
            *mut address_space,
            loff_t,
            ::core::ffi::c_uint,
            *mut *mut folio,
            *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub write_end: Option<
        unsafe extern "C" fn(
            *const kiocb,
            *mut address_space,
            loff_t,
            ::core::ffi::c_uint,
            ::core::ffi::c_uint,
            *mut folio,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub bmap: Option<unsafe extern "C" fn(*mut address_space, sector_t) -> sector_t>,
    pub invalidate_folio: Option<unsafe extern "C" fn(*mut folio, size_t, size_t) -> ()>,
    pub release_folio: Option<unsafe extern "C" fn(*mut folio, gfp_t) -> bool_0>,
    pub free_folio: Option<unsafe extern "C" fn(*mut folio) -> ()>,
    pub direct_IO: Option<unsafe extern "C" fn(*mut kiocb, *mut iov_iter) -> ssize_t>,
    pub migrate_folio: Option<
        unsafe extern "C" fn(
            *mut address_space,
            *mut folio,
            *mut folio,
            migrate_mode,
        ) -> ::core::ffi::c_int,
    >,
    pub launder_folio: Option<unsafe extern "C" fn(*mut folio) -> ::core::ffi::c_int>,
    pub is_partially_uptodate: Option<unsafe extern "C" fn(*mut folio, size_t, size_t) -> bool_0>,
    pub is_dirty_writeback:
        Option<unsafe extern "C" fn(*mut folio, *mut bool_0, *mut bool_0) -> ()>,
    pub error_remove_folio:
        Option<unsafe extern "C" fn(*mut address_space, *mut folio) -> ::core::ffi::c_int>,
    pub swap_activate: Option<
        unsafe extern "C" fn(*mut swap_info_struct, *mut file, *mut sector_t) -> ::core::ffi::c_int,
    >,
    pub swap_deactivate: Option<unsafe extern "C" fn(*mut file) -> ()>,
    pub swap_rw: Option<unsafe extern "C" fn(*mut kiocb, *mut iov_iter) -> ::core::ffi::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iov_iter {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kiocb {
    pub ki_filp: *mut file,
    pub ki_pos: loff_t,
    pub ki_complete: Option<unsafe extern "C" fn(*mut kiocb, ::core::ffi::c_long) -> ()>,
    pub private: *mut ::core::ffi::c_void,
    pub ki_flags: ::core::ffi::c_int,
    pub ki_ioprio: u16_0,
    pub ki_write_stream: u8_0,
    pub ki_waitq: *mut wait_page_queue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wait_page_queue {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file {
    pub f_lock: spinlock_t,
    pub f_mode: fmode_t,
    pub f_op: *const file_operations,
    pub f_mapping: *mut address_space,
    pub private_data: *mut ::core::ffi::c_void,
    pub f_inode: *mut inode,
    pub f_flags: ::core::ffi::c_uint,
    pub f_iocb_flags: ::core::ffi::c_uint,
    pub f_cred: *const cred,
    pub f_owner: *mut fown_struct,
    pub c2rust_unnamed: C2Rust_Unnamed_10,
    pub c2rust_unnamed_0: C2Rust_Unnamed_9,
    pub f_pos: loff_t,
    pub f_wb_err: errseq_t,
    pub f_sb_err: errseq_t,
    pub c2rust_unnamed_1: C2Rust_Unnamed_8,
    pub f_ref: file_ref_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_ref_t {
    pub refcnt: atomic64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
    pub f_task_work: callback_head,
    pub f_llist: llist_node,
    pub f_ra: file_ra_state,
    pub f_freeptr: freeptr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freeptr_t {
    pub v: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_ra_state {
    pub start: ::core::ffi::c_ulong,
    pub size: ::core::ffi::c_uint,
    pub async_size: ::core::ffi::c_uint,
    pub ra_pages: ::core::ffi::c_uint,
    pub order: ::core::ffi::c_ushort,
    pub mmap_miss: ::core::ffi::c_ushort,
    pub prev_pos: loff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct llist_node {
    pub next: *mut llist_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_9 {
    pub f_pos_lock: mutex,
    pub f_pipe: u64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mutex {
    pub owner: atomic_long_t,
    pub wait_lock: raw_spinlock_t,
    pub first_waiter: *mut mutex_waiter,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mutex_waiter {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_10 {
    pub f_path: path,
    pub __f_path: path,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct path {
    pub mnt: *mut vfsmount,
    pub dentry: *mut dentry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dentry {
    pub d_flags: ::core::ffi::c_uint,
    pub d_seq: seqcount_spinlock_t,
    pub d_hash: hlist_bl_node,
    pub d_parent: *mut dentry,
    pub c2rust_unnamed: C2Rust_Unnamed_38,
    pub d_inode: *mut inode,
    pub d_shortname: shortname_store,
    pub d_op: *const dentry_operations,
    pub d_sb: *mut super_block,
    pub d_time: ::core::ffi::c_ulong,
    pub d_fsdata: *mut ::core::ffi::c_void,
    pub d_lockref: lockref,
    pub d_lru: list_head,
    pub d_sib: hlist_node,
    pub d_children: hlist_head,
    pub c2rust_unnamed_0: C2Rust_Unnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_11 {
    pub d_alias: hlist_node,
    pub d_in_lookup_hash: hlist_bl_node,
    pub d_rcu: callback_head,
    pub waiters: *mut completion_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct completion_list {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hlist_bl_node {
    pub next: *mut hlist_bl_node,
    pub pprev: *mut *mut hlist_bl_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lockref {
    pub c2rust_unnamed: C2Rust_Unnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_12 {
    pub c2rust_unnamed: C2Rust_Unnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
    pub lock: spinlock_t,
    pub count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct super_block {
    pub s_list: list_head,
    pub s_dev: dev_t,
    pub s_blocksize_bits: ::core::ffi::c_uchar,
    pub s_blocksize: ::core::ffi::c_ulong,
    pub s_maxbytes: loff_t,
    pub s_type: *mut file_system_type,
    pub s_op: *const super_operations,
    pub dq_op: *const dquot_operations,
    pub s_qcop: *const quotactl_ops,
    pub s_export_op: *const export_operations,
    pub s_flags: ::core::ffi::c_ulong,
    pub s_iflags: ::core::ffi::c_ulong,
    pub s_magic: ::core::ffi::c_ulong,
    pub s_root: *mut dentry,
    pub s_umount: rw_semaphore,
    pub s_count: ::core::ffi::c_int,
    pub s_active: atomic_t,
    pub s_xattr: *const *const xattr_handler,
    pub s_roots: hlist_head,
    pub s_roots_lock: spinlock_t,
    pub s_mounts: *mut mount,
    pub s_bdev: *mut block_device,
    pub s_bdev_file: *mut file,
    pub s_bdi: *mut backing_dev_info,
    pub s_mtd: *mut mtd_info,
    pub s_instances: hlist_node,
    pub s_quota_types: ::core::ffi::c_uint,
    pub s_dquot: quota_info,
    pub s_writers: sb_writers,
    pub s_fs_info: *mut ::core::ffi::c_void,
    pub s_time_gran: u32_0,
    pub s_time_min: time64_t,
    pub s_time_max: time64_t,
    pub s_id: [::core::ffi::c_char; 32],
    pub s_uuid: uuid_t,
    pub s_uuid_len: u8_0,
    pub s_sysfs_name: [::core::ffi::c_char; 37],
    pub s_max_links: ::core::ffi::c_uint,
    pub s_d_flags: ::core::ffi::c_uint,
    pub s_vfs_rename_mutex: mutex,
    pub s_subtype: *const ::core::ffi::c_char,
    pub __s_d_op: *const dentry_operations,
    pub s_shrink: *mut shrinker,
    pub s_remove_count: atomic_long_t,
    pub s_readonly_remount: ::core::ffi::c_int,
    pub s_wb_err: errseq_t,
    pub s_dio_done_wq: *mut workqueue_struct,
    pub s_pins: hlist_head,
    pub s_user_ns: *mut user_namespace,
    pub s_dentry_lru: list_lru,
    pub s_inode_lru: list_lru,
    pub rcu: callback_head,
    pub destroy_work: work_struct,
    pub s_sync_lock: mutex,
    pub s_stack_depth: ::core::ffi::c_int,
    pub s_inode_list_lock: spinlock_t,
    pub s_inodes: list_head,
    pub s_inode_wblist_lock: spinlock_t,
    pub s_inodes_wb: list_head,
    pub s_min_writeback_pages: ::core::ffi::c_long,
    pub s_pending_errors: refcount_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct work_struct {
    pub data: atomic_long_t,
    pub entry: list_head,
    pub func: work_func_t,
}
pub type work_func_t = Option<unsafe extern "C" fn(*mut work_struct) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_lru {
    pub node: *mut list_lru_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_lru_node {
    pub lru: list_lru_one,
    pub nr_items: atomic_long_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_lru_one {
    pub list: list_head,
    pub nr_items: ::core::ffi::c_long,
    pub lock: spinlock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct user_namespace {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shrinker {
    pub count_objects:
        Option<unsafe extern "C" fn(*mut shrinker, *mut shrink_control) -> ::core::ffi::c_ulong>,
    pub scan_objects:
        Option<unsafe extern "C" fn(*mut shrinker, *mut shrink_control) -> ::core::ffi::c_ulong>,
    pub batch: ::core::ffi::c_long,
    pub seeks: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_uint,
    pub refcount: refcount_t,
    pub done: completion,
    pub rcu: callback_head,
    pub private_data: *mut ::core::ffi::c_void,
    pub list: list_head,
    pub nr_deferred: *mut atomic_long_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct completion {
    pub done: ::core::ffi::c_uint,
    pub wait: swait_queue_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swait_queue_head {
    pub lock: raw_spinlock_t,
    pub task_list: list_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shrink_control {
    pub gfp_mask: gfp_t,
    pub nid: ::core::ffi::c_int,
    pub nr_to_scan: ::core::ffi::c_ulong,
    pub nr_scanned: ::core::ffi::c_ulong,
    pub memcg: *mut mem_cgroup,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mem_cgroup {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dentry_operations {
    pub d_revalidate: Option<
        unsafe extern "C" fn(
            *mut inode,
            *const qstr,
            *mut dentry,
            ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int,
    >,
    pub d_weak_revalidate:
        Option<unsafe extern "C" fn(*mut dentry, ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub d_hash: Option<unsafe extern "C" fn(*const dentry, *mut qstr) -> ::core::ffi::c_int>,
    pub d_compare: Option<
        unsafe extern "C" fn(
            *const dentry,
            ::core::ffi::c_uint,
            *const ::core::ffi::c_char,
            *const qstr,
        ) -> ::core::ffi::c_int,
    >,
    pub d_delete: Option<unsafe extern "C" fn(*const dentry) -> ::core::ffi::c_int>,
    pub d_init: Option<unsafe extern "C" fn(*mut dentry) -> ::core::ffi::c_int>,
    pub d_release: Option<unsafe extern "C" fn(*mut dentry) -> ()>,
    pub d_prune: Option<unsafe extern "C" fn(*mut dentry) -> ()>,
    pub d_iput: Option<unsafe extern "C" fn(*mut dentry, *mut inode) -> ()>,
    pub d_dname: Option<
        unsafe extern "C" fn(
            *mut dentry,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char,
    >,
    pub d_automount: Option<unsafe extern "C" fn(*mut path) -> *mut vfsmount>,
    pub d_manage: Option<unsafe extern "C" fn(*const path, bool_0) -> ::core::ffi::c_int>,
    pub d_real: Option<unsafe extern "C" fn(*mut dentry, d_real_type) -> *mut dentry>,
    pub d_unalias_trylock: Option<unsafe extern "C" fn(*const dentry) -> bool_0>,
    pub d_unalias_unlock: Option<unsafe extern "C" fn(*const dentry) -> ()>,
}
pub type d_real_type = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vfsmount {
    pub mnt_root: *mut dentry,
    pub mnt_sb: *mut super_block,
    pub mnt_flags: ::core::ffi::c_int,
    pub mnt_idmap: *mut mnt_idmap,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mnt_idmap {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inode {
    pub i_mode: umode_t,
    pub i_opflags: ::core::ffi::c_ushort,
    pub i_flags: ::core::ffi::c_uint,
    pub i_uid: kuid_t,
    pub i_gid: kgid_t,
    pub i_op: *const inode_operations,
    pub i_sb: *mut super_block,
    pub i_mapping: *mut address_space,
    pub i_ino: u64_0,
    pub c2rust_unnamed: C2Rust_Unnamed_32,
    pub i_rdev: dev_t,
    pub i_size: loff_t,
    pub i_atime_sec: time64_t,
    pub i_mtime_sec: time64_t,
    pub i_ctime_sec: time64_t,
    pub i_atime_nsec: u32_0,
    pub i_mtime_nsec: u32_0,
    pub i_ctime_nsec: u32_0,
    pub i_generation: u32_0,
    pub i_lock: spinlock_t,
    pub i_bytes: ::core::ffi::c_ushort,
    pub i_blkbits: u8_0,
    pub i_write_hint: rw_hint,
    pub i_blocks: blkcnt_t,
    pub i_state: inode_state_flags,
    pub i_rwsem: rw_semaphore,
    pub dirtied_when: ::core::ffi::c_ulong,
    pub dirtied_time_when: ::core::ffi::c_ulong,
    pub i_hash: hlist_node,
    pub i_io_list: list_head,
    pub i_lru: list_head,
    pub i_sb_list: list_head,
    pub i_wb_list: list_head,
    pub c2rust_unnamed_0: C2Rust_Unnamed_31,
    pub i_version: atomic64_t,
    pub i_sequence: atomic64_t,
    pub i_count: atomic_t,
    pub i_dio_count: atomic_t,
    pub i_writecount: atomic_t,
    pub c2rust_unnamed_1: C2Rust_Unnamed_16,
    pub i_flctx: *mut file_lock_context,
    pub i_data: address_space,
    pub c2rust_unnamed_2: C2Rust_Unnamed_15,
    pub c2rust_unnamed_3: C2Rust_Unnamed_14,
    pub i_private: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_14 {
    pub i_pipe: *mut pipe_inode_info,
    pub i_cdev: *mut cdev,
    pub i_link: *mut ::core::ffi::c_char,
    pub i_dir_seq: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cdev {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pipe_inode_info {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_15 {
    pub i_devices: list_head,
    pub i_linklen: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_lock_context {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_16 {
    pub i_fop: *const file_operations,
    pub free_inode: Option<unsafe extern "C" fn(*mut inode) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_operations {
    pub owner: *mut module,
    pub fop_flags: fop_flags_t,
    pub llseek: Option<unsafe extern "C" fn(*mut file, loff_t, ::core::ffi::c_int) -> loff_t>,
    pub read: Option<
        unsafe extern "C" fn(*mut file, *mut ::core::ffi::c_char, size_t, *mut loff_t) -> ssize_t,
    >,
    pub write: Option<
        unsafe extern "C" fn(*mut file, *const ::core::ffi::c_char, size_t, *mut loff_t) -> ssize_t,
    >,
    pub read_iter: Option<unsafe extern "C" fn(*mut kiocb, *mut iov_iter) -> ssize_t>,
    pub write_iter: Option<unsafe extern "C" fn(*mut kiocb, *mut iov_iter) -> ssize_t>,
    pub iopoll: Option<
        unsafe extern "C" fn(
            *mut kiocb,
            *mut io_comp_batch,
            ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int,
    >,
    pub iterate_shared:
        Option<unsafe extern "C" fn(*mut file, *mut dir_context) -> ::core::ffi::c_int>,
    pub poll: Option<unsafe extern "C" fn(*mut file, *mut poll_table_struct) -> __poll_t>,
    pub unlocked_ioctl: Option<
        unsafe extern "C" fn(
            *mut file,
            ::core::ffi::c_uint,
            ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_long,
    >,
    pub compat_ioctl: Option<
        unsafe extern "C" fn(
            *mut file,
            ::core::ffi::c_uint,
            ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_long,
    >,
    pub mmap: Option<unsafe extern "C" fn(*mut file, *mut vm_area_struct) -> ::core::ffi::c_int>,
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> ::core::ffi::c_int>,
    pub flush: Option<unsafe extern "C" fn(*mut file, fl_owner_t) -> ::core::ffi::c_int>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> ::core::ffi::c_int>,
    pub fsync: Option<
        unsafe extern "C" fn(*mut file, loff_t, loff_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub fasync: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *mut file,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub lock: Option<
        unsafe extern "C" fn(*mut file, ::core::ffi::c_int, *mut file_lock) -> ::core::ffi::c_int,
    >,
    pub get_unmapped_area: Option<
        unsafe extern "C" fn(
            *mut file,
            ::core::ffi::c_ulong,
            ::core::ffi::c_ulong,
            ::core::ffi::c_ulong,
            ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_ulong,
    >,
    pub check_flags: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub flock: Option<
        unsafe extern "C" fn(*mut file, ::core::ffi::c_int, *mut file_lock) -> ::core::ffi::c_int,
    >,
    pub splice_write: Option<
        unsafe extern "C" fn(
            *mut pipe_inode_info,
            *mut file,
            *mut loff_t,
            size_t,
            ::core::ffi::c_uint,
        ) -> ssize_t,
    >,
    pub splice_read: Option<
        unsafe extern "C" fn(
            *mut file,
            *mut loff_t,
            *mut pipe_inode_info,
            size_t,
            ::core::ffi::c_uint,
        ) -> ssize_t,
    >,
    pub splice_eof: Option<unsafe extern "C" fn(*mut file) -> ()>,
    pub setlease: Option<
        unsafe extern "C" fn(
            *mut file,
            ::core::ffi::c_int,
            *mut *mut file_lease,
            *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub fallocate: Option<
        unsafe extern "C" fn(*mut file, ::core::ffi::c_int, loff_t, loff_t) -> ::core::ffi::c_long,
    >,
    pub show_fdinfo: Option<unsafe extern "C" fn(*mut seq_file, *mut file) -> ()>,
    pub copy_file_range: Option<
        unsafe extern "C" fn(
            *mut file,
            loff_t,
            *mut file,
            loff_t,
            size_t,
            ::core::ffi::c_uint,
        ) -> ssize_t,
    >,
    pub remap_file_range: Option<
        unsafe extern "C" fn(
            *mut file,
            loff_t,
            *mut file,
            loff_t,
            loff_t,
            ::core::ffi::c_uint,
        ) -> loff_t,
    >,
    pub fadvise: Option<
        unsafe extern "C" fn(*mut file, loff_t, loff_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub uring_cmd:
        Option<unsafe extern "C" fn(*mut io_uring_cmd, ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub uring_cmd_iopoll: Option<
        unsafe extern "C" fn(
            *mut io_uring_cmd,
            *mut io_comp_batch,
            ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int,
    >,
    pub mmap_prepare: Option<unsafe extern "C" fn(*mut vm_area_desc) -> ::core::ffi::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vm_area_desc {
    pub mm: *mut mm_struct,
    pub file: *mut file,
    pub start: ::core::ffi::c_ulong,
    pub end: ::core::ffi::c_ulong,
    pub pgoff: ::core::ffi::c_ulong,
    pub vm_file: *mut file,
    pub vma_flags: vma_flags_t,
    pub page_prot: pgprot_t,
    pub vm_ops: *const vm_operations_struct,
    pub private_data: *mut ::core::ffi::c_void,
    pub action: mmap_action,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pgprot_t {
    pub pgprot: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vm_operations_struct {
    pub open: Option<unsafe extern "C" fn(*mut vm_area_struct) -> ()>,
    pub close: Option<unsafe extern "C" fn(*mut vm_area_struct) -> ()>,
    pub mapped: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_ulong,
            ::core::ffi::c_ulong,
            ::core::ffi::c_ulong,
            *const file,
            *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub may_split: Option<
        unsafe extern "C" fn(*mut vm_area_struct, ::core::ffi::c_ulong) -> ::core::ffi::c_int,
    >,
    pub mremap: Option<unsafe extern "C" fn(*mut vm_area_struct) -> ::core::ffi::c_int>,
    pub mprotect: Option<
        unsafe extern "C" fn(
            *mut vm_area_struct,
            ::core::ffi::c_ulong,
            ::core::ffi::c_ulong,
            ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_int,
    >,
    pub fault: Option<unsafe extern "C" fn(*mut vm_fault) -> vm_fault_t>,
    pub huge_fault: Option<unsafe extern "C" fn(*mut vm_fault, ::core::ffi::c_uint) -> vm_fault_t>,
    pub map_pages: Option<
        unsafe extern "C" fn(
            *mut vm_fault,
            ::core::ffi::c_ulong,
            ::core::ffi::c_ulong,
        ) -> vm_fault_t,
    >,
    pub pagesize: Option<unsafe extern "C" fn(*mut vm_area_struct) -> ::core::ffi::c_ulong>,
    pub page_mkwrite: Option<unsafe extern "C" fn(*mut vm_fault) -> vm_fault_t>,
    pub pfn_mkwrite: Option<unsafe extern "C" fn(*mut vm_fault) -> vm_fault_t>,
    pub access: Option<
        unsafe extern "C" fn(
            *mut vm_area_struct,
            ::core::ffi::c_ulong,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub name: Option<unsafe extern "C" fn(*mut vm_area_struct) -> *const ::core::ffi::c_char>,
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct rb_node(pub C2Rust_rb_node_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_rb_node_Inner {
    pub __rb_parent_color: ::core::ffi::c_ulong,
    pub rb_right: *mut rb_node,
    pub rb_left: *mut rb_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vma_flags_t {
    pub __vma_flags: [::core::ffi::c_ulong; 1],
}
pub type vm_flags_t = ::core::ffi::c_ulong;
pub type seqcount_t = seqcount;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqcount {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pgd_t {
    pub pgd: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct maple_tree {
    pub c2rust_unnamed: C2Rust_Unnamed_25,
    pub ma_flags: ::core::ffi::c_uint,
    pub ma_root: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_25 {
    pub ma_lock: spinlock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vm_fault {
    pub c2rust_unnamed: C2Rust_Unnamed_30,
    pub flags: fault_flag,
    pub pmd: *mut pmd_t,
    pub pud: *mut pud_t,
    pub c2rust_unnamed_0: C2Rust_Unnamed_29,
    pub cow_page: *mut page,
    pub page: *mut page,
    pub pte: *mut pte_t,
    pub ptl: *mut spinlock_t,
    pub prealloc_pte: pgtable_t,
}
pub type pgtable_t = *mut page;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pte_t {
    pub pte: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_29 {
    pub orig_pte: pte_t,
    pub orig_pmd: pmd_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pmd_t {
    pub pmd: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pud_t {
    pub pud: ::core::ffi::c_ulong,
}
pub type fault_flag = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_30 {
    pub vma: *mut vm_area_struct,
    pub gfp_mask: gfp_t,
    pub pgoff: ::core::ffi::c_ulong,
    pub address: ::core::ffi::c_ulong,
    pub real_address: ::core::ffi::c_ulong,
}
pub type vm_fault_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct io_comp_batch {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct io_uring_cmd {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_lease {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_lock {
    _private: [u8; 0],
}
pub type fl_owner_t = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct poll_table_struct {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dir_context {
    pub actor: filldir_t,
    pub pos: loff_t,
    pub count: ::core::ffi::c_int,
    pub dt_flags_mask: ::core::ffi::c_uint,
}
pub type filldir_t = Option<
    unsafe extern "C" fn(
        *mut dir_context,
        *const ::core::ffi::c_char,
        ::core::ffi::c_int,
        loff_t,
        u64_0,
        ::core::ffi::c_uint,
    ) -> bool_0,
>;
pub type fop_flags_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct module {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_31 {
    pub i_dentry: hlist_head,
    pub i_rcu: callback_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inode_state_flags {
    pub __state: inode_state_flags_enum,
}
pub type inode_state_flags_enum = ::core::ffi::c_uint;
pub type rw_hint = ::core::ffi::c_uchar;
pub type time64_t = __s64;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_32 {
    pub i_nlink: ::core::ffi::c_uint,
    pub __i_nlink: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inode_operations {
    pub lookup:
        Option<unsafe extern "C" fn(*mut inode, *mut dentry, ::core::ffi::c_uint) -> *mut dentry>,
    pub get_link: Option<
        unsafe extern "C" fn(
            *mut dentry,
            *mut inode,
            *mut delayed_call,
        ) -> *const ::core::ffi::c_char,
    >,
    pub permission: Option<
        unsafe extern "C" fn(*mut mnt_idmap, *mut inode, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub get_inode_acl:
        Option<unsafe extern "C" fn(*mut inode, ::core::ffi::c_int, bool_0) -> *mut posix_acl>,
    pub readlink: Option<
        unsafe extern "C" fn(
            *mut dentry,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub create: Option<
        unsafe extern "C" fn(
            *mut mnt_idmap,
            *mut inode,
            *mut dentry,
            umode_t,
            bool_0,
        ) -> ::core::ffi::c_int,
    >,
    pub link:
        Option<unsafe extern "C" fn(*mut dentry, *mut inode, *mut dentry) -> ::core::ffi::c_int>,
    pub unlink: Option<unsafe extern "C" fn(*mut inode, *mut dentry) -> ::core::ffi::c_int>,
    pub symlink: Option<
        unsafe extern "C" fn(
            *mut mnt_idmap,
            *mut inode,
            *mut dentry,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub mkdir: Option<
        unsafe extern "C" fn(*mut mnt_idmap, *mut inode, *mut dentry, umode_t) -> *mut dentry,
    >,
    pub rmdir: Option<unsafe extern "C" fn(*mut inode, *mut dentry) -> ::core::ffi::c_int>,
    pub mknod: Option<
        unsafe extern "C" fn(
            *mut mnt_idmap,
            *mut inode,
            *mut dentry,
            umode_t,
            dev_t,
        ) -> ::core::ffi::c_int,
    >,
    pub rename: Option<
        unsafe extern "C" fn(
            *mut mnt_idmap,
            *mut inode,
            *mut dentry,
            *mut inode,
            *mut dentry,
            ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int,
    >,
    pub setattr:
        Option<unsafe extern "C" fn(*mut mnt_idmap, *mut dentry, *mut iattr) -> ::core::ffi::c_int>,
    pub getattr: Option<
        unsafe extern "C" fn(
            *mut mnt_idmap,
            *const path,
            *mut kstat,
            u32_0,
            ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int,
    >,
    pub listxattr:
        Option<unsafe extern "C" fn(*mut dentry, *mut ::core::ffi::c_char, size_t) -> ssize_t>,
    pub fiemap: Option<
        unsafe extern "C" fn(
            *mut inode,
            *mut fiemap_extent_info,
            u64_0,
            u64_0,
        ) -> ::core::ffi::c_int,
    >,
    pub update_time: Option<
        unsafe extern "C" fn(*mut inode, fs_update_time, ::core::ffi::c_uint) -> ::core::ffi::c_int,
    >,
    pub sync_lazytime: Option<unsafe extern "C" fn(*mut inode) -> ()>,
    pub atomic_open: Option<
        unsafe extern "C" fn(
            *mut inode,
            *mut dentry,
            *mut file,
            ::core::ffi::c_uint,
            umode_t,
        ) -> ::core::ffi::c_int,
    >,
    pub tmpfile: Option<
        unsafe extern "C" fn(*mut mnt_idmap, *mut inode, *mut file, umode_t) -> ::core::ffi::c_int,
    >,
    pub get_acl: Option<
        unsafe extern "C" fn(*mut mnt_idmap, *mut dentry, ::core::ffi::c_int) -> *mut posix_acl,
    >,
    pub set_acl: Option<
        unsafe extern "C" fn(
            *mut mnt_idmap,
            *mut dentry,
            *mut posix_acl,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub fileattr_set: Option<
        unsafe extern "C" fn(*mut mnt_idmap, *mut dentry, *mut file_kattr) -> ::core::ffi::c_int,
    >,
    pub fileattr_get:
        Option<unsafe extern "C" fn(*mut dentry, *mut file_kattr) -> ::core::ffi::c_int>,
    pub get_offset_ctx: Option<unsafe extern "C" fn(*mut inode) -> *mut offset_ctx>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct offset_ctx {
    pub mt: maple_tree,
    pub next_offset: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_kattr {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_acl {
    _private: [u8; 0],
}
pub type fs_update_time = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fiemap_extent_info {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kstat {
    pub result_mask: u32_0,
    pub mode: umode_t,
    pub nlink: ::core::ffi::c_uint,
    pub blksize: uint32_t,
    pub attributes: u64_0,
    pub attributes_mask: u64_0,
    pub ino: u64_0,
    pub dev: dev_t,
    pub rdev: dev_t,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub size: loff_t,
    pub atime: timespec64,
    pub mtime: timespec64,
    pub ctime: timespec64,
    pub btime: timespec64,
    pub blocks: u64_0,
    pub mnt_id: u64_0,
    pub change_cookie: u64_0,
    pub subvol: u64_0,
    pub dio_mem_align: u32_0,
    pub dio_offset_align: u32_0,
    pub dio_read_offset_align: u32_0,
    pub atomic_write_unit_min: u32_0,
    pub atomic_write_unit_max: u32_0,
    pub atomic_write_unit_max_opt: u32_0,
    pub atomic_write_segments_max: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec64 {
    pub tv_sec: time64_t,
    pub tv_nsec: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kgid_t {
    pub val: gid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kuid_t {
    pub val: uid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iattr {
    pub ia_valid: ::core::ffi::c_uint,
    pub ia_mode: umode_t,
    pub c2rust_unnamed: C2Rust_Unnamed_34,
    pub c2rust_unnamed_0: C2Rust_Unnamed_33,
    pub ia_size: loff_t,
    pub ia_atime: timespec64,
    pub ia_mtime: timespec64,
    pub ia_ctime: timespec64,
    pub ia_file: *mut file,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_33 {
    pub ia_gid: kgid_t,
    pub ia_vfsgid: vfsgid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vfsgid_t {
    pub val: gid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_34 {
    pub ia_uid: kuid_t,
    pub ia_vfsuid: vfsuid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vfsuid_t {
    pub val: uid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delayed_call {
    pub r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub arg: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qstr {
    pub c2rust_unnamed: C2Rust_Unnamed_35,
    pub name: *const ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_35 {
    pub c2rust_unnamed: C2Rust_Unnamed_36,
    pub hash_len: u64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_36 {
    pub hash: u32_0,
    pub len: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuid_t {
    pub b: [__u8; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sb_writers {
    pub frozen: ::core::ffi::c_ushort,
    pub freeze_kcount: ::core::ffi::c_int,
    pub freeze_ucount: ::core::ffi::c_int,
    pub freeze_owner: *const ::core::ffi::c_void,
    pub rw_sem: [percpu_rw_semaphore; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct percpu_rw_semaphore {
    pub rss: rcu_sync,
    pub read_count: *mut ::core::ffi::c_uint,
    pub writer: rcuwait,
    pub waiters: wait_queue_head_t,
    pub block: atomic_t,
}
pub type wait_queue_head_t = wait_queue_head;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wait_queue_head {
    pub lock: spinlock_t,
    pub head: list_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rcu_sync {
    pub gp_state: ::core::ffi::c_int,
    pub gp_count: ::core::ffi::c_int,
    pub gp_wait: wait_queue_head_t,
    pub cb_head: callback_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quota_info {
    pub flags: ::core::ffi::c_uint,
    pub dqio_sem: rw_semaphore,
    pub files: [*mut inode; 3],
    pub info: [mem_dqinfo; 3],
    pub ops: [*const quota_format_ops; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quota_format_ops {
    pub check_quota_file:
        Option<unsafe extern "C" fn(*mut super_block, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub read_file_info:
        Option<unsafe extern "C" fn(*mut super_block, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub write_file_info:
        Option<unsafe extern "C" fn(*mut super_block, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub free_file_info:
        Option<unsafe extern "C" fn(*mut super_block, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub read_dqblk: Option<unsafe extern "C" fn(*mut dquot) -> ::core::ffi::c_int>,
    pub commit_dqblk: Option<unsafe extern "C" fn(*mut dquot) -> ::core::ffi::c_int>,
    pub release_dqblk: Option<unsafe extern "C" fn(*mut dquot) -> ::core::ffi::c_int>,
    pub get_next_id:
        Option<unsafe extern "C" fn(*mut super_block, *mut kqid) -> ::core::ffi::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kqid {
    pub c2rust_unnamed: C2Rust_Unnamed_37,
    pub r#type: quota_type,
}
pub type quota_type = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_37 {
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub projid: kprojid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kprojid_t {
    pub val: projid_t,
}
pub type projid_t = __kernel_uid32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dquot {
    pub dq_hash: hlist_node,
    pub dq_inuse: list_head,
    pub dq_free: list_head,
    pub dq_dirty: list_head,
    pub dq_lock: mutex,
    pub dq_dqb_lock: spinlock_t,
    pub dq_count: atomic_t,
    pub dq_sb: *mut super_block,
    pub dq_id: kqid,
    pub dq_off: loff_t,
    pub dq_flags: ::core::ffi::c_ulong,
    pub dq_dqb: mem_dqblk,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mem_dqblk {
    pub dqb_bhardlimit: qsize_t,
    pub dqb_bsoftlimit: qsize_t,
    pub dqb_curspace: qsize_t,
    pub dqb_rsvspace: qsize_t,
    pub dqb_ihardlimit: qsize_t,
    pub dqb_isoftlimit: qsize_t,
    pub dqb_curinodes: qsize_t,
    pub dqb_btime: time64_t,
    pub dqb_itime: time64_t,
}
pub type qsize_t = ::core::ffi::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mem_dqinfo {
    pub dqi_format: *mut quota_format_type,
    pub dqi_fmt_id: ::core::ffi::c_int,
    pub dqi_dirty_list: list_head,
    pub dqi_flags: ::core::ffi::c_ulong,
    pub dqi_bgrace: ::core::ffi::c_uint,
    pub dqi_igrace: ::core::ffi::c_uint,
    pub dqi_max_spc_limit: qsize_t,
    pub dqi_max_ino_limit: qsize_t,
    pub dqi_priv: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quota_format_type {
    pub qf_fmt_id: ::core::ffi::c_int,
    pub qf_ops: *const quota_format_ops,
    pub qf_owner: *mut module,
    pub qf_next: *mut quota_format_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtd_info {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct backing_dev_info {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct block_device {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mount {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xattr_handler {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct export_operations {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quotactl_ops {
    pub quota_on: Option<
        unsafe extern "C" fn(
            *mut super_block,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const path,
        ) -> ::core::ffi::c_int,
    >,
    pub quota_off:
        Option<unsafe extern "C" fn(*mut super_block, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub quota_enable:
        Option<unsafe extern "C" fn(*mut super_block, ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub quota_disable:
        Option<unsafe extern "C" fn(*mut super_block, ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub quota_sync:
        Option<unsafe extern "C" fn(*mut super_block, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub set_info: Option<
        unsafe extern "C" fn(
            *mut super_block,
            ::core::ffi::c_int,
            *mut qc_info,
        ) -> ::core::ffi::c_int,
    >,
    pub get_dqblk:
        Option<unsafe extern "C" fn(*mut super_block, kqid, *mut qc_dqblk) -> ::core::ffi::c_int>,
    pub get_nextdqblk: Option<
        unsafe extern "C" fn(*mut super_block, *mut kqid, *mut qc_dqblk) -> ::core::ffi::c_int,
    >,
    pub set_dqblk:
        Option<unsafe extern "C" fn(*mut super_block, kqid, *mut qc_dqblk) -> ::core::ffi::c_int>,
    pub get_state:
        Option<unsafe extern "C" fn(*mut super_block, *mut qc_state) -> ::core::ffi::c_int>,
    pub rm_xquota:
        Option<unsafe extern "C" fn(*mut super_block, ::core::ffi::c_uint) -> ::core::ffi::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qc_state {
    pub s_incoredqs: ::core::ffi::c_uint,
    pub s_state: [qc_type_state; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qc_type_state {
    pub flags: ::core::ffi::c_uint,
    pub spc_timelimit: ::core::ffi::c_uint,
    pub ino_timelimit: ::core::ffi::c_uint,
    pub rt_spc_timelimit: ::core::ffi::c_uint,
    pub spc_warnlimit: ::core::ffi::c_uint,
    pub ino_warnlimit: ::core::ffi::c_uint,
    pub rt_spc_warnlimit: ::core::ffi::c_uint,
    pub ino: ::core::ffi::c_ulonglong,
    pub blocks: blkcnt_t,
    pub nextents: blkcnt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qc_dqblk {
    pub d_fieldmask: ::core::ffi::c_int,
    pub d_spc_hardlimit: u64_0,
    pub d_spc_softlimit: u64_0,
    pub d_ino_hardlimit: u64_0,
    pub d_ino_softlimit: u64_0,
    pub d_space: u64_0,
    pub d_ino_count: u64_0,
    pub d_ino_timer: s64,
    pub d_spc_timer: s64,
    pub d_ino_warns: ::core::ffi::c_int,
    pub d_spc_warns: ::core::ffi::c_int,
    pub d_rt_spc_hardlimit: u64_0,
    pub d_rt_spc_softlimit: u64_0,
    pub d_rt_space: u64_0,
    pub d_rt_spc_timer: s64,
    pub d_rt_spc_warns: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qc_info {
    pub i_fieldmask: ::core::ffi::c_int,
    pub i_flags: ::core::ffi::c_uint,
    pub i_spc_timelimit: ::core::ffi::c_uint,
    pub i_ino_timelimit: ::core::ffi::c_uint,
    pub i_rt_spc_timelimit: ::core::ffi::c_uint,
    pub i_spc_warnlimit: ::core::ffi::c_uint,
    pub i_ino_warnlimit: ::core::ffi::c_uint,
    pub i_rt_spc_warnlimit: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dquot_operations {
    pub write_dquot: Option<unsafe extern "C" fn(*mut dquot) -> ::core::ffi::c_int>,
    pub alloc_dquot:
        Option<unsafe extern "C" fn(*mut super_block, ::core::ffi::c_int) -> *mut dquot>,
    pub destroy_dquot: Option<unsafe extern "C" fn(*mut dquot) -> ()>,
    pub acquire_dquot: Option<unsafe extern "C" fn(*mut dquot) -> ::core::ffi::c_int>,
    pub release_dquot: Option<unsafe extern "C" fn(*mut dquot) -> ::core::ffi::c_int>,
    pub mark_dirty: Option<unsafe extern "C" fn(*mut dquot) -> ::core::ffi::c_int>,
    pub write_info:
        Option<unsafe extern "C" fn(*mut super_block, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub get_reserved_space: Option<unsafe extern "C" fn(*mut inode) -> *mut qsize_t>,
    pub get_projid: Option<unsafe extern "C" fn(*mut inode, *mut kprojid_t) -> ::core::ffi::c_int>,
    pub get_inode_usage:
        Option<unsafe extern "C" fn(*mut inode, *mut qsize_t) -> ::core::ffi::c_int>,
    pub get_next_id:
        Option<unsafe extern "C" fn(*mut super_block, *mut kqid) -> ::core::ffi::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct super_operations {
    pub alloc_inode: Option<unsafe extern "C" fn(*mut super_block) -> *mut inode>,
    pub destroy_inode: Option<unsafe extern "C" fn(*mut inode) -> ()>,
    pub free_inode: Option<unsafe extern "C" fn(*mut inode) -> ()>,
    pub dirty_inode: Option<unsafe extern "C" fn(*mut inode, ::core::ffi::c_int) -> ()>,
    pub write_inode:
        Option<unsafe extern "C" fn(*mut inode, *mut writeback_control) -> ::core::ffi::c_int>,
    pub drop_inode: Option<unsafe extern "C" fn(*mut inode) -> ::core::ffi::c_int>,
    pub evict_inode: Option<unsafe extern "C" fn(*mut inode) -> ()>,
    pub put_super: Option<unsafe extern "C" fn(*mut super_block) -> ()>,
    pub sync_fs:
        Option<unsafe extern "C" fn(*mut super_block, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub freeze_super: Option<
        unsafe extern "C" fn(
            *mut super_block,
            freeze_holder,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub freeze_fs: Option<unsafe extern "C" fn(*mut super_block) -> ::core::ffi::c_int>,
    pub thaw_super: Option<
        unsafe extern "C" fn(
            *mut super_block,
            freeze_holder,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub unfreeze_fs: Option<unsafe extern "C" fn(*mut super_block) -> ::core::ffi::c_int>,
    pub statfs: Option<unsafe extern "C" fn(*mut dentry, *mut kstatfs) -> ::core::ffi::c_int>,
    pub umount_begin: Option<unsafe extern "C" fn(*mut super_block) -> ()>,
    pub show_options:
        Option<unsafe extern "C" fn(*mut seq_file, *mut dentry) -> ::core::ffi::c_int>,
    pub show_devname:
        Option<unsafe extern "C" fn(*mut seq_file, *mut dentry) -> ::core::ffi::c_int>,
    pub show_path: Option<unsafe extern "C" fn(*mut seq_file, *mut dentry) -> ::core::ffi::c_int>,
    pub show_stats: Option<unsafe extern "C" fn(*mut seq_file, *mut dentry) -> ::core::ffi::c_int>,
    pub nr_cached_objects:
        Option<unsafe extern "C" fn(*mut super_block, *mut shrink_control) -> ::core::ffi::c_long>,
    pub free_cached_objects:
        Option<unsafe extern "C" fn(*mut super_block, *mut shrink_control) -> ::core::ffi::c_long>,
    pub remove_bdev:
        Option<unsafe extern "C" fn(*mut super_block, *mut block_device) -> ::core::ffi::c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut super_block) -> ()>,
    pub report_error: Option<unsafe extern "C" fn(*const fserror_event) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fserror_event {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kstatfs {
    _private: [u8; 0],
}
pub type freeze_holder = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct writeback_control {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_system_type {
    pub name: *const ::core::ffi::c_char,
    pub fs_flags: ::core::ffi::c_int,
    pub init_fs_context: Option<unsafe extern "C" fn(*mut fs_context) -> ::core::ffi::c_int>,
    pub parameters: *const fs_parameter_spec,
    pub kill_sb: Option<unsafe extern "C" fn(*mut super_block) -> ()>,
    pub owner: *mut module,
    pub list: hlist_node,
    pub fs_supers: hlist_head,
    pub s_lock_key: lock_class_key,
    pub s_umount_key: lock_class_key,
    pub s_vfs_rename_key: lock_class_key,
    pub s_writers_key: [lock_class_key; 3],
    pub i_lock_key: lock_class_key,
    pub i_mutex_key: lock_class_key,
    pub invalidate_lock_key: lock_class_key,
    pub i_mutex_dir_key: lock_class_key,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lock_class_key {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fs_parameter_spec {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fs_context {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union shortname_store {
    pub string: [::core::ffi::c_uchar; 40],
    pub words: [::core::ffi::c_ulong; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_38 {
    pub __d_name: qstr,
    pub d_name: qstr,
}
pub type seqcount_spinlock_t = seqcount_spinlock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqcount_spinlock {
    pub seqcount: seqcount_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fown_struct {
    pub file: *mut file,
    pub lock: rwlock_t,
    pub pid: *mut pid,
    pub pid_type: pid_type,
    pub uid: kuid_t,
    pub euid: kuid_t,
    pub signum: ::core::ffi::c_int,
}
pub type pid_type = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pid {
    pub count: refcount_t,
    pub level: ::core::ffi::c_uint,
    pub lock: spinlock_t,
    pub c2rust_unnamed: C2Rust_Unnamed_39,
    pub tasks: [hlist_head; 4],
    pub inodes: hlist_head,
    pub wait_pidfd: wait_queue_head_t,
    pub rcu: callback_head,
    pub numbers: [upid; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct upid {
    pub nr: ::core::ffi::c_int,
    pub ns: *mut pid_namespace,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pid_namespace {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_39 {
    pub ino: u64_0,
    pub pidfs_hash: rhash_head,
    pub stashed: *mut dentry,
    pub attr: *mut pidfs_attr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pidfs_attr {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rhash_head {
    pub next: *mut rhash_head,
}
pub type rwlock_t = rwlock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rwlock {
    pub raw_lock: arch_rwlock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arch_rwlock_t {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cred {
    pub usage: atomic_long_t,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub suid: kuid_t,
    pub sgid: kgid_t,
    pub euid: kuid_t,
    pub egid: kgid_t,
    pub fsuid: kuid_t,
    pub fsgid: kgid_t,
    pub securebits: ::core::ffi::c_uint,
    pub cap_inheritable: kernel_cap_t,
    pub cap_permitted: kernel_cap_t,
    pub cap_effective: kernel_cap_t,
    pub cap_bset: kernel_cap_t,
    pub cap_ambient: kernel_cap_t,
    pub user: *mut user_struct,
    pub user_ns: *mut user_namespace,
    pub ucounts: *mut ucounts,
    pub group_info: *mut group_info,
    pub c2rust_unnamed: C2Rust_Unnamed_40,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_40 {
    pub non_rcu: ::core::ffi::c_int,
    pub rcu: callback_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group_info {
    pub usage: refcount_t,
    pub ngroups: ::core::ffi::c_int,
    pub gid: [kgid_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucounts {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct user_struct {
    pub __count: refcount_t,
    pub unix_inflight: ::core::ffi::c_ulong,
    pub pipe_bufs: atomic_long_t,
    pub uidhash_node: hlist_node,
    pub uid: kuid_t,
    pub locked_vm: atomic_long_t,
    pub ratelimit: ratelimit_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ratelimit_state {
    pub lock: raw_spinlock_t,
    pub interval: ::core::ffi::c_int,
    pub burst: ::core::ffi::c_int,
    pub rs_n_left: atomic_t,
    pub missed: atomic_t,
    pub flags: ::core::ffi::c_uint,
    pub begin: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kernel_cap_t {
    pub val: u64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swap_info_struct {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct folio {
    pub c2rust_unnamed: C2Rust_Unnamed_50,
    pub c2rust_unnamed_0: C2Rust_Unnamed_45,
    pub c2rust_unnamed_1: C2Rust_Unnamed_43,
    pub c2rust_unnamed_2: C2Rust_Unnamed_41,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_41 {
    pub c2rust_unnamed: C2Rust_Unnamed_42,
    pub __page_3: page,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_42 {
    pub _flags_3: ::core::ffi::c_ulong,
    pub _head_3: ::core::ffi::c_ulong,
    pub _hugetlb_subpool: *mut ::core::ffi::c_void,
    pub _hugetlb_cgroup: *mut ::core::ffi::c_void,
    pub _hugetlb_cgroup_rsvd: *mut ::core::ffi::c_void,
    pub _hugetlb_hwpoison: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_43 {
    pub c2rust_unnamed: C2Rust_Unnamed_44,
    pub __page_2: page,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_44 {
    pub _flags_2: ::core::ffi::c_ulong,
    pub _head_2: ::core::ffi::c_ulong,
    pub _deferred_list: list_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_45 {
    pub c2rust_unnamed: C2Rust_Unnamed_46,
    pub __page_1: page,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_46 {
    pub _flags_1: ::core::ffi::c_ulong,
    pub _head_1: ::core::ffi::c_ulong,
    pub c2rust_unnamed: C2Rust_Unnamed_47,
    pub _mapcount_1: atomic_t,
    pub _refcount_1: atomic_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_47 {
    pub c2rust_unnamed: C2Rust_Unnamed_48,
    pub _usable_1: [::core::ffi::c_ulong; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_48 {
    pub _large_mapcount: atomic_t,
    pub _nr_pages_mapped: atomic_t,
    pub _entire_mapcount: atomic_t,
    pub _pincount: atomic_t,
    pub _mm_id_mapcount: [mm_id_mapcount_t; 2],
    pub c2rust_unnamed: C2Rust_Unnamed_49,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_49 {
    pub _mm_id: [mm_id_t; 2],
    pub _mm_ids: ::core::ffi::c_ulong,
}
pub type mm_id_t = ::core::ffi::c_uint;
pub type mm_id_mapcount_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_50 {
    pub c2rust_unnamed: C2Rust_Unnamed_51,
    pub page: page,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_51 {
    pub flags: memdesc_flags_t,
    pub c2rust_unnamed: C2Rust_Unnamed_54,
    pub mapping: *mut address_space,
    pub c2rust_unnamed_0: C2Rust_Unnamed_53,
    pub c2rust_unnamed_1: C2Rust_Unnamed_52,
    pub _mapcount: atomic_t,
    pub _refcount: atomic_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_52 {
    pub private: *mut ::core::ffi::c_void,
    pub swap: swp_entry_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swp_entry_t {
    pub val: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_53 {
    pub index: ::core::ffi::c_ulong,
    pub share: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_54 {
    pub lru: list_head,
    pub c2rust_unnamed: C2Rust_Unnamed_58,
    pub pgmap: *mut dev_pagemap,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dev_pagemap {
    pub altmap: vmem_altmap,
    pub r#ref: percpu_ref,
    pub done: completion,
    pub r#type: memory_type,
    pub flags: ::core::ffi::c_uint,
    pub vmemmap_shift: ::core::ffi::c_ulong,
    pub ops: *const dev_pagemap_ops,
    pub owner: *mut ::core::ffi::c_void,
    pub nr_range: ::core::ffi::c_int,
    pub c2rust_unnamed: C2Rust_Unnamed_55,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_55 {
    pub range: range,
    pub c2rust_unnamed: C2Rust_Unnamed_56,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_56 {
    pub __empty_ranges: C2Rust_Unnamed_57,
    pub ranges: [range; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct range {
    pub start: u64_0,
    pub end: u64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_57 {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dev_pagemap_ops {
    pub folio_free: Option<unsafe extern "C" fn(*mut folio) -> ()>,
    pub migrate_to_ram: Option<unsafe extern "C" fn(*mut vm_fault) -> vm_fault_t>,
    pub memory_failure: Option<
        unsafe extern "C" fn(
            *mut dev_pagemap,
            ::core::ffi::c_ulong,
            ::core::ffi::c_ulong,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub folio_split: Option<unsafe extern "C" fn(*mut folio, *mut folio) -> ()>,
}
pub type memory_type = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct percpu_ref {
    pub percpu_count_ptr: ::core::ffi::c_ulong,
    pub data: *mut percpu_ref_data,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vmem_altmap {
    pub base_pfn: ::core::ffi::c_ulong,
    pub end_pfn: ::core::ffi::c_ulong,
    pub reserve: ::core::ffi::c_ulong,
    pub free: ::core::ffi::c_ulong,
    pub align: ::core::ffi::c_ulong,
    pub alloc: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_58 {
    pub __filler: *mut ::core::ffi::c_void,
    pub mlock_count: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct memdesc_flags_t {
    pub f: ::core::ffi::c_ulong,
}
pub type migrate_mode = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct readahead_control {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rb_root_cached {
    pub rb_root: rb_root,
    pub rb_leftmost: *mut rb_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xarray {
    pub xa_lock: spinlock_t,
    pub xa_flags: gfp_t,
    pub xa_head: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_59 {
    pub lru: list_head,
    pub buddy_list: list_head,
    pub pcp_list: list_head,
    pub pcp_llist: llist_node,
}
pub type C2Rust_Unnamed_80 = ::core::ffi::c_uint;
pub const ___GFP_THISNODE_BIT: C2Rust_Unnamed_80 = 21;
pub const ___GFP_NOWARN_BIT: C2Rust_Unnamed_80 = 13;
pub const ___GFP_KSWAPD_RECLAIM_BIT: C2Rust_Unnamed_80 = 11;
pub const ___GFP_DIRECT_RECLAIM_BIT: C2Rust_Unnamed_80 = 10;
pub const ___GFP_FS_BIT: C2Rust_Unnamed_80 = 7;
pub const ___GFP_IO_BIT: C2Rust_Unnamed_80 = 6;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kernel_mapping {
    pub virt_addr: ::core::ffi::c_ulong,
    pub virt_offset: ::core::ffi::c_ulong,
    pub phys_addr: uintptr_t,
    pub size: uintptr_t,
    pub va_pa_offset: ::core::ffi::c_ulong,
    pub page_offset: ::core::ffi::c_ulong,
    pub va_kernel_pa_offset: ::core::ffi::c_ulong,
}
pub type node_states = ::core::ffi::c_uint;
pub const N_ONLINE: node_states = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timens_offset {
    pub sec: s64,
    pub nsec: u64_0,
}
pub type vm_fault_reason = ::core::ffi::c_uint;
pub const VM_FAULT_FALLBACK: vm_fault_reason = 2048;
pub const VM_FAULT_NOPAGE: vm_fault_reason = 256;
pub const VM_FAULT_SIGSEGV: vm_fault_reason = 64;
pub const VM_FAULT_HWPOISON_LARGE: vm_fault_reason = 32;
pub const VM_FAULT_HWPOISON: vm_fault_reason = 16;
pub const VM_FAULT_SIGBUS: vm_fault_reason = 2;
pub const VM_FAULT_OOM: vm_fault_reason = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vm_special_mapping {
    pub name: *const ::core::ffi::c_char,
    pub pages: *mut *mut page,
    pub fault: Option<
        unsafe extern "C" fn(
            *const vm_special_mapping,
            *mut vm_area_struct,
            *mut vm_fault,
        ) -> vm_fault_t,
    >,
    pub mremap: Option<
        unsafe extern "C" fn(*const vm_special_mapping, *mut vm_area_struct) -> ::core::ffi::c_int,
    >,
    pub close: Option<unsafe extern "C" fn(*const vm_special_mapping, *mut vm_area_struct) -> ()>,
}
// [bisect test] Standard kernel-crate idiom (see e.g. rust/kernel/device.rs's
// `unsafe impl Sync for Device {}`) for asserting a raw-pointer-containing
// type is safe to place in an immutable static: the C original is `const`,
// read/written only by the kernel's own VMA fault-handling machinery,
// never mutated by this TU or anything else after link time.
unsafe impl Sync for vm_special_mapping {}
pub type pagetype = ::core::ffi::c_uint;
pub const PGTY_large_kmalloc: pagetype = 248;
pub const PGTY_slab: pagetype = 245;
pub type C2Rust_Unnamed_82 = ::core::ffi::c_uint;
pub const VMA_MIXEDMAP_BIT: C2Rust_Unnamed_82 = 28;
pub const VMA_DONTDUMP_BIT: C2Rust_Unnamed_82 = 26;
pub const VMA_IO_BIT: C2Rust_Unnamed_82 = 14;
pub const VMA_MAYREAD_BIT: C2Rust_Unnamed_82 = 4;
pub const VMA_READ_BIT: C2Rust_Unnamed_82 = 0;
pub const VDSO_ARCH_PAGES_START: vdso_pages = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdso_arch_data {
    pub all_cpu_hwprobe_values: [__u64; 17],
    pub homogeneous_cpus: __u8,
    pub ready: __u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdso_rng_data {
    pub generation: u64_0,
    pub is_ready: u8_0,
}
pub const VDSO_RNG_PAGE_OFFSET: vdso_pages = 2;
#[derive(Copy, Clone)]
#[repr(C, align(64))]
pub struct vdso_time_data(pub C2Rust_vdso_time_data_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_vdso_time_data_Inner {
    pub arch_data: arch_vdso_time_data,
    pub clock_data: [vdso_clock; 2],
    pub aux_clock_data: [vdso_clock; 8],
    pub tz_minuteswest: s32,
    pub tz_dsttime: s32,
    pub hrtimer_res: u32_0,
    pub __unused: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdso_clock {
    pub seq: u32_0,
    pub clock_mode: s32,
    pub cycle_last: u64_0,
    pub mask: u64_0,
    pub mult: u32_0,
    pub shift: u32_0,
    pub c2rust_unnamed: C2Rust_Unnamed_83,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_83 {
    pub basetime: [vdso_timestamp; 12],
    pub offset: [timens_offset; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdso_timestamp {
    pub sec: u64_0,
    pub nsec: u64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arch_vdso_time_data {}
pub const VDSO_TIME_PAGE_OFFSET: vdso_pages = 0;
pub const VDSO_TIMENS_PAGE_OFFSET: vdso_pages = 1;
pub const VDSO_NR_PAGES: vdso_pages = 4;
pub type vdso_pages = ::core::ffi::c_uint;
pub const BITS_PER_LONG: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn __kcsan_enable_current() {}
#[inline]
unsafe extern "C" fn __kcsan_disable_current() {}
pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const EBUSY: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
// [bisect] riscv_current_is_tp / current_stack_pointer statics removed:
// c2rust mistranslated arch/riscv/include/asm/current.h's GCC register-variable
// pseudo-globals (`register struct task_struct *riscv_current_is_tp __asm__("tp")`
// and `register unsigned long current_stack_pointer __asm__("sp")`) into ordinary
// #[no_mangle] pub static mut globals with real .bss storage -- these never exist
// as linkable symbols in the C build (register-asm vars compile to direct register
// reads, no symbol at all). Confirmed unreferenced anywhere else in this TU.
pub const RISCV_ISA_EXT_ZBB: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
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
unsafe extern "C" fn generic_fls(mut x: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    unsafe {
        return (u32::BITS - x.leading_zeros()) as ::core::ffi::c_int;
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
pub const PAGE_SHIFT: ::core::ffi::c_int = CONFIG_PAGE_SHIFT;
pub const PAGE_SIZE: ::core::ffi::c_ulong = (1 as ::core::ffi::c_ulong) << CONFIG_PAGE_SHIFT;
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
#[inline(always)]
unsafe extern "C" fn is_power_of_2(mut n: ::core::ffi::c_ulong) -> bool_0 {
    unsafe {
        return n.wrapping_sub(1 as ::core::ffi::c_ulong)
            < n ^ n.wrapping_sub(1 as ::core::ffi::c_ulong);
    }
}
#[inline(always)]
unsafe extern "C" fn get_order(mut size: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    unsafe {
        if false {
            if size == 0 {
                return BITS_PER_LONG - PAGE_SHIFT;
            }
            if size < (1 as ::core::ffi::c_ulong) << PAGE_SHIFT {
                return 0 as ::core::ffi::c_int;
            }
            return (if false {
                (if size.wrapping_sub(1 as ::core::ffi::c_ulong) < 2 as ::core::ffi::c_ulong {
                    0 as ::core::ffi::c_int
                } else {
                    63 as ::core::ffi::c_int
                        - (size.wrapping_sub(1 as ::core::ffi::c_ulong) as ::core::ffi::c_ulonglong)
                            .leading_zeros() as ::core::ffi::c_int
                })
            } else {
                (if ::core::mem::size_of::<::core::ffi::c_ulong>() <= 4 as usize {
                    __ilog2_u32(size.wrapping_sub(1 as ::core::ffi::c_ulong) as u32_0)
                } else {
                    __ilog2_u64(size.wrapping_sub(1 as ::core::ffi::c_ulong) as u64_0)
                })
            }) - PAGE_SHIFT
                + 1 as ::core::ffi::c_int;
        }
        size = size.wrapping_sub(1);
        size >>= PAGE_SHIFT;
        return fls64(size as __u64);
    }
}
#[inline(always)]
unsafe extern "C" fn arch_atomic_add(mut i: ::core::ffi::c_int, mut v: *mut atomic_t) {
    unsafe {
        asm!(
            "\tamoadd.w zero, {1}, 0({0})\n", in (reg) & raw mut (* v).counter,
            inlateout(reg) i => _, options(preserves_flags)
        );
    }
}
#[inline(always)]
unsafe extern "C" fn raw_atomic_add(mut i: ::core::ffi::c_int, mut v: *mut atomic_t) {
    unsafe {
        arch_atomic_add(i, v);
    }
}
#[inline(always)]
unsafe extern "C" fn raw_atomic_inc(mut v: *mut atomic_t) {
    unsafe {
        raw_atomic_add(1 as ::core::ffi::c_int, v);
    }
}
#[inline(always)]
unsafe extern "C" fn atomic_inc(mut v: *mut atomic_t) {
    unsafe {
        instrument_atomic_read_write(
            v as *const ::core::ffi::c_void,
            ::core::mem::size_of::<atomic_t>() as size_t,
        );
        raw_atomic_inc(v);
    }
}
pub const NUMA_NO_NODE: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn node_state(
    mut node: ::core::ffi::c_int,
    mut state: node_states,
) -> ::core::ffi::c_int {
    unsafe {
        return (node == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn mem_alloc_profiling_enabled() -> bool_0 {
    unsafe {
        return r#false as ::core::ffi::c_int != 0;
    }
}
#[inline(always)]
unsafe extern "C" fn compound_info_has_mask() -> bool_0 {
    unsafe {
        if true {
            return r#false as ::core::ffi::c_int != 0;
        }
        return is_power_of_2(::core::mem::size_of::<page>() as ::core::ffi::c_ulong);
    }
}
#[inline(always)]
unsafe extern "C" fn _compound_head(mut page: *const page) -> ::core::ffi::c_ulong {
    unsafe {
        let mut info: ::core::ffi::c_ulong = ({
            extern "C" {
                #[link_name = "__compiletime_assert_123"]
                fn __compiletime_assert_123_0() -> !;
            }
            if !(::core::mem::size_of::<::core::ffi::c_ulong>()
                == ::core::mem::size_of::<::core::ffi::c_char>()
                || ::core::mem::size_of::<::core::ffi::c_ulong>()
                    == ::core::mem::size_of::<::core::ffi::c_short>()
                || ::core::mem::size_of::<::core::ffi::c_ulong>()
                    == ::core::mem::size_of::<::core::ffi::c_int>()
                || ::core::mem::size_of::<::core::ffi::c_ulong>()
                    == ::core::mem::size_of::<::core::ffi::c_long>()
                || ::core::mem::size_of::<::core::ffi::c_ulong>()
                    == ::core::mem::size_of::<::core::ffi::c_longlong>())
            {
                __compiletime_assert_123_0();
            }
            ::core::ptr::read_volatile::<::core::ffi::c_ulong>(
                (&raw const (*page).c2rust_unnamed.c2rust_unnamed_1.compound_info
                    as *const ::core::ffi::c_ulong),
            )
        });
        let mut mask: ::core::ffi::c_ulong = 0;
        if !compound_info_has_mask() {
            if info & 1 as ::core::ffi::c_ulong != 0 {
                return info.wrapping_sub(1 as ::core::ffi::c_ulong);
            }
            return page.expose_provenance() as ::core::ffi::c_ulong;
        }
        mask = (info & 1 as ::core::ffi::c_ulong).wrapping_sub(1 as ::core::ffi::c_ulong);
        mask |= info;
        return page.expose_provenance() as ::core::ffi::c_ulong & mask;
    }
}
#[inline(always)]
unsafe extern "C" fn folio_test_slab(mut folio: *const folio) -> bool_0 {
    unsafe {
        return ({
            __kcsan_disable_current();
            let mut __v: ::core::ffi::c_uint =
                (*folio).c2rust_unnamed.page.c2rust_unnamed_0.page_type as ::core::ffi::c_uint
                    >> 24 as ::core::ffi::c_int;
            __kcsan_enable_current();
            __v
        }) == PGTY_slab as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
}
#[inline(always)]
unsafe extern "C" fn folio_test_large_kmalloc(mut folio: *const folio) -> bool_0 {
    unsafe {
        return ({
            __kcsan_disable_current();
            let mut __v: ::core::ffi::c_uint =
                (*folio).c2rust_unnamed.page.c2rust_unnamed_0.page_type as ::core::ffi::c_uint
                    >> 24 as ::core::ffi::c_int;
            __kcsan_enable_current();
            __v
        }) == PGTY_large_kmalloc as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
}
#[inline]
unsafe extern "C" fn numa_node_id() -> ::core::ffi::c_int {
    unsafe {
        return 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn numa_mem_id() -> ::core::ffi::c_int {
    unsafe {
        return numa_node_id();
    }
}
#[inline]
unsafe extern "C" fn warn_if_node_offline(mut this_node: ::core::ffi::c_int, mut gfp_mask: gfp_t) {
    unsafe {
        let mut warn_gfp: gfp_t = gfp_mask
            & (((1 as ::core::ffi::c_ulong) << ___GFP_THISNODE_BIT as ::core::ffi::c_int) as gfp_t
                | ((1 as ::core::ffi::c_ulong) << ___GFP_NOWARN_BIT as ::core::ffi::c_int)
                    as gfp_t);
        if warn_gfp
            != ((1 as ::core::ffi::c_ulong) << ___GFP_THISNODE_BIT as ::core::ffi::c_int) as gfp_t
                | ((1 as ::core::ffi::c_ulong) << ___GFP_NOWARN_BIT as ::core::ffi::c_int) as gfp_t
        {
            return;
        }
        if node_state(this_node, N_ONLINE) != 0 {
            return;
        }
        ({
            _printk(
                b"\x014%pGg allocation from offline node %d\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut gfp_mask,
                this_node,
            );
        });
        dump_stack();
    }
}
#[inline]
unsafe extern "C" fn __alloc_pages_node_noprof(
    mut nid: ::core::ffi::c_int,
    mut gfp_mask: gfp_t,
    mut order: ::core::ffi::c_uint,
) -> *mut page {
    unsafe {
        warn_if_node_offline(nid, gfp_mask);
        return __alloc_pages_noprof(gfp_mask, order, nid, ::core::ptr::null_mut::<nodemask_t>());
    }
}
#[inline]
unsafe extern "C" fn alloc_pages_node_noprof(
    mut nid: ::core::ffi::c_int,
    mut gfp_mask: gfp_t,
    mut order: ::core::ffi::c_uint,
) -> *mut page {
    unsafe {
        if nid == NUMA_NO_NODE {
            nid = numa_mem_id();
        }
        return __alloc_pages_node_noprof(nid, gfp_mask, order);
    }
}
#[inline]
unsafe extern "C" fn alloc_pages_noprof(
    mut gfp_mask: gfp_t,
    mut order: ::core::ffi::c_uint,
) -> *mut page {
    unsafe {
        return alloc_pages_node_noprof(numa_node_id(), gfp_mask, order);
    }
}
#[inline]
unsafe extern "C" fn __page_ref_mod(mut page: *mut page, mut v: ::core::ffi::c_int) {}
#[inline]
unsafe extern "C" fn page_ref_inc(mut page: *mut page) {
    unsafe {
        atomic_inc(&raw mut (*page)._refcount);
        if r#false as ::core::ffi::c_int != 0 {
            __page_ref_mod(page, 1 as ::core::ffi::c_int);
        }
    }
}
#[inline]
unsafe extern "C" fn folio_ref_inc(mut folio: *mut folio) {
    unsafe {
        page_ref_inc(&raw mut (*folio).c2rust_unnamed.page);
    }
}
pub const VM_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const VM_SEALED_SYSMAP: ::core::ffi::c_int = VM_NONE;
#[inline]
unsafe extern "C" fn folio_get(mut folio: *mut folio) {
    unsafe {
        folio_ref_inc(folio);
    }
}
#[inline]
unsafe extern "C" fn get_page(mut page: *mut page) {
    unsafe {
        let mut folio: *mut folio = ::core::ptr::with_exposed_provenance_mut::<folio>(
            _compound_head(page as *const page) as usize,
        );
        if kernel::warn_on!(folio_test_slab(folio)) {
            return;
        }
        if kernel::warn_on!(folio_test_large_kmalloc(folio)) {
            return;
        }
        folio_get(folio);
    }
}
#[inline(always)]
unsafe extern "C" fn lowmem_page_address(mut page: *const page) -> *mut ::core::ffi::c_void {
    unsafe {
        return ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
            ((((page.offset_from(mem_map) as ::core::ffi::c_ulong)
                .wrapping_add(phys_ram_base as ::core::ffi::c_ulong >> 12 as ::core::ffi::c_int)
                as phys_addr_t)
                << 12 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
                .wrapping_add(kernel_map.va_pa_offset) as usize,
        );
    }
}
#[inline]
unsafe extern "C" fn vmf_insert_page(
    mut vma: *mut vm_area_struct,
    mut addr: ::core::ffi::c_ulong,
    mut page: *mut page,
) -> vm_fault_t {
    unsafe {
        let mut err: ::core::ffi::c_int = vm_insert_page(vma, addr, page);
        if err == -ENOMEM {
            return VM_FAULT_OOM as ::core::ffi::c_int as vm_fault_t;
        }
        if err < 0 as ::core::ffi::c_int && err != -EBUSY {
            return VM_FAULT_SIGBUS as ::core::ffi::c_int as vm_fault_t;
        }
        return VM_FAULT_NOPAGE as ::core::ffi::c_int as vm_fault_t;
    }
}
#[inline]
unsafe extern "C" fn find_timens_vvar_page(mut vma: *mut vm_area_struct) -> *mut page {
    unsafe {
        return ::core::ptr::null_mut::<page>();
    }
}
// [bisect] page-alignment candidate fix: the C original is
// `__aligned(PAGE_SIZE)` (arch/riscv PAGE_SIZE=4096); c2rust dropped this
// GNU attribute entirely, leaving the translated static at the section's
// default byte alignment. Confirmed via readelf/System.map: without this
// wrapper, vdso_initdata landed at 0xffffffff804090f8 (not page-aligned).
// Rust statics can't take #[align] directly, hence the wrapper struct.
#[repr(C, align(4096))]
struct VdsoInitdataAligned([u8_0; 16384]);
#[link_section = ".init.data"]
static mut vdso_initdata_wrapped: VdsoInitdataAligned = VdsoInitdataAligned([0; 16384]);
#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut vdso_k_time_data: *mut vdso_time_data = ::core::ptr::null_mut::<vdso_time_data>();
#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut vdso_k_rng_data: *mut vdso_rng_data = ::core::ptr::null_mut::<vdso_rng_data>();
#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut vdso_k_arch_data: *mut vdso_arch_data = ::core::ptr::null_mut::<vdso_arch_data>();
#[no_mangle]
#[link_section = ".init.text"]
#[cold]
pub unsafe extern "C" fn vdso_setup_data_pages() {
    unsafe {
        let mut order: ::core::ffi::c_uint = get_order(
            (VDSO_NR_PAGES as ::core::ffi::c_int as ::core::ffi::c_ulong).wrapping_mul(PAGE_SIZE),
        ) as ::core::ffi::c_uint;
        let mut pages: *mut page = ::core::ptr::null_mut::<page>();
        pages = ({
            ({
                let mut _res: *mut page = ::core::ptr::null_mut::<page>();
                if mem_alloc_profiling_enabled() {
                    let mut _old: *mut alloc_tag = ::core::ptr::null_mut::<alloc_tag>();
                    _old = ::core::ptr::null_mut::<alloc_tag>();
                    _res = alloc_pages_noprof(
                        ((1 as ::core::ffi::c_ulong)
                            << ___GFP_DIRECT_RECLAIM_BIT as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_ulong)
                                << ___GFP_KSWAPD_RECLAIM_BIT as ::core::ffi::c_int)
                            as gfp_t
                            | ((1 as ::core::ffi::c_ulong) << ___GFP_IO_BIT as ::core::ffi::c_int)
                                as gfp_t
                            | ((1 as ::core::ffi::c_ulong) << ___GFP_FS_BIT as ::core::ffi::c_int)
                                as gfp_t,
                        order,
                    ) as *mut page;
                } else {
                    _res = alloc_pages_noprof(
                        ((1 as ::core::ffi::c_ulong)
                            << ___GFP_DIRECT_RECLAIM_BIT as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_ulong)
                                << ___GFP_KSWAPD_RECLAIM_BIT as ::core::ffi::c_int)
                            as gfp_t
                            | ((1 as ::core::ffi::c_ulong) << ___GFP_IO_BIT as ::core::ffi::c_int)
                                as gfp_t
                            | ((1 as ::core::ffi::c_ulong) << ___GFP_FS_BIT as ::core::ffi::c_int)
                                as gfp_t,
                        order,
                    ) as *mut page;
                }
                _res
            })
        }) as *mut page;
        if pages.is_null() {
            panic(b"Unable to allocate VDSO storage pages\0".as_ptr() as *const ::core::ffi::c_char);
        }
        split_page(pages, order);
        memcpy(
            lowmem_page_address(pages),
            &raw mut vdso_initdata_wrapped.0 as *mut u8_0 as *const ::core::ffi::c_void,
            (VDSO_NR_PAGES as ::core::ffi::c_int as size_t).wrapping_mul(PAGE_SIZE),
        );
        vdso_k_time_data =
            lowmem_page_address(pages.offset(VDSO_TIME_PAGE_OFFSET as ::core::ffi::c_int as isize))
                as *mut vdso_time_data;
        vdso_k_rng_data =
            lowmem_page_address(pages.offset(VDSO_RNG_PAGE_OFFSET as ::core::ffi::c_int as isize))
                as *mut vdso_rng_data;
        vdso_k_arch_data =
            lowmem_page_address(pages.offset(VDSO_ARCH_PAGES_START as ::core::ffi::c_int as isize))
                as *mut vdso_arch_data;
    }
}
unsafe extern "C" fn vvar_fault(
    mut sm: *const vm_special_mapping,
    mut vma: *mut vm_area_struct,
    mut vmf: *mut vm_fault,
) -> vm_fault_t {
    unsafe {
        let mut page: *mut page = ::core::ptr::null_mut::<page>();
        let mut timens_page: *mut page = ::core::ptr::null_mut::<page>();
        timens_page = find_timens_vvar_page(vma);
        match (*vmf).c2rust_unnamed.pgoff {
            0 => {
                if false {
                    return VM_FAULT_SIGBUS as ::core::ffi::c_int as vm_fault_t;
                }
                page = mem_map.offset(
                    (({
                        let mut _x: ::core::ffi::c_ulong =
                            vdso_k_time_data.expose_provenance() as ::core::ffi::c_ulong;
                        (if _x >= kernel_map.page_offset
                            && (false
                                || (_x as usize)
                                    < (kernel_map.page_offset as usize).wrapping_add(
                                        ((1 as usize) << 12 as ::core::ffi::c_int)
                                            .wrapping_div(::core::mem::size_of::<pgd_t>())
                                            .wrapping_div(2 as usize)
                                            .wrapping_mul(
                                                (1 as usize)
                                                    << (if pgtable_l5_enabled as ::core::ffi::c_int
                                                        != 0
                                                    {
                                                        48 as ::core::ffi::c_int
                                                    } else {
                                                        (if pgtable_l4_enabled as ::core::ffi::c_int
                                                            != 0
                                                        {
                                                            39 as ::core::ffi::c_int
                                                        } else {
                                                            30 as ::core::ffi::c_int
                                                        })
                                                    }),
                                            )
                                            .wrapping_div(2 as usize),
                                    ))
                        {
                            _x.wrapping_sub(kernel_map.va_pa_offset)
                        } else {
                            _x.wrapping_sub(kernel_map.va_kernel_pa_offset)
                        })
                    }) >> 12 as ::core::ffi::c_int)
                        .wrapping_sub(phys_ram_base as ::core::ffi::c_ulong >> PAGE_SHIFT)
                        as isize,
                ) as *mut page;
                if !timens_page.is_null() {
                    let mut addr: ::core::ffi::c_ulong = 0;
                    let mut err: vm_fault_t = 0;
                    addr = (*vmf).c2rust_unnamed.address.wrapping_add(
                        (VDSO_TIMENS_PAGE_OFFSET as ::core::ffi::c_int as ::core::ffi::c_ulong)
                            .wrapping_mul(PAGE_SIZE),
                    );
                    err = vmf_insert_page(vma as *mut vm_area_struct, addr, page);
                    if (err
                        & (VM_FAULT_OOM as ::core::ffi::c_int
                            | VM_FAULT_SIGBUS as ::core::ffi::c_int
                            | VM_FAULT_SIGSEGV as ::core::ffi::c_int
                            | VM_FAULT_HWPOISON as ::core::ffi::c_int
                            | VM_FAULT_HWPOISON_LARGE as ::core::ffi::c_int
                            | VM_FAULT_FALLBACK as ::core::ffi::c_int)
                            as vm_fault_t
                        != 0) as ::core::ffi::c_int as ::core::ffi::c_long
                        != 0
                    {
                        return err;
                    }
                    page = timens_page;
                }
            }
            1 => {
                if true || timens_page.is_null() {
                    return VM_FAULT_SIGBUS as ::core::ffi::c_int as vm_fault_t;
                }
                page = mem_map.offset(
                    (({
                        let mut _x: ::core::ffi::c_ulong =
                            vdso_k_time_data.expose_provenance() as ::core::ffi::c_ulong;
                        (if _x >= kernel_map.page_offset
                            && (false
                                || (_x as usize)
                                    < (kernel_map.page_offset as usize).wrapping_add(
                                        ((1 as usize) << 12 as ::core::ffi::c_int)
                                            .wrapping_div(::core::mem::size_of::<pgd_t>())
                                            .wrapping_div(2 as usize)
                                            .wrapping_mul(
                                                (1 as usize)
                                                    << (if pgtable_l5_enabled as ::core::ffi::c_int
                                                        != 0
                                                    {
                                                        48 as ::core::ffi::c_int
                                                    } else {
                                                        (if pgtable_l4_enabled as ::core::ffi::c_int
                                                            != 0
                                                        {
                                                            39 as ::core::ffi::c_int
                                                        } else {
                                                            30 as ::core::ffi::c_int
                                                        })
                                                    }),
                                            )
                                            .wrapping_div(2 as usize),
                                    ))
                        {
                            _x.wrapping_sub(kernel_map.va_pa_offset)
                        } else {
                            _x.wrapping_sub(kernel_map.va_kernel_pa_offset)
                        })
                    }) >> 12 as ::core::ffi::c_int)
                        .wrapping_sub(phys_ram_base as ::core::ffi::c_ulong >> PAGE_SHIFT)
                        as isize,
                ) as *mut page;
            }
            2 => {
                if false {
                    return VM_FAULT_SIGBUS as ::core::ffi::c_int as vm_fault_t;
                }
                page = mem_map.offset(
                    (({
                        let mut _x: ::core::ffi::c_ulong =
                            vdso_k_rng_data.expose_provenance() as ::core::ffi::c_ulong;
                        (if _x >= kernel_map.page_offset
                            && (false
                                || (_x as usize)
                                    < (kernel_map.page_offset as usize).wrapping_add(
                                        ((1 as usize) << 12 as ::core::ffi::c_int)
                                            .wrapping_div(::core::mem::size_of::<pgd_t>())
                                            .wrapping_div(2 as usize)
                                            .wrapping_mul(
                                                (1 as usize)
                                                    << (if pgtable_l5_enabled as ::core::ffi::c_int
                                                        != 0
                                                    {
                                                        48 as ::core::ffi::c_int
                                                    } else {
                                                        (if pgtable_l4_enabled as ::core::ffi::c_int
                                                            != 0
                                                        {
                                                            39 as ::core::ffi::c_int
                                                        } else {
                                                            30 as ::core::ffi::c_int
                                                        })
                                                    }),
                                            )
                                            .wrapping_div(2 as usize),
                                    ))
                        {
                            _x.wrapping_sub(kernel_map.va_pa_offset)
                        } else {
                            _x.wrapping_sub(kernel_map.va_kernel_pa_offset)
                        })
                    }) >> 12 as ::core::ffi::c_int)
                        .wrapping_sub(phys_ram_base as ::core::ffi::c_ulong >> PAGE_SHIFT)
                        as isize,
                ) as *mut page;
            }
            3 => {
                if false {
                    return VM_FAULT_SIGBUS as ::core::ffi::c_int as vm_fault_t;
                }
                page = mem_map
                    .offset(
                        (({
                            let mut _x: ::core::ffi::c_ulong =
                                vdso_k_arch_data.expose_provenance() as ::core::ffi::c_ulong;
                            (if _x >= kernel_map.page_offset
                                && (false
                                    || (_x as usize)
                                        < (kernel_map.page_offset as usize).wrapping_add(
                                            ((1 as usize) << 12 as ::core::ffi::c_int)
                                                .wrapping_div(::core::mem::size_of::<pgd_t>())
                                                .wrapping_div(2 as usize)
                                                .wrapping_mul(
                                                    (1 as usize)
                                                        << (if pgtable_l5_enabled
                                                            as ::core::ffi::c_int
                                                            != 0
                                                        {
                                                            48 as ::core::ffi::c_int
                                                        } else {
                                                            (if pgtable_l4_enabled
                                                                as ::core::ffi::c_int
                                                                != 0
                                                            {
                                                                39 as ::core::ffi::c_int
                                                            } else {
                                                                30 as ::core::ffi::c_int
                                                            })
                                                        }),
                                                )
                                                .wrapping_div(2 as usize),
                                        ))
                            {
                                _x.wrapping_sub(kernel_map.va_pa_offset)
                            } else {
                                _x.wrapping_sub(kernel_map.va_kernel_pa_offset)
                            })
                        }) >> 12 as ::core::ffi::c_int)
                            .wrapping_sub(phys_ram_base as ::core::ffi::c_ulong >> PAGE_SHIFT)
                            as isize,
                    )
                    .offset((*vmf).c2rust_unnamed.pgoff as isize)
                    .offset(-(VDSO_ARCH_PAGES_START as ::core::ffi::c_int as isize))
                    as *mut page;
            }
            _ => return VM_FAULT_SIGBUS as ::core::ffi::c_int as vm_fault_t,
        }
        get_page(page);
        (*vmf).page = page as *mut page;
        return 0 as vm_fault_t;
    }
}
// [bisect test] C original is `const struct vm_special_mapping
// vdso_vvar_mapping = {...}` -- read-only, lands in .rodata with
// relocations in .rela.rodata (confirmed via readelf against
// linux-riscv-worktrees/pure-c-baseline's real datastore.o). The
// c2rust translation emitted `static mut`, which lands in writable
// .data with relocations in .rela.data instead -- a genuine,
// confirmed section-placement discrepancy. Testing whether removing
// `mut` (matching the C's const-ness) changes the hang.
#[no_mangle]
pub static vdso_vvar_mapping: vm_special_mapping = vm_special_mapping {
    name: b"[vvar]\0".as_ptr() as *const ::core::ffi::c_char,
    pages: ::core::ptr::null_mut::<*mut page>(),
    fault: Some(
        vvar_fault
            as unsafe extern "C" fn(
                *const vm_special_mapping,
                *mut vm_area_struct,
                *mut vm_fault,
            ) -> vm_fault_t,
    ),
    mremap: None,
    close: None,
};
#[no_mangle]
pub unsafe extern "C" fn vdso_install_vvar_mapping(
    mut mm: *mut mm_struct,
    mut addr: ::core::ffi::c_ulong,
) -> *mut vm_area_struct {
    unsafe {
        return _install_special_mapping(
            mm,
            addr,
            (VDSO_NR_PAGES as ::core::ffi::c_int as ::core::ffi::c_ulong).wrapping_mul(PAGE_SIZE),
            (1 as vm_flags_t) << VMA_READ_BIT as ::core::ffi::c_int
                | (1 as vm_flags_t) << VMA_MAYREAD_BIT as ::core::ffi::c_int
                | (1 as vm_flags_t) << VMA_IO_BIT as ::core::ffi::c_int
                | (1 as vm_flags_t) << VMA_DONTDUMP_BIT as ::core::ffi::c_int
                | (1 as vm_flags_t) << VMA_MIXEDMAP_BIT as ::core::ffi::c_int
                | VM_SEALED_SYSMAP as vm_flags_t,
            &raw const vdso_vvar_mapping,
        ) as *mut vm_area_struct;
    }
}
pub const CONFIG_PAGE_SHIFT: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
// c2rust's glibc-style INIT_ARRAY/c2rust_run_static_initializers
// constructor trick removed -- dead in a kernel-linker context, and
// additionally dead HERE: vdso_setup_data_pages() unconditionally
// overwrites vdso_k_{arch,rng,time}_data before any real code path
// reads the constructor's precomputed values. Removing this also fixed
// a real modpost section-mismatch (the constructor sat in .text but
// looked reachable from vdso_initdata's real .init.data placement).


