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
extern "C" {
    fn kunit_ptr_not_err_assert_format(
        assert: *const kunit_assert,
        message: *const va_format,
        stream: *mut string_stream,
    );
    fn kunit_binary_assert_format(
        assert: *const kunit_assert,
        message: *const va_format,
        stream: *mut string_stream,
    );
    fn kunit_kmalloc_array(
        test: *mut kunit,
        n: size_t,
        size: size_t,
        gfp: gfp_t,
    ) -> *mut ::core::ffi::c_void;
    fn __kunit_abort(test: *mut kunit) -> !;
    fn __kunit_do_failed_assertion(
        test: *mut kunit,
        loc: *const kunit_loc,
        r#type: kunit_assert_type,
        assert: *const kunit_assert,
        assert_format: assert_format_t,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn sort(
        base: *mut ::core::ffi::c_void,
        num: size_t,
        size: size_t,
        cmp_func: cmp_func_t,
        swap_func: swap_func_t,
    );
}
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
opaque_marker!(task_struct, mmap_action, kobject, kernfs_open_file, percpu_ref_data, signal_struct, sched_dl_entity);

pub type __builtin_va_list = *mut ::core::ffi::c_void;
pub type __s8 = ::core::ffi::c_schar;
pub type __u8 = ::core::ffi::c_uchar;
pub type __u16 = ::core::ffi::c_ushort;
pub type __s32 = ::core::ffi::c_int;
pub type __u32 = ::core::ffi::c_uint;
pub type __s64 = ::core::ffi::c_longlong;
pub type __u64 = ::core::ffi::c_ulonglong;
pub type s8 = __s8;
pub type u8_0 = __u8;
pub type u16_0 = __u16;
pub type s32 = __s32;
pub type u32_0 = __u32;
pub type s64 = __s64;
pub type u64_0 = __u64;
pub type __kernel_long_t = ::core::ffi::c_long;
pub type __kernel_ulong_t = ::core::ffi::c_ulong;
pub type __kernel_pid_t = ::core::ffi::c_int;
pub type __kernel_uid32_t = ::core::ffi::c_uint;
pub type __kernel_gid32_t = ::core::ffi::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_loff_t = ::core::ffi::c_longlong;
pub type __kernel_time64_t = ::core::ffi::c_longlong;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = ::core::ffi::c_int;
pub type __kernel_clockid_t = ::core::ffi::c_int;
pub type __poll_t = ::core::ffi::c_uint;
pub type __kernel_dev_t = u32_0;
pub type dev_t = __kernel_dev_t;
pub type umode_t = ::core::ffi::c_ushort;
pub type pid_t = __kernel_pid_t;
pub type clockid_t = __kernel_clockid_t;
pub type bool_0 = bool;
pub type uid_t = __kernel_uid32_t;
pub type gid_t = __kernel_gid32_t;
pub type loff_t = __kernel_loff_t;
pub type size_t = __kernel_size_t;
pub type ssize_t = isize;
pub type uint32_t = u32;
pub type ktime_t = s64;
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
#[allow(dead_code, non_upper_case_globals)]
const C2Rust_callback_head_PADDING: usize =
    ::core::mem::size_of::<callback_head>() - ::core::mem::size_of::<C2Rust_callback_head_Inner>();
pub type swap_func_t = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type cmp_func_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rcuwait {
    pub task: *mut task_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread_struct {
    pub ra: ::core::ffi::c_ulong,
    pub sp: ::core::ffi::c_ulong,
    pub s: [::core::ffi::c_ulong; 12],
    pub fstate: __riscv_d_ext_state,
    pub bad_cause: ::core::ffi::c_ulong,
    pub envcfg: ::core::ffi::c_ulong,
    pub sum: ::core::ffi::c_ulong,
    pub riscv_v_flags: u32_0,
    pub vstate_ctrl: u32_0,
    pub vstate: __riscv_v_ext_state,
    pub align_ctl: ::core::ffi::c_ulong,
    pub kernel_vstate: __riscv_v_ext_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __riscv_v_ext_state {
    pub vstart: ::core::ffi::c_ulong,
    pub vl: ::core::ffi::c_ulong,
    pub vtype: ::core::ffi::c_ulong,
    pub vcsr: ::core::ffi::c_ulong,
    pub vlenb: ::core::ffi::c_ulong,
    pub datap: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __riscv_d_ext_state {
    pub f: [__u64; 32],
    pub fcsr: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpf_net_context {
    _private: [u8; 0],
}
pub type refcount_t = refcount_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refcount_struct {
    pub refs: atomic_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timer_list {
    pub entry: hlist_node,
    pub expires: ::core::ffi::c_ulong,
    pub function: Option<unsafe extern "C" fn(*mut timer_list) -> ()>,
    pub flags: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmap_ctrl {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kunit {
    pub r#priv: *mut ::core::ffi::c_void,
    pub parent: *mut kunit,
    pub params_array: kunit_params,
    pub name: *const ::core::ffi::c_char,
    pub log: *mut string_stream,
    pub try_catch: kunit_try_catch,
    pub param_value: *const ::core::ffi::c_void,
    pub param_index: ::core::ffi::c_int,
    pub lock: spinlock_t,
    pub status: kunit_status,
    pub resources: list_head,
    pub status_comment: [::core::ffi::c_char; 256],
    pub last_seen: kunit_loc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kunit_loc {
    pub line: ::core::ffi::c_int,
    pub file: *const ::core::ffi::c_char,
}
pub type kunit_status = ::core::ffi::c_uint;
pub const KUNIT_SKIPPED: kunit_status = 2;
pub const KUNIT_FAILURE: kunit_status = 1;
pub const KUNIT_SUCCESS: kunit_status = 0;
pub type spinlock_t = spinlock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spinlock {
    pub c2rust_unnamed: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub rlock: raw_spinlock,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct raw_spinlock {
    pub raw_lock: arch_spinlock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arch_spinlock_t {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kunit_try_catch {
    pub test: *mut kunit,
    pub try_result: ::core::ffi::c_int,
    pub r#try: kunit_try_catch_func_t,
    pub catch: kunit_try_catch_func_t,
    pub timeout: ::core::ffi::c_ulong,
    pub context: *mut ::core::ffi::c_void,
}
pub type kunit_try_catch_func_t = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_stream {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kunit_params {
    pub params: *const ::core::ffi::c_void,
    pub get_description: Option<
        unsafe extern "C" fn(
            *mut kunit,
            *const ::core::ffi::c_void,
            *mut ::core::ffi::c_char,
        ) -> (),
    >,
    pub num_params: size_t,
    pub elem_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct page_frag {
    pub page: *mut page,
    pub offset: __u32,
    pub size: __u32,
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
    pub c2rust_unnamed: C2Rust_Unnamed_66,
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
    pub iter_type: u8_0,
    pub nofault: bool_0,
    pub data_source: bool_0,
    pub iov_offset: size_t,
    pub c2rust_unnamed: C2Rust_Unnamed_8,
    pub c2rust_unnamed_0: C2Rust_Unnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_7 {
    pub nr_segs: ::core::ffi::c_ulong,
    pub folioq_slot: u8_0,
    pub xarray_start: loff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
    pub __ubuf_iovec: iovec,
    pub c2rust_unnamed: C2Rust_Unnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
    pub c2rust_unnamed: C2Rust_Unnamed_10,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_10 {
    pub __iov: *const iovec,
    pub kvec: *const kvec,
    pub bvec: *const bio_vec,
    pub folioq: *const folio_queue,
    pub xarray: *mut xarray,
    pub ubuf: *mut ::core::ffi::c_void,
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
pub struct folio_queue {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bio_vec {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kvec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: __kernel_size_t,
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
    pub c2rust_unnamed: C2Rust_Unnamed_13,
    pub c2rust_unnamed_0: C2Rust_Unnamed_12,
    pub f_pos: loff_t,
    pub f_wb_err: errseq_t,
    pub f_sb_err: errseq_t,
    pub c2rust_unnamed_1: C2Rust_Unnamed_11,
    pub f_ref: file_ref_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_ref_t {
    pub refcnt: atomic64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_11 {
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
pub union C2Rust_Unnamed_12 {
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
pub union C2Rust_Unnamed_13 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_45,
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
    pub c2rust_unnamed_0: C2Rust_Unnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_14 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_15 {
    pub c2rust_unnamed: C2Rust_Unnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
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
pub const D_REAL_METADATA: d_real_type = 1;
pub const D_REAL_DATA: d_real_type = 0;
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
    pub c2rust_unnamed: C2Rust_Unnamed_39,
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
    pub c2rust_unnamed_0: C2Rust_Unnamed_38,
    pub i_version: atomic64_t,
    pub i_sequence: atomic64_t,
    pub i_count: atomic_t,
    pub i_dio_count: atomic_t,
    pub i_writecount: atomic_t,
    pub c2rust_unnamed_1: C2Rust_Unnamed_19,
    pub i_flctx: *mut file_lock_context,
    pub i_data: address_space,
    pub c2rust_unnamed_2: C2Rust_Unnamed_18,
    pub c2rust_unnamed_3: C2Rust_Unnamed_17,
    pub i_private: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_17 {
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
pub union C2Rust_Unnamed_18 {
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
pub union C2Rust_Unnamed_19 {
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
pub type mmap_action_type = ::core::ffi::c_uint;
pub const MMAP_MAP_KERNEL_PAGES: mmap_action_type = 4;
pub const MMAP_SIMPLE_IO_REMAP: mmap_action_type = 3;
pub const MMAP_IO_REMAP_PFN: mmap_action_type = 2;
pub const MMAP_REMAP_PFN: mmap_action_type = 1;
pub const MMAP_NOTHING: mmap_action_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_20 {
    pub remap: C2Rust_Unnamed_23,
    pub simple_ioremap: C2Rust_Unnamed_22,
    pub map_kernel: C2Rust_Unnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub start: ::core::ffi::c_ulong,
    pub pages: *mut *mut page,
    pub nr_pages: ::core::ffi::c_ulong,
    pub pgoff: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub start_phys_addr: phys_addr_t,
    pub size: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub start: ::core::ffi::c_ulong,
    pub start_pfn: ::core::ffi::c_ulong,
    pub size: ::core::ffi::c_ulong,
    pub pgprot: pgprot_t,
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
#[repr(C)]
pub struct vm_area_struct {
    pub c2rust_unnamed: C2Rust_Unnamed_30,
    pub vm_mm: *mut mm_struct,
    pub vm_page_prot: pgprot_t,
    pub c2rust_unnamed_0: C2Rust_Unnamed_25,
    pub anon_vma_chain: list_head,
    pub anon_vma: *mut anon_vma,
    pub vm_ops: *const vm_operations_struct,
    pub vm_pgoff: ::core::ffi::c_ulong,
    pub vm_file: *mut file,
    pub vm_private_data: *mut ::core::ffi::c_void,
    pub swap_readahead_info: atomic_long_t,
    pub shared: C2Rust_Unnamed_24,
    pub vm_userfaultfd_ctx: vm_userfaultfd_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vm_userfaultfd_ctx {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
    pub rb: rb_node,
    pub rb_subtree_last: ::core::ffi::c_ulong,
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
#[allow(dead_code, non_upper_case_globals)]
const C2Rust_rb_node_PADDING: usize =
    ::core::mem::size_of::<rb_node>() - ::core::mem::size_of::<C2Rust_rb_node_Inner>();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct anon_vma {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_25 {
    pub vm_flags: vm_flags_t,
    pub flags: vma_flags_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vma_flags_t {
    pub __vma_flags: [::core::ffi::c_ulong; 1],
}
pub type vm_flags_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_struct {
    pub c2rust_unnamed: C2Rust_Unnamed_26,
    pub flexible_array: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
    pub c2rust_unnamed: C2Rust_Unnamed_29,
    pub mm_mt: maple_tree,
    pub mmap_base: ::core::ffi::c_ulong,
    pub mmap_legacy_base: ::core::ffi::c_ulong,
    pub task_size: ::core::ffi::c_ulong,
    pub pgd: *mut pgd_t,
    pub mm_users: atomic_t,
    pub mm_cid: mm_mm_cid,
    pub sc_stat: sched_cache_stat,
    pub pgtables_bytes: atomic_long_t,
    pub map_count: ::core::ffi::c_int,
    pub page_table_lock: spinlock_t,
    pub mmap_lock: rw_semaphore,
    pub mmlist: list_head,
    pub futex: futex_mm_data,
    pub hiwater_rss: ::core::ffi::c_ulong,
    pub hiwater_vm: ::core::ffi::c_ulong,
    pub total_vm: ::core::ffi::c_ulong,
    pub locked_vm: ::core::ffi::c_ulong,
    pub pinned_vm: atomic64_t,
    pub data_vm: ::core::ffi::c_ulong,
    pub exec_vm: ::core::ffi::c_ulong,
    pub stack_vm: ::core::ffi::c_ulong,
    pub c2rust_unnamed_0: C2Rust_Unnamed_27,
    pub write_protect_seq: seqcount_t,
    pub arg_lock: spinlock_t,
    pub start_code: ::core::ffi::c_ulong,
    pub end_code: ::core::ffi::c_ulong,
    pub start_data: ::core::ffi::c_ulong,
    pub end_data: ::core::ffi::c_ulong,
    pub start_brk: ::core::ffi::c_ulong,
    pub brk: ::core::ffi::c_ulong,
    pub start_stack: ::core::ffi::c_ulong,
    pub arg_start: ::core::ffi::c_ulong,
    pub arg_end: ::core::ffi::c_ulong,
    pub env_start: ::core::ffi::c_ulong,
    pub env_end: ::core::ffi::c_ulong,
    pub saved_auxv: [::core::ffi::c_ulong; 70],
    pub rss_stat: [percpu_counter; 4],
    pub binfmt: *mut linux_binfmt,
    pub context: mm_context_t,
    pub flags: mm_flags_t,
    pub exe_file: *mut file,
    pub tlb_flush_pending: atomic_t,
    pub tlb_flush_batched: atomic_t,
    pub uprobes_state: uprobes_state,
    pub async_put_work: work_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uprobes_state {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_flags_t {
    pub __mm_flags: [::core::ffi::c_ulong; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_context_t {
    pub id: atomic_long_t,
    pub vdso: *mut ::core::ffi::c_void,
    pub flags: ::core::ffi::c_ulong,
    pub pmlen: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_binfmt {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct percpu_counter {
    pub count: s64,
}
pub type seqcount_t = seqcount;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqcount {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_27 {
    pub def_flags: vm_flags_t,
    pub def_vma_flags: vma_flags_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct futex_mm_data {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_cache_stat {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_mm_cid {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pgd_t {
    pub pgd: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct maple_tree {
    pub c2rust_unnamed: C2Rust_Unnamed_28,
    pub ma_flags: ::core::ffi::c_uint,
    pub ma_root: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_28 {
    pub ma_lock: spinlock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_29 {
    pub mm_count: atomic_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_30 {
    pub c2rust_unnamed: C2Rust_Unnamed_31,
    pub vm_freeptr: freeptr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
    pub vm_start: ::core::ffi::c_ulong,
    pub vm_end: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vm_fault {
    pub c2rust_unnamed: C2Rust_Unnamed_33,
    pub flags: fault_flag,
    pub pmd: *mut pmd_t,
    pub pud: *mut pud_t,
    pub c2rust_unnamed_0: C2Rust_Unnamed_32,
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
pub union C2Rust_Unnamed_32 {
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
pub const FAULT_FLAG_VMA_LOCK: fault_flag = 4096;
pub const FAULT_FLAG_ORIG_PTE_VALID: fault_flag = 2048;
pub const FAULT_FLAG_UNSHARE: fault_flag = 1024;
pub const FAULT_FLAG_INTERRUPTIBLE: fault_flag = 512;
pub const FAULT_FLAG_INSTRUCTION: fault_flag = 256;
pub const FAULT_FLAG_REMOTE: fault_flag = 128;
pub const FAULT_FLAG_USER: fault_flag = 64;
pub const FAULT_FLAG_TRIED: fault_flag = 32;
pub const FAULT_FLAG_KILLABLE: fault_flag = 16;
pub const FAULT_FLAG_RETRY_NOWAIT: fault_flag = 8;
pub const FAULT_FLAG_ALLOW_RETRY: fault_flag = 4;
pub const FAULT_FLAG_MKWRITE: fault_flag = 2;
pub const FAULT_FLAG_WRITE: fault_flag = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_33 {
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
    pub state: module_state,
    pub list: list_head,
    pub name: [::core::ffi::c_char; 56],
    pub mkobj: module_kobject,
    pub modinfo_attrs: *mut module_attribute,
    pub version: *const ::core::ffi::c_char,
    pub srcversion: *const ::core::ffi::c_char,
    pub imported_namespaces: *const ::core::ffi::c_char,
    pub holders_dir: *mut kobject,
    pub syms: *const kernel_symbol,
    pub crcs: *const u32_0,
    pub flagstab: *const u8_0,
    pub num_syms: ::core::ffi::c_uint,
    pub param_lock: mutex,
    pub kp: *mut kernel_param,
    pub num_kp: ::core::ffi::c_uint,
    pub using_gplonly_symbols: bool_0,
    pub async_probe_requested: bool_0,
    pub num_exentries: ::core::ffi::c_uint,
    pub extable: *mut exception_table_entry,
    pub init: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub mem: [module_memory; 7],
    pub arch: mod_arch_specific,
    pub taints: ::core::ffi::c_ulong,
    pub args: *mut ::core::ffi::c_char,
    pub noinstr_text_start: *mut ::core::ffi::c_void,
    pub noinstr_text_size: ::core::ffi::c_uint,
    pub num_kunit_init_suites: ::core::ffi::c_int,
    pub kunit_init_suites: *mut *mut kunit_suite,
    pub num_kunit_suites: ::core::ffi::c_int,
    pub kunit_suites: *mut *mut kunit_suite,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kunit_suite {
    pub name: [::core::ffi::c_char; 256],
    pub suite_init: Option<unsafe extern "C" fn(*mut kunit_suite) -> ::core::ffi::c_int>,
    pub suite_exit: Option<unsafe extern "C" fn(*mut kunit_suite) -> ()>,
    pub init: Option<unsafe extern "C" fn(*mut kunit) -> ::core::ffi::c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut kunit) -> ()>,
    pub test_cases: *mut kunit_case,
    pub attr: kunit_attributes,
    pub status_comment: [::core::ffi::c_char; 256],
    pub debugfs: *mut dentry,
    pub log: *mut string_stream,
    pub suite_init_err: ::core::ffi::c_int,
    pub is_init: bool_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kunit_attributes {
    pub speed: kunit_speed,
}
pub type kunit_speed = ::core::ffi::c_uint;
pub const KUNIT_SPEED_MAX: kunit_speed = 3;
pub const KUNIT_SPEED_NORMAL: kunit_speed = 3;
pub const KUNIT_SPEED_SLOW: kunit_speed = 2;
pub const KUNIT_SPEED_VERY_SLOW: kunit_speed = 1;
pub const KUNIT_SPEED_UNSET: kunit_speed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kunit_case {
    pub run_case: Option<unsafe extern "C" fn(*mut kunit) -> ()>,
    pub name: *const ::core::ffi::c_char,
    pub generate_params: Option<
        unsafe extern "C" fn(
            *mut kunit,
            *const ::core::ffi::c_void,
            *mut ::core::ffi::c_char,
        ) -> *const ::core::ffi::c_void,
    >,
    pub attr: kunit_attributes,
    pub param_init: Option<unsafe extern "C" fn(*mut kunit) -> ::core::ffi::c_int>,
    pub param_exit: Option<unsafe extern "C" fn(*mut kunit) -> ()>,
    pub status: kunit_status,
    pub module_name: *mut ::core::ffi::c_char,
    pub log: *mut string_stream,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mod_arch_specific {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct module_memory {
    pub base: *mut ::core::ffi::c_void,
    pub is_rox: bool_0,
    pub size: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exception_table_entry {
    pub insn: ::core::ffi::c_int,
    pub fixup: ::core::ffi::c_int,
    pub r#type: ::core::ffi::c_short,
    pub data: ::core::ffi::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kernel_param {
    pub name: *const ::core::ffi::c_char,
    pub r#mod: *mut module,
    pub ops: *const kernel_param_ops,
    pub perm: u16_0,
    pub level: s8,
    pub flags: u8_0,
    pub c2rust_unnamed: C2Rust_Unnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_34 {
    pub arg: *mut ::core::ffi::c_void,
    pub str: *const kparam_string,
    pub arr: *const kparam_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kparam_array {
    pub max: ::core::ffi::c_uint,
    pub elemsize: ::core::ffi::c_uint,
    pub num: *mut ::core::ffi::c_uint,
    pub ops: *const kernel_param_ops,
    pub elem: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kernel_param_ops {
    pub flags: ::core::ffi::c_uint,
    pub set: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char, *const kernel_param) -> ::core::ffi::c_int,
    >,
    pub get: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_char, *const kernel_param) -> ::core::ffi::c_int,
    >,
    pub free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kparam_string {
    pub maxlen: ::core::ffi::c_uint,
    pub string: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kernel_symbol {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kref {
    pub refcount: refcount_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kernfs_node {
    pub count: atomic_t,
    pub active: atomic_t,
    pub __parent: *mut kernfs_node,
    pub name: *const ::core::ffi::c_char,
    pub rb: rb_node,
    pub ns: *const ns_common,
    pub hash: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_ushort,
    pub mode: umode_t,
    pub c2rust_unnamed: C2Rust_Unnamed_35,
    pub id: u64_0,
    pub r#priv: *mut ::core::ffi::c_void,
    pub iattr: *mut kernfs_iattrs,
    pub rcu: callback_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kernfs_iattrs {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_35 {
    pub dir: kernfs_elem_dir,
    pub symlink: kernfs_elem_symlink,
    pub attr: kernfs_elem_attr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kernfs_elem_attr {
    pub ops: *const kernfs_ops,
    pub open: *mut kernfs_open_node,
    pub size: loff_t,
    pub notify_next: *mut kernfs_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kernfs_open_node {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kernfs_ops {
    pub open: Option<unsafe extern "C" fn(*mut kernfs_open_file) -> ::core::ffi::c_int>,
    pub release: Option<unsafe extern "C" fn(*mut kernfs_open_file) -> ()>,
    pub seq_show:
        Option<unsafe extern "C" fn(*mut seq_file, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub seq_start:
        Option<unsafe extern "C" fn(*mut seq_file, *mut loff_t) -> *mut ::core::ffi::c_void>,
    pub seq_next: Option<
        unsafe extern "C" fn(
            *mut seq_file,
            *mut ::core::ffi::c_void,
            *mut loff_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub seq_stop: Option<unsafe extern "C" fn(*mut seq_file, *mut ::core::ffi::c_void) -> ()>,
    pub read: Option<
        unsafe extern "C" fn(
            *mut kernfs_open_file,
            *mut ::core::ffi::c_char,
            size_t,
            loff_t,
        ) -> ssize_t,
    >,
    pub atomic_write_len: size_t,
    pub prealloc: bool_0,
    pub write: Option<
        unsafe extern "C" fn(
            *mut kernfs_open_file,
            *mut ::core::ffi::c_char,
            size_t,
            loff_t,
        ) -> ssize_t,
    >,
    pub poll:
        Option<unsafe extern "C" fn(*mut kernfs_open_file, *mut poll_table_struct) -> __poll_t>,
    pub mmap: Option<
        unsafe extern "C" fn(*mut kernfs_open_file, *mut vm_area_struct) -> ::core::ffi::c_int,
    >,
    pub llseek:
        Option<unsafe extern "C" fn(*mut kernfs_open_file, loff_t, ::core::ffi::c_int) -> loff_t>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kernfs_elem_symlink {
    pub target_kn: *mut kernfs_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kernfs_elem_dir {
    pub subdirs: ::core::ffi::c_ulong,
    pub children: rb_root,
    pub root: *mut kernfs_root,
    pub rev: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kernfs_root {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ns_common {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kobj_type {
    pub release: Option<unsafe extern "C" fn(*mut kobject) -> ()>,
    pub sysfs_ops: *const sysfs_ops,
    pub default_groups: *mut *const attribute_group,
    pub child_ns_type:
        Option<unsafe extern "C" fn(*const kobject) -> *const kobj_ns_type_operations>,
    pub namespace: Option<unsafe extern "C" fn(*const kobject) -> *const ns_common>,
    pub get_ownership: Option<unsafe extern "C" fn(*const kobject, *mut kuid_t, *mut kgid_t) -> ()>,
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
pub struct kobj_ns_type_operations {
    pub r#type: kobj_ns_type,
    pub current_may_mount: Option<unsafe extern "C" fn() -> bool_0>,
    pub grab_current_ns: Option<unsafe extern "C" fn() -> *mut ns_common>,
    pub netlink_ns: Option<unsafe extern "C" fn(*mut sock) -> *const ns_common>,
    pub initial_ns: Option<unsafe extern "C" fn() -> *const ns_common>,
    pub drop_ns: Option<unsafe extern "C" fn(*mut ns_common) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}
pub type kobj_ns_type = ::core::ffi::c_uint;
pub const KOBJ_NS_TYPES: kobj_ns_type = 2;
pub const KOBJ_NS_TYPE_NET: kobj_ns_type = 1;
pub const KOBJ_NS_TYPE_NONE: kobj_ns_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attribute_group {
    pub name: *const ::core::ffi::c_char,
    pub c2rust_unnamed: C2Rust_Unnamed_37,
    pub is_bin_visible: Option<
        unsafe extern "C" fn(*mut kobject, *const bin_attribute, ::core::ffi::c_int) -> umode_t,
    >,
    pub bin_size: Option<
        unsafe extern "C" fn(*mut kobject, *const bin_attribute, ::core::ffi::c_int) -> size_t,
    >,
    pub c2rust_unnamed_0: C2Rust_Unnamed_36,
    pub bin_attrs: *const *const bin_attribute,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bin_attribute {
    pub attr: attribute,
    pub size: size_t,
    pub private: *mut ::core::ffi::c_void,
    pub f_mapping: Option<unsafe extern "C" fn() -> *mut address_space>,
    pub read: Option<
        unsafe extern "C" fn(
            *mut file,
            *mut kobject,
            *const bin_attribute,
            *mut ::core::ffi::c_char,
            loff_t,
            size_t,
        ) -> ssize_t,
    >,
    pub write: Option<
        unsafe extern "C" fn(
            *mut file,
            *mut kobject,
            *const bin_attribute,
            *mut ::core::ffi::c_char,
            loff_t,
            size_t,
        ) -> ssize_t,
    >,
    pub llseek: Option<
        unsafe extern "C" fn(
            *mut file,
            *mut kobject,
            *const bin_attribute,
            loff_t,
            ::core::ffi::c_int,
        ) -> loff_t,
    >,
    pub mmap: Option<
        unsafe extern "C" fn(
            *mut file,
            *mut kobject,
            *const bin_attribute,
            *mut vm_area_struct,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attribute {
    pub name: *const ::core::ffi::c_char,
    pub mode: umode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_36 {
    pub attrs: *mut *mut attribute,
    pub attrs_const: *const *const attribute,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_37 {
    pub is_visible:
        Option<unsafe extern "C" fn(*mut kobject, *mut attribute, ::core::ffi::c_int) -> umode_t>,
    pub is_visible_const:
        Option<unsafe extern "C" fn(*mut kobject, *const attribute, ::core::ffi::c_int) -> umode_t>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sysfs_ops {
    pub show: Option<
        unsafe extern "C" fn(*mut kobject, *mut attribute, *mut ::core::ffi::c_char) -> ssize_t,
    >,
    pub store: Option<
        unsafe extern "C" fn(
            *mut kobject,
            *mut attribute,
            *const ::core::ffi::c_char,
            size_t,
        ) -> ssize_t,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kset {
    pub list: list_head,
    pub list_lock: spinlock_t,
    pub kobj: kobject,
    pub uevent_ops: *const kset_uevent_ops,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kset_uevent_ops {
    pub filter: Option<unsafe extern "C" fn(*const kobject) -> ::core::ffi::c_int>,
    pub name: Option<unsafe extern "C" fn(*const kobject) -> *const ::core::ffi::c_char>,
    pub uevent:
        Option<unsafe extern "C" fn(*const kobject, *mut kobj_uevent_env) -> ::core::ffi::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kobj_uevent_env {
    pub argv: [*mut ::core::ffi::c_char; 3],
    pub envp: [*mut ::core::ffi::c_char; 64],
    pub envp_idx: ::core::ffi::c_int,
    pub buf: [::core::ffi::c_char; 2048],
    pub buflen: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct module_attribute {
    pub attr: attribute,
    pub show: Option<
        unsafe extern "C" fn(
            *const module_attribute,
            *mut module_kobject,
            *mut ::core::ffi::c_char,
        ) -> ssize_t,
    >,
    pub store: Option<
        unsafe extern "C" fn(
            *const module_attribute,
            *mut module_kobject,
            *const ::core::ffi::c_char,
            size_t,
        ) -> ssize_t,
    >,
    pub setup: Option<unsafe extern "C" fn(*mut module, *const ::core::ffi::c_char) -> ()>,
    pub test: Option<unsafe extern "C" fn(*mut module) -> ::core::ffi::c_int>,
    pub free: Option<unsafe extern "C" fn(*mut module) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct module_kobject {
    pub kobj: kobject,
    pub r#mod: *mut module,
    pub drivers_dir: *mut kobject,
    pub mp: *mut module_param_attrs,
    pub kobj_completion: *mut completion,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct module_param_attrs {
    _private: [u8; 0],
}
pub type module_state = ::core::ffi::c_uint;
pub const MODULE_STATE_UNFORMED: module_state = 3;
pub const MODULE_STATE_GOING: module_state = 2;
pub const MODULE_STATE_COMING: module_state = 1;
pub const MODULE_STATE_LIVE: module_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_38 {
    pub i_dentry: hlist_head,
    pub i_rcu: callback_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inode_state_flags {
    pub __state: inode_state_flags_enum,
}
pub type inode_state_flags_enum = ::core::ffi::c_uint;
pub const I_PINNING_NETFS_WB: inode_state_flags_enum = 262144;
pub const I_SYNC_QUEUED: inode_state_flags_enum = 131072;
pub const I_DONTCACHE: inode_state_flags_enum = 65536;
pub const I_CREATING: inode_state_flags_enum = 32768;
pub const I_OVL_INUSE: inode_state_flags_enum = 16384;
pub const I_WB_SWITCH: inode_state_flags_enum = 8192;
pub const I_DIRTY_TIME: inode_state_flags_enum = 4096;
pub const I_LINKABLE: inode_state_flags_enum = 2048;
pub const I_REFERENCED: inode_state_flags_enum = 1024;
pub const I_CLEAR: inode_state_flags_enum = 512;
pub const I_FREEING: inode_state_flags_enum = 256;
pub const I_WILL_FREE: inode_state_flags_enum = 128;
pub const I_DIRTY_PAGES: inode_state_flags_enum = 64;
pub const I_DIRTY_DATASYNC: inode_state_flags_enum = 32;
pub const I_DIRTY_SYNC: inode_state_flags_enum = 16;
pub const I_LRU_ISOLATING: inode_state_flags_enum = 4;
pub const I_SYNC: inode_state_flags_enum = 2;
pub const I_NEW: inode_state_flags_enum = 1;
pub type rw_hint = ::core::ffi::c_uchar;
pub const WRITE_LIFE_HINT_NR: rw_hint = 6;
pub const WRITE_LIFE_EXTREME: rw_hint = 5;
pub const WRITE_LIFE_LONG: rw_hint = 4;
pub const WRITE_LIFE_MEDIUM: rw_hint = 3;
pub const WRITE_LIFE_SHORT: rw_hint = 2;
pub const WRITE_LIFE_NONE: rw_hint = 1;
pub const WRITE_LIFE_NOT_SET: rw_hint = 0;
pub type time64_t = __s64;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_39 {
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
pub const FS_UPD_CMTIME: fs_update_time = 1;
pub const FS_UPD_ATIME: fs_update_time = 0;
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
pub struct iattr {
    pub ia_valid: ::core::ffi::c_uint,
    pub ia_mode: umode_t,
    pub c2rust_unnamed: C2Rust_Unnamed_41,
    pub c2rust_unnamed_0: C2Rust_Unnamed_40,
    pub ia_size: loff_t,
    pub ia_atime: timespec64,
    pub ia_mtime: timespec64,
    pub ia_ctime: timespec64,
    pub ia_file: *mut file,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_40 {
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
pub union C2Rust_Unnamed_41 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_42,
    pub name: *const ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_42 {
    pub c2rust_unnamed: C2Rust_Unnamed_43,
    pub hash_len: u64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_43 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_44,
    pub r#type: quota_type,
}
pub type quota_type = ::core::ffi::c_uint;
pub const PRJQUOTA: quota_type = 2;
pub const GRPQUOTA: quota_type = 1;
pub const USRQUOTA: quota_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_44 {
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
pub const FREEZE_EXCL: freeze_holder = 8;
pub const FREEZE_MAY_NEST: freeze_holder = 4;
pub const FREEZE_HOLDER_USERSPACE: freeze_holder = 2;
pub const FREEZE_HOLDER_KERNEL: freeze_holder = 1;
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
pub union C2Rust_Unnamed_45 {
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
pub const PIDTYPE_MAX: pid_type = 4;
pub const PIDTYPE_SID: pid_type = 3;
pub const PIDTYPE_PGID: pid_type = 2;
pub const PIDTYPE_TGID: pid_type = 1;
pub const PIDTYPE_PID: pid_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pid {
    pub count: refcount_t,
    pub level: ::core::ffi::c_uint,
    pub lock: spinlock_t,
    pub c2rust_unnamed: C2Rust_Unnamed_46,
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
pub struct C2Rust_Unnamed_46 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_47,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_47 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_57,
    pub c2rust_unnamed_0: C2Rust_Unnamed_52,
    pub c2rust_unnamed_1: C2Rust_Unnamed_50,
    pub c2rust_unnamed_2: C2Rust_Unnamed_48,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_48 {
    pub c2rust_unnamed: C2Rust_Unnamed_49,
    pub __page_3: page,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_49 {
    pub _flags_3: ::core::ffi::c_ulong,
    pub _head_3: ::core::ffi::c_ulong,
    pub _hugetlb_subpool: *mut ::core::ffi::c_void,
    pub _hugetlb_cgroup: *mut ::core::ffi::c_void,
    pub _hugetlb_cgroup_rsvd: *mut ::core::ffi::c_void,
    pub _hugetlb_hwpoison: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_50 {
    pub c2rust_unnamed: C2Rust_Unnamed_51,
    pub __page_2: page,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_51 {
    pub _flags_2: ::core::ffi::c_ulong,
    pub _head_2: ::core::ffi::c_ulong,
    pub _deferred_list: list_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_52 {
    pub c2rust_unnamed: C2Rust_Unnamed_53,
    pub __page_1: page,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_53 {
    pub _flags_1: ::core::ffi::c_ulong,
    pub _head_1: ::core::ffi::c_ulong,
    pub c2rust_unnamed: C2Rust_Unnamed_54,
    pub _mapcount_1: atomic_t,
    pub _refcount_1: atomic_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_54 {
    pub c2rust_unnamed: C2Rust_Unnamed_55,
    pub _usable_1: [::core::ffi::c_ulong; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_55 {
    pub _large_mapcount: atomic_t,
    pub _nr_pages_mapped: atomic_t,
    pub _entire_mapcount: atomic_t,
    pub _pincount: atomic_t,
    pub _mm_id_mapcount: [mm_id_mapcount_t; 2],
    pub c2rust_unnamed: C2Rust_Unnamed_56,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_56 {
    pub _mm_id: [mm_id_t; 2],
    pub _mm_ids: ::core::ffi::c_ulong,
}
pub type mm_id_t = ::core::ffi::c_uint;
pub type mm_id_mapcount_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_57 {
    pub c2rust_unnamed: C2Rust_Unnamed_58,
    pub page: page,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_58 {
    pub flags: memdesc_flags_t,
    pub c2rust_unnamed: C2Rust_Unnamed_61,
    pub mapping: *mut address_space,
    pub c2rust_unnamed_0: C2Rust_Unnamed_60,
    pub c2rust_unnamed_1: C2Rust_Unnamed_59,
    pub _mapcount: atomic_t,
    pub _refcount: atomic_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_59 {
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
pub union C2Rust_Unnamed_60 {
    pub index: ::core::ffi::c_ulong,
    pub share: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_61 {
    pub lru: list_head,
    pub c2rust_unnamed: C2Rust_Unnamed_65,
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
    pub c2rust_unnamed: C2Rust_Unnamed_62,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_62 {
    pub range: range,
    pub c2rust_unnamed: C2Rust_Unnamed_63,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_63 {
    pub __empty_ranges: C2Rust_Unnamed_64,
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
pub struct C2Rust_Unnamed_64 {}
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
pub const MEMORY_DEVICE_PCI_P2PDMA: memory_type = 5;
pub const MEMORY_DEVICE_GENERIC: memory_type = 4;
pub const MEMORY_DEVICE_FS_DAX: memory_type = 3;
pub const MEMORY_DEVICE_COHERENT: memory_type = 2;
pub const MEMORY_DEVICE_PRIVATE: memory_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct percpu_ref {
    pub percpu_count_ptr: ::core::ffi::c_ulong,
    pub data: *mut percpu_ref_data,
}
pub type percpu_ref_func_t = unsafe extern "C" fn(*mut percpu_ref) -> ();
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
pub struct C2Rust_Unnamed_65 {
    pub __filler: *mut ::core::ffi::c_void,
    pub mlock_count: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct memdesc_flags_t {
    pub f: ::core::ffi::c_ulong,
}
pub type migrate_mode = ::core::ffi::c_uint;
pub const MIGRATE_SYNC: migrate_mode = 2;
pub const MIGRATE_SYNC_LIGHT: migrate_mode = 1;
pub const MIGRATE_ASYNC: migrate_mode = 0;
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
pub union C2Rust_Unnamed_66 {
    pub lru: list_head,
    pub buddy_list: list_head,
    pub pcp_list: list_head,
    pub pcp_llist: llist_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tlbflush_unmap_batch {
    pub arch: arch_tlbflush_unmap_batch,
    pub flush_required: bool_0,
    pub writable: bool_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arch_tlbflush_unmap_batch {
    pub cpumask: cpumask,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpumask {
    pub bits: [::core::ffi::c_ulong; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_mm_cid {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rseq_data {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct perf_ctx_data {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct perf_event_context {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct futex_sched_data {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct task_io_accounting {}
pub type kernel_siginfo_t = kernel_siginfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kernel_siginfo {
    pub c2rust_unnamed: C2Rust_Unnamed_67,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_67 {
    pub si_signo: ::core::ffi::c_int,
    pub si_errno: ::core::ffi::c_int,
    pub si_code: ::core::ffi::c_int,
    pub _sifields: __sifields,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __sifields {
    pub _kill: C2Rust_Unnamed_78,
    pub _timer: C2Rust_Unnamed_77,
    pub _rt: C2Rust_Unnamed_76,
    pub _sigchld: C2Rust_Unnamed_75,
    pub _sigfault: C2Rust_Unnamed_70,
    pub _sigpoll: C2Rust_Unnamed_69,
    pub _sigsys: C2Rust_Unnamed_68,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_68 {
    pub _call_addr: *mut ::core::ffi::c_void,
    pub _syscall: ::core::ffi::c_int,
    pub _arch: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_69 {
    pub _band: ::core::ffi::c_long,
    pub _fd: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_70 {
    pub _addr: *mut ::core::ffi::c_void,
    pub c2rust_unnamed: C2Rust_Unnamed_71,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_71 {
    pub _trapno: ::core::ffi::c_int,
    pub _addr_lsb: ::core::ffi::c_short,
    pub _addr_bnd: C2Rust_Unnamed_74,
    pub _addr_pkey: C2Rust_Unnamed_73,
    pub _perf: C2Rust_Unnamed_72,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_72 {
    pub _data: ::core::ffi::c_ulong,
    pub _type: __u32,
    pub _flags: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_73 {
    pub _dummy_pkey: [::core::ffi::c_char; 8],
    pub _pkey: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_74 {
    pub _dummy_bnd: [::core::ffi::c_char; 8],
    pub _lower: *mut ::core::ffi::c_void,
    pub _upper: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_75 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
    pub _status: ::core::ffi::c_int,
    pub _utime: __kernel_clock_t,
    pub _stime: __kernel_clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_76 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
    pub _sigval: sigval_t,
}
pub type sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: ::core::ffi::c_int,
    pub sival_ptr: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_77 {
    pub _tid: __kernel_timer_t,
    pub _overrun: ::core::ffi::c_int,
    pub _sigval: sigval_t,
    pub _sys_private: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_78 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct io_context {
    pub refcount: atomic_long_t,
    pub active_ref: atomic_t,
    pub ioprio: ::core::ffi::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reclaim_state {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blk_plug {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bio_list {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wake_q_node {
    pub next: *mut wake_q_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct syscall_user_dispatch {
    pub selector: *mut ::core::ffi::c_char,
    pub offset: ::core::ffi::c_ulong,
    pub len: ::core::ffi::c_ulong,
    pub on_dispatch: bool_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seccomp {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigpending {
    pub list: list_head,
    pub signal: sigset_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigset_t {
    pub sig: [::core::ffi::c_ulong; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sighand_struct {
    pub siglock: spinlock_t,
    pub count: refcount_t,
    pub signalfd_wqh: wait_queue_head_t,
    pub action: [k_sigaction; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct k_sigaction {
    pub sa: sigaction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub sa_handler: __sighandler_t,
    pub sa_flags: ::core::ffi::c_ulong,
    pub sa_mask: sigset_t,
}
pub type __sighandler_t = Option<__signalfn_t>;
pub type __signalfn_t = unsafe extern "C" fn(::core::ffi::c_int) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: __kernel_ulong_t,
    pub rlim_max: __kernel_ulong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prev_cputime {
    pub utime: u64_0,
    pub stime: u64_0,
    pub lock: raw_spinlock_t,
}
pub type seqlock_t = seqlock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqlock {
    pub seqcount: seqcount_spinlock_t,
    pub lock: spinlock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_struct {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_cputimers {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct core_state {
    pub nr_threads: atomic_t,
    pub dumper: core_thread,
    pub startup: completion,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct core_thread {
    pub task: *mut task_struct,
    pub next: *mut core_thread,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nsproxy {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct files_struct {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fs_struct {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameidata {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct restart_block {
    pub arch_data: ::core::ffi::c_ulong,
    pub r#fn: Option<unsafe extern "C" fn(*mut restart_block) -> ::core::ffi::c_long>,
    pub c2rust_unnamed: C2Rust_Unnamed_79,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_79 {
    pub futex: C2Rust_Unnamed_83,
    pub nanosleep: C2Rust_Unnamed_81,
    pub poll: C2Rust_Unnamed_80,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_80 {
    pub ufds: *mut pollfd,
    pub nfds: ::core::ffi::c_int,
    pub has_timeout: ::core::ffi::c_int,
    pub end_time: timespec64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_81 {
    pub clockid: clockid_t,
    pub r#type: timespec_type,
    pub c2rust_unnamed: C2Rust_Unnamed_82,
    pub expires: ktime_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_82 {
    pub rmtp: *mut __kernel_timespec,
    pub compat_rmtp: *mut old_timespec32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct old_timespec32 {
    pub tv_sec: old_time32_t,
    pub tv_nsec: s32,
}
pub type old_time32_t = s32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __kernel_timespec {
    pub tv_sec: __kernel_time64_t,
    pub tv_nsec: ::core::ffi::c_longlong,
}
pub type timespec_type = ::core::ffi::c_uint;
pub const TT_COMPAT: timespec_type = 2;
pub const TT_NATIVE: timespec_type = 1;
pub const TT_NONE: timespec_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_83 {
    pub uaddr: *mut u32_0,
    pub val: u32_0,
    pub flags: u32_0,
    pub bitset: u32_0,
    pub time: ktime_t,
    pub uaddr2: *mut u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct task_exec_state {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plist_node {
    pub prio: ::core::ffi::c_int,
    pub prio_list: list_head,
    pub node_list: list_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_info {}
pub type cpumask_t = cpumask;
#[derive(Copy, Clone)]
#[repr(C, align(64))]
pub struct sched_statistics(pub C2Rust_sched_statistics_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_sched_statistics_Inner {}
#[allow(dead_code, non_upper_case_globals)]
const C2Rust_sched_statistics_PADDING: usize = ::core::mem::size_of::<sched_statistics>()
    - ::core::mem::size_of::<C2Rust_sched_statistics_Inner>();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_class {
    _private: [u8; 0],
}
pub type dl_server_pick_f =
    Option<unsafe extern "C" fn(*mut sched_dl_entity, *mut rq_flags) -> *mut task_struct>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rq_flags {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rq {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hrtimer {
    pub node: timerqueue_linked_node,
    pub base: *mut hrtimer_clock_base,
    pub is_queued: bool_0,
    pub is_rel: bool_0,
    pub is_soft: bool_0,
    pub is_hard: bool_0,
    pub is_lazy: bool_0,
    pub _softexpires: ktime_t,
    pub function: Option<unsafe extern "C" fn(*mut hrtimer) -> hrtimer_restart>,
}
pub type hrtimer_restart = ::core::ffi::c_uint;
pub const HRTIMER_RESTART: hrtimer_restart = 1;
pub const HRTIMER_NORESTART: hrtimer_restart = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hrtimer_clock_base {
    pub cpu_base: *mut hrtimer_cpu_base,
    pub index: ::core::ffi::c_uint,
    pub clockid: clockid_t,
    pub seq: seqcount_raw_spinlock_t,
    pub expires_next: ktime_t,
    pub running: *mut hrtimer,
    pub active: timerqueue_linked_head,
    pub offset: ktime_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timerqueue_linked_head {
    pub rb_root: rb_root_linked,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rb_root_linked {
    pub rb_root: rb_root,
    pub rb_leftmost: *mut rb_node_linked,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rb_node_linked {
    pub node: rb_node,
    pub prev: *mut rb_node_linked,
    pub next: *mut rb_node_linked,
}
pub type seqcount_raw_spinlock_t = seqcount_raw_spinlock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqcount_raw_spinlock {
    pub seqcount: seqcount_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hrtimer_cpu_base {
    pub lock: raw_spinlock_t,
    pub cpu: ::core::ffi::c_uint,
    pub active_bases: ::core::ffi::c_uint,
    pub clock_was_set_seq: ::core::ffi::c_uint,
    pub hres_active: bool_0,
    pub deferred_rearm: bool_0,
    pub deferred_needs_update: bool_0,
    pub hang_detected: bool_0,
    pub softirq_activated: bool_0,
    pub online: bool_0,
    pub expires_next: ktime_t,
    pub next_timer: *mut hrtimer,
    pub softirq_expires_next: ktime_t,
    pub softirq_next_timer: *mut hrtimer,
    pub deferred_expires_next: ktime_t,
    pub clock_base: [hrtimer_clock_base; 8],
    pub csd: call_single_data_t,
}
pub type call_single_data_t = __call_single_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __call_single_data {
    pub node: __call_single_node,
    pub func: smp_call_func_t,
    pub info: *mut ::core::ffi::c_void,
}
pub type smp_call_func_t = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __call_single_node {
    pub llist: llist_node,
    pub c2rust_unnamed: C2Rust_Unnamed_84,
    pub src: u16_0,
    pub dst: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_84 {
    pub u_flags: ::core::ffi::c_uint,
    pub a_flags: atomic_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timerqueue_linked_node {
    pub node: rb_node_linked,
    pub expires: ktime_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_rt_entity {
    pub run_list: list_head,
    pub timeout: ::core::ffi::c_ulong,
    pub watchdog_stamp: ::core::ffi::c_ulong,
    pub time_slice: ::core::ffi::c_uint,
    pub on_rq: ::core::ffi::c_ushort,
    pub on_list: ::core::ffi::c_ushort,
    pub back: *mut sched_rt_entity,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_entity {
    pub load: load_weight,
    pub run_node: rb_node,
    pub deadline: u64_0,
    pub min_vruntime: u64_0,
    pub min_slice: u64_0,
    pub max_slice: u64_0,
    pub group_node: list_head,
    pub on_rq: ::core::ffi::c_uchar,
    pub sched_delayed: ::core::ffi::c_uchar,
    pub rel_deadline: ::core::ffi::c_uchar,
    pub custom_slice: ::core::ffi::c_uchar,
    pub exec_start: u64_0,
    pub sum_exec_runtime: u64_0,
    pub prev_sum_exec_runtime: u64_0,
    pub vruntime: u64_0,
    pub vlag: s64,
    pub vprot: u64_0,
    pub slice: u64_0,
    pub nr_migrations: u64_0,
    pub avg: sched_avg,
}
#[derive(Copy, Clone)]
#[repr(C, align(64))]
pub struct sched_avg(pub C2Rust_sched_avg_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_sched_avg_Inner {
    pub last_update_time: u64_0,
    pub load_sum: u64_0,
    pub runnable_sum: u64_0,
    pub util_sum: u32_0,
    pub period_contrib: u32_0,
    pub load_avg: ::core::ffi::c_ulong,
    pub runnable_avg: ::core::ffi::c_ulong,
    pub util_avg: ::core::ffi::c_ulong,
    pub util_est: ::core::ffi::c_uint,
}
#[allow(dead_code, non_upper_case_globals)]
const C2Rust_sched_avg_PADDING: usize =
    ::core::mem::size_of::<sched_avg>() - ::core::mem::size_of::<C2Rust_sched_avg_Inner>();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct load_weight {
    pub weight: ::core::ffi::c_ulong,
    pub inv_weight: u32_0,
}
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
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct va_format {
    pub fmt: *const ::core::ffi::c_char,
    pub va: *mut ::core::ffi::c_void,
}
pub type kunit_assert_type = ::core::ffi::c_uint;
pub const KUNIT_EXPECTATION: kunit_assert_type = 1;
pub const KUNIT_ASSERTION: kunit_assert_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kunit_assert {}
pub type assert_format_t =
    Option<unsafe extern "C" fn(*const kunit_assert, *const va_format, *mut string_stream) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kunit_ptr_not_err_assert {
    pub assert: kunit_assert,
    pub text: *const ::core::ffi::c_char,
    pub value: *const ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kunit_binary_assert_text {
    pub operation: *const ::core::ffi::c_char,
    pub left_text: *const ::core::ffi::c_char,
    pub right_text: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kunit_binary_assert {
    pub assert: kunit_assert,
    pub text: *const kunit_binary_assert_text,
    pub left_value: ::core::ffi::c_longlong,
    pub right_value: ::core::ffi::c_longlong,
}
pub type C2Rust_Unnamed_85 = ::core::ffi::c_uint;
pub const ___GFP_LAST_BIT: C2Rust_Unnamed_85 = 25;
pub const ___GFP_NO_OBJ_EXT_BIT: C2Rust_Unnamed_85 = 24;
pub const ___GFP_ZEROTAGS_BIT: C2Rust_Unnamed_85 = 23;
pub const ___GFP_ACCOUNT_BIT: C2Rust_Unnamed_85 = 22;
pub const ___GFP_THISNODE_BIT: C2Rust_Unnamed_85 = 21;
pub const ___GFP_HARDWALL_BIT: C2Rust_Unnamed_85 = 20;
pub const ___GFP_NOMEMALLOC_BIT: C2Rust_Unnamed_85 = 19;
pub const ___GFP_COMP_BIT: C2Rust_Unnamed_85 = 18;
pub const ___GFP_MEMALLOC_BIT: C2Rust_Unnamed_85 = 17;
pub const ___GFP_NORETRY_BIT: C2Rust_Unnamed_85 = 16;
pub const ___GFP_NOFAIL_BIT: C2Rust_Unnamed_85 = 15;
pub const ___GFP_RETRY_MAYFAIL_BIT: C2Rust_Unnamed_85 = 14;
pub const ___GFP_NOWARN_BIT: C2Rust_Unnamed_85 = 13;
pub const ___GFP_WRITE_BIT: C2Rust_Unnamed_85 = 12;
pub const ___GFP_KSWAPD_RECLAIM_BIT: C2Rust_Unnamed_85 = 11;
pub const ___GFP_DIRECT_RECLAIM_BIT: C2Rust_Unnamed_85 = 10;
pub const ___GFP_UNUSED_BIT: C2Rust_Unnamed_85 = 9;
pub const ___GFP_ZERO_BIT: C2Rust_Unnamed_85 = 8;
pub const ___GFP_FS_BIT: C2Rust_Unnamed_85 = 7;
pub const ___GFP_IO_BIT: C2Rust_Unnamed_85 = 6;
pub const ___GFP_HIGH_BIT: C2Rust_Unnamed_85 = 5;
pub const ___GFP_RECLAIMABLE_BIT: C2Rust_Unnamed_85 = 4;
pub const ___GFP_MOVABLE_BIT: C2Rust_Unnamed_85 = 3;
pub const ___GFP_DMA32_BIT: C2Rust_Unnamed_85 = 2;
pub const ___GFP_HIGHMEM_BIT: C2Rust_Unnamed_85 = 1;
pub const ___GFP_DMA_BIT: C2Rust_Unnamed_85 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline(always)]
unsafe extern "C" fn IS_ERR_OR_NULL(mut ptr: *const ::core::ffi::c_void) -> bool_0 {
    unsafe {
        return ptr.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0
            || (::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                ptr.expose_provenance() as ::core::ffi::c_ulong as usize,
            )
            .expose_provenance() as ::core::ffi::c_ulong
                >= -4095 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                as ::core::ffi::c_int as ::core::ffi::c_long
                != 0;
    }
}
pub const KUNIT_CURRENT_LOC: kunit_loc = kunit_loc {
    line: 45 as ::core::ffi::c_int,
    file: b"lib/tests/test_sort.c\0".as_ptr() as *const ::core::ffi::c_char,
};
pub const TEST_LEN: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
unsafe extern "C" fn cmpint(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        return *(a as *mut ::core::ffi::c_int) - *(b as *mut ::core::ffi::c_int);
    }
}
unsafe extern "C" fn test_sort(mut test: *mut kunit) {
    unsafe {
        let mut a: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
        let mut i: ::core::ffi::c_int = 0;
        let mut r: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        a = kunit_kmalloc_array(
            test,
            TEST_LEN as size_t,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            ((1 as ::core::ffi::c_ulong) << ___GFP_DIRECT_RECLAIM_BIT as ::core::ffi::c_int
                | (1 as ::core::ffi::c_ulong) << ___GFP_KSWAPD_RECLAIM_BIT as ::core::ffi::c_int)
                as gfp_t
                | ((1 as ::core::ffi::c_ulong) << ___GFP_IO_BIT as ::core::ffi::c_int) as gfp_t
                | ((1 as ::core::ffi::c_ulong) << ___GFP_FS_BIT as ::core::ffi::c_int) as gfp_t,
        ) as *mut ::core::ffi::c_int;
        let __ptr: *mut ::core::ffi::c_int = a as *mut ::core::ffi::c_int;
        extern "C" {
            #[link_name = "__compiletime_assert_316"]
            fn __compiletime_assert_316_0() -> !;
        }
        if !(::core::mem::size_of::<*const ::core::ffi::c_char>()
            == ::core::mem::size_of::<::core::ffi::c_char>()
            || ::core::mem::size_of::<*const ::core::ffi::c_char>()
                == ::core::mem::size_of::<::core::ffi::c_short>()
            || ::core::mem::size_of::<*const ::core::ffi::c_char>()
                == ::core::mem::size_of::<::core::ffi::c_int>()
            || ::core::mem::size_of::<*const ::core::ffi::c_char>()
                == ::core::mem::size_of::<::core::ffi::c_long>()
            || ::core::mem::size_of::<*const ::core::ffi::c_char>()
                == ::core::mem::size_of::<::core::ffi::c_longlong>())
        {
            __compiletime_assert_316_0();
        }
        ::core::ptr::write_volatile(
            (&raw mut (*test).last_seen.file as *mut *const ::core::ffi::c_char),
            b"lib/tests/test_sort.c\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_char,
        );
        extern "C" {
            #[link_name = "__compiletime_assert_317"]
            fn __compiletime_assert_317_0() -> !;
        }
        if !(::core::mem::size_of::<::core::ffi::c_int>()
            == ::core::mem::size_of::<::core::ffi::c_char>()
            || ::core::mem::size_of::<::core::ffi::c_int>()
                == ::core::mem::size_of::<::core::ffi::c_short>()
            || ::core::mem::size_of::<::core::ffi::c_int>()
                == ::core::mem::size_of::<::core::ffi::c_int>()
            || ::core::mem::size_of::<::core::ffi::c_int>()
                == ::core::mem::size_of::<::core::ffi::c_long>()
            || ::core::mem::size_of::<::core::ffi::c_int>()
                == ::core::mem::size_of::<::core::ffi::c_longlong>())
        {
            __compiletime_assert_317_0();
        }
        ::core::ptr::write_volatile(
            (&raw mut (*test).last_seen.line as *mut ::core::ffi::c_int),
            23 as ::core::ffi::c_int as ::core::ffi::c_int,
        );
        if IS_ERR_OR_NULL(__ptr as *const ::core::ffi::c_void) {
            static mut __loc: kunit_loc = KUNIT_CURRENT_LOC;
            let __assertion: kunit_ptr_not_err_assert = kunit_ptr_not_err_assert {
                assert: kunit_assert {},
                text: b"a\0".as_ptr() as *const ::core::ffi::c_char,
                value: __ptr as *const ::core::ffi::c_void,
            };
            __kunit_do_failed_assertion(
                test,
                &raw const __loc,
                KUNIT_ASSERTION,
                &raw const __assertion.assert,
                Some(
                    kunit_ptr_not_err_assert_format
                        as unsafe extern "C" fn(
                            *const kunit_assert,
                            *const va_format,
                            *mut string_stream,
                        ) -> (),
                ),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            if KUNIT_ASSERTION as ::core::ffi::c_int == KUNIT_ASSERTION as ::core::ffi::c_int {
                __kunit_abort(test);
            }
        }
        i = 0 as ::core::ffi::c_int;
        while i < TEST_LEN {
            r = r.wrapping_mul(725861 as ::core::ffi::c_int) % 6599 as ::core::ffi::c_int;
            *a.offset(i as isize) = r;
            i += 1;
        }
        sort(
            a as *mut ::core::ffi::c_void,
            TEST_LEN as size_t,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            Some(
                cmpint
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            None,
        );
        i = 0 as ::core::ffi::c_int;
        while i < TEST_LEN - 1 as ::core::ffi::c_int {
            let __left: ::core::ffi::c_int = *a.offset(i as isize) as ::core::ffi::c_int;
            let __right: ::core::ffi::c_int =
                *a.offset((i + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int;
            static mut __text: kunit_binary_assert_text = kunit_binary_assert_text {
                operation: b"<=\0".as_ptr() as *const ::core::ffi::c_char,
                left_text: b"a[i]\0".as_ptr() as *const ::core::ffi::c_char,
                right_text: b"a[i + 1]\0".as_ptr() as *const ::core::ffi::c_char,
            };
            extern "C" {
                #[link_name = "__compiletime_assert_318"]
                fn __compiletime_assert_318_0() -> !;
            }
            if !(::core::mem::size_of::<*const ::core::ffi::c_char>()
                == ::core::mem::size_of::<::core::ffi::c_char>()
                || ::core::mem::size_of::<*const ::core::ffi::c_char>()
                    == ::core::mem::size_of::<::core::ffi::c_short>()
                || ::core::mem::size_of::<*const ::core::ffi::c_char>()
                    == ::core::mem::size_of::<::core::ffi::c_int>()
                || ::core::mem::size_of::<*const ::core::ffi::c_char>()
                    == ::core::mem::size_of::<::core::ffi::c_long>()
                || ::core::mem::size_of::<*const ::core::ffi::c_char>()
                    == ::core::mem::size_of::<::core::ffi::c_longlong>())
            {
                __compiletime_assert_318_0();
            }
            ::core::ptr::write_volatile(
                (&raw mut (*test).last_seen.file as *mut *const ::core::ffi::c_char),
                b"lib/tests/test_sort.c\0".as_ptr() as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_char,
            );
            extern "C" {
                #[link_name = "__compiletime_assert_319"]
                fn __compiletime_assert_319_0() -> !;
            }
            if !(::core::mem::size_of::<::core::ffi::c_int>()
                == ::core::mem::size_of::<::core::ffi::c_char>()
                || ::core::mem::size_of::<::core::ffi::c_int>()
                    == ::core::mem::size_of::<::core::ffi::c_short>()
                || ::core::mem::size_of::<::core::ffi::c_int>()
                    == ::core::mem::size_of::<::core::ffi::c_int>()
                || ::core::mem::size_of::<::core::ffi::c_int>()
                    == ::core::mem::size_of::<::core::ffi::c_long>()
                || ::core::mem::size_of::<::core::ffi::c_int>()
                    == ::core::mem::size_of::<::core::ffi::c_longlong>())
            {
                __compiletime_assert_319_0();
            }
            ::core::ptr::write_volatile(
                (&raw mut (*test).last_seen.line as *mut ::core::ffi::c_int),
                33 as ::core::ffi::c_int as ::core::ffi::c_int,
            );
            if (__left <= __right) as ::core::ffi::c_int as ::core::ffi::c_long == 0 {
                static mut __loc_0: kunit_loc = KUNIT_CURRENT_LOC;
                let __assertion_0: kunit_binary_assert = kunit_binary_assert {
                    assert: kunit_assert {},
                    text: &raw const __text,
                    left_value: __left as ::core::ffi::c_longlong,
                    right_value: __right as ::core::ffi::c_longlong,
                };
                __kunit_do_failed_assertion(
                    test,
                    &raw const __loc_0,
                    KUNIT_ASSERTION,
                    &raw const __assertion_0.assert,
                    Some(
                        kunit_binary_assert_format
                            as unsafe extern "C" fn(
                                *const kunit_assert,
                                *const va_format,
                                *mut string_stream,
                            ) -> (),
                    ),
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
                if KUNIT_ASSERTION as ::core::ffi::c_int == KUNIT_ASSERTION as ::core::ffi::c_int {
                    __kunit_abort(test);
                }
            }
            i += 1;
        }
        r = 48 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < TEST_LEN - 1 as ::core::ffi::c_int {
            r = r.wrapping_mul(725861 as ::core::ffi::c_int) % 6599 as ::core::ffi::c_int;
            *a.offset(i as isize) = r;
            i += 1;
        }
        sort(
            a as *mut ::core::ffi::c_void,
            (TEST_LEN - 1 as ::core::ffi::c_int) as size_t,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            Some(
                cmpint
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            None,
        );
        i = 0 as ::core::ffi::c_int;
        while i < TEST_LEN - 2 as ::core::ffi::c_int {
            let __left_0: ::core::ffi::c_int = *a.offset(i as isize) as ::core::ffi::c_int;
            let __right_0: ::core::ffi::c_int =
                *a.offset((i + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int;
            static mut __text_0: kunit_binary_assert_text = kunit_binary_assert_text {
                operation: b"<=\0".as_ptr() as *const ::core::ffi::c_char,
                left_text: b"a[i]\0".as_ptr() as *const ::core::ffi::c_char,
                right_text: b"a[i + 1]\0".as_ptr() as *const ::core::ffi::c_char,
            };
            extern "C" {
                #[link_name = "__compiletime_assert_320"]
                fn __compiletime_assert_320_0() -> !;
            }
            if !(::core::mem::size_of::<*const ::core::ffi::c_char>()
                == ::core::mem::size_of::<::core::ffi::c_char>()
                || ::core::mem::size_of::<*const ::core::ffi::c_char>()
                    == ::core::mem::size_of::<::core::ffi::c_short>()
                || ::core::mem::size_of::<*const ::core::ffi::c_char>()
                    == ::core::mem::size_of::<::core::ffi::c_int>()
                || ::core::mem::size_of::<*const ::core::ffi::c_char>()
                    == ::core::mem::size_of::<::core::ffi::c_long>()
                || ::core::mem::size_of::<*const ::core::ffi::c_char>()
                    == ::core::mem::size_of::<::core::ffi::c_longlong>())
            {
                __compiletime_assert_320_0();
            }
            ::core::ptr::write_volatile(
                (&raw mut (*test).last_seen.file as *mut *const ::core::ffi::c_char),
                b"lib/tests/test_sort.c\0".as_ptr() as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_char,
            );
            extern "C" {
                #[link_name = "__compiletime_assert_321"]
                fn __compiletime_assert_321_0() -> !;
            }
            if !(::core::mem::size_of::<::core::ffi::c_int>()
                == ::core::mem::size_of::<::core::ffi::c_char>()
                || ::core::mem::size_of::<::core::ffi::c_int>()
                    == ::core::mem::size_of::<::core::ffi::c_short>()
                || ::core::mem::size_of::<::core::ffi::c_int>()
                    == ::core::mem::size_of::<::core::ffi::c_int>()
                || ::core::mem::size_of::<::core::ffi::c_int>()
                    == ::core::mem::size_of::<::core::ffi::c_long>()
                || ::core::mem::size_of::<::core::ffi::c_int>()
                    == ::core::mem::size_of::<::core::ffi::c_longlong>())
            {
                __compiletime_assert_321_0();
            }
            ::core::ptr::write_volatile(
                (&raw mut (*test).last_seen.line as *mut ::core::ffi::c_int),
                45 as ::core::ffi::c_int as ::core::ffi::c_int,
            );
            if (__left_0 <= __right_0) as ::core::ffi::c_int as ::core::ffi::c_long == 0 {
                static mut __loc_1: kunit_loc = KUNIT_CURRENT_LOC;
                let __assertion_1: kunit_binary_assert = kunit_binary_assert {
                    assert: kunit_assert {},
                    text: &raw const __text_0,
                    left_value: __left_0 as ::core::ffi::c_longlong,
                    right_value: __right_0 as ::core::ffi::c_longlong,
                };
                __kunit_do_failed_assertion(
                    test,
                    &raw const __loc_1,
                    KUNIT_ASSERTION,
                    &raw const __assertion_1.assert,
                    Some(
                        kunit_binary_assert_format
                            as unsafe extern "C" fn(
                                *const kunit_assert,
                                *const va_format,
                                *mut string_stream,
                            ) -> (),
                    ),
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
                if KUNIT_ASSERTION as ::core::ffi::c_int == KUNIT_ASSERTION as ::core::ffi::c_int {
                    __kunit_abort(test);
                }
            }
            i += 1;
        }
    }
}
static mut sort_test_cases: [kunit_case; 2] = [
    kunit_case {
        run_case: Some(test_sort as unsafe extern "C" fn(*mut kunit) -> ()),
        name: b"test_sort\0".as_ptr() as *const ::core::ffi::c_char,
        generate_params: None,
        attr: kunit_attributes {
            speed: KUNIT_SPEED_UNSET,
        },
        param_init: None,
        param_exit: None,
        status: KUNIT_SUCCESS,
        module_name: KBUILD_MODNAME.as_ptr() as *mut ::core::ffi::c_char,
        log: ::core::ptr::null_mut::<string_stream>(),
    },
    kunit_case {
        run_case: None,
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        generate_params: None,
        attr: kunit_attributes {
            speed: KUNIT_SPEED_UNSET,
        },
        param_init: None,
        param_exit: None,
        status: KUNIT_SUCCESS,
        module_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        log: ::core::ptr::null_mut::<string_stream>(),
    },
];
static mut sort_test_suite: kunit_suite = unsafe {
    kunit_suite {
        name: ::core::mem::transmute::<
            [u8; 256],
            [::core::ffi::c_char; 256],
        >(
            *b"lib_sort\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        suite_init: None,
        suite_exit: None,
        init: None,
        exit: None,
        test_cases: &raw const sort_test_cases as *mut kunit_case,
        attr: kunit_attributes {
            speed: KUNIT_SPEED_UNSET,
        },
        status_comment: [0; 256],
        debugfs: ::core::ptr::null_mut::<dentry>(),
        log: ::core::ptr::null_mut::<string_stream>(),
        suite_init_err: 0,
        is_init: false,
    }
};
#[used]
#[link_section = ".kunit_test_suites"]
static mut __UNIQUE_ID_array_322: [*mut kunit_suite; 1] =
    unsafe { [&raw const sort_test_suite as *mut kunit_suite] };
#[used]
#[link_section = ".modinfo"]
static mut __UNIQUE_ID_modinfo_323: [::core::ffi::c_char; 46] = unsafe {
    ::core::mem::transmute::<[u8; 46], [::core::ffi::c_char; 46]>(
        *b"test_sort.description=sort() KUnit test suite\0",
    )
};
#[used]
#[link_section = ".modinfo"]
static mut __UNIQUE_ID_modinfo_324: [::core::ffi::c_char; 35] = unsafe {
    ::core::mem::transmute::<[u8; 35], [::core::ffi::c_char; 35]>(
        *b"test_sort.file=lib/tests/test_sort\0",
    )
};
#[used]
#[link_section = ".modinfo"]
static mut __UNIQUE_ID_modinfo_325: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"test_sort.license=GPL\0")
};
pub const KBUILD_MODNAME: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"test_sort\0") };
