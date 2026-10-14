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
use ::core::arch::{asm, global_asm};
use ::kernel::warn_on;
extern "C" {
    fn __riscv_isa_extension_available(
        isa_bitmap: *const ::core::ffi::c_ulong,
        bit: ::core::ffi::c_uint,
    ) -> bool_0;
    fn __riscv_has_extension_likely(
        vendor: ::core::ffi::c_ulong,
        ext: ::core::ffi::c_ulong,
    ) -> bool_0;
    fn __riscv_has_extension_unlikely(
        vendor: ::core::ffi::c_ulong,
        ext: ::core::ffi::c_ulong,
    ) -> bool_0;
    fn refcount_warn_saturate(r: *mut refcount_t, t: refcount_saturation_type);
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn kfree(objp: *const ::core::ffi::c_void);
    static mut kmalloc_caches: [kmem_buckets; 1];
    fn __kmalloc_noprof(size: size_t, flags: gfp_t) -> *mut ::core::ffi::c_void;
    fn __kmalloc_node_noprof(
        size: size_t,
        flags: gfp_t,
        node: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn __kmalloc_cache_noprof(
        s: *mut kmem_cache,
        flags: gfp_t,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn __kmalloc_cache_node_noprof(
        s: *mut kmem_cache,
        gfpflags: gfp_t,
        node: ::core::ffi::c_int,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn __kmalloc_large_noprof(size: size_t, flags: gfp_t) -> *mut ::core::ffi::c_void;
    fn __kmalloc_large_node_noprof(
        size: size_t,
        flags: gfp_t,
        node: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn kvfree(addr: *const ::core::ffi::c_void);
    fn __vmalloc_node_noprof(
        size: ::core::ffi::c_ulong,
        align: ::core::ffi::c_ulong,
        gfp_mask: gfp_t,
        node: ::core::ffi::c_int,
        caller: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
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
pub type uint32_t = u32;
pub type ktime_t = s64;
pub type gfp_t = ::core::ffi::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
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
    _private: [u8; 0],
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
    pub c2rust_unnamed: C2Rust_Unnamed_7,
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
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_7 {
    pub lru: list_head,
    pub buddy_list: list_head,
    pub pcp_list: list_head,
    pub pcp_llist: llist_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct llist_node {
    pub next: *mut llist_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct memdesc_flags_t {
    pub f: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pipe_inode_info {
    _private: [u8; 0],
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
pub type raw_spinlock_t = raw_spinlock;
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
    pub c2rust_unnamed: C2Rust_Unnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
    pub si_signo: ::core::ffi::c_int,
    pub si_errno: ::core::ffi::c_int,
    pub si_code: ::core::ffi::c_int,
    pub _sifields: __sifields,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __sifields {
    pub _kill: C2Rust_Unnamed_19,
    pub _timer: C2Rust_Unnamed_18,
    pub _rt: C2Rust_Unnamed_17,
    pub _sigchld: C2Rust_Unnamed_16,
    pub _sigfault: C2Rust_Unnamed_11,
    pub _sigpoll: C2Rust_Unnamed_10,
    pub _sigsys: C2Rust_Unnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
    pub _call_addr: *mut ::core::ffi::c_void,
    pub _syscall: ::core::ffi::c_int,
    pub _arch: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub _band: ::core::ffi::c_long,
    pub _fd: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
    pub _addr: *mut ::core::ffi::c_void,
    pub c2rust_unnamed: C2Rust_Unnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_12 {
    pub _trapno: ::core::ffi::c_int,
    pub _addr_lsb: ::core::ffi::c_short,
    pub _addr_bnd: C2Rust_Unnamed_15,
    pub _addr_pkey: C2Rust_Unnamed_14,
    pub _perf: C2Rust_Unnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
    pub _data: ::core::ffi::c_ulong,
    pub _type: __u32,
    pub _flags: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub _dummy_pkey: [::core::ffi::c_char; 8],
    pub _pkey: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub _dummy_bnd: [::core::ffi::c_char; 8],
    pub _lower: *mut ::core::ffi::c_void,
    pub _upper: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
    pub _status: ::core::ffi::c_int,
    pub _utime: __kernel_clock_t,
    pub _stime: __kernel_clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
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
pub struct C2Rust_Unnamed_18 {
    pub _tid: __kernel_timer_t,
    pub _overrun: ::core::ffi::c_int,
    pub _sigval: sigval_t,
    pub _sys_private: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct io_context {
    _private: [u8; 0],
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
pub type spinlock_t = spinlock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spinlock {
    pub c2rust_unnamed: C2Rust_Unnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_20 {
    pub rlock: raw_spinlock,
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
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct signal_struct {
    _private: [u8; 0],
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
pub struct cred {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_cputimers {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prev_cputime {
    pub utime: u64_0,
    pub stime: u64_0,
    pub lock: raw_spinlock_t,
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
pub struct pid {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct restart_block {
    pub arch_data: ::core::ffi::c_ulong,
    pub r#fn: Option<unsafe extern "C" fn(*mut restart_block) -> ::core::ffi::c_long>,
    pub c2rust_unnamed: C2Rust_Unnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_21 {
    pub futex: C2Rust_Unnamed_25,
    pub nanosleep: C2Rust_Unnamed_23,
    pub poll: C2Rust_Unnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub ufds: *mut pollfd,
    pub nfds: ::core::ffi::c_int,
    pub has_timeout: ::core::ffi::c_int,
    pub end_time: timespec64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec64 {
    pub tv_sec: time64_t,
    pub tv_nsec: ::core::ffi::c_long,
}
pub type time64_t = __s64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub clockid: clockid_t,
    pub r#type: timespec_type,
    pub c2rust_unnamed: C2Rust_Unnamed_24,
    pub expires: ktime_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_24 {
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
pub struct C2Rust_Unnamed_25 {
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
pub struct work_struct {
    pub data: atomic_long_t,
    pub entry: list_head,
    pub func: work_func_t,
}
pub type work_func_t = Option<unsafe extern "C" fn(*mut work_struct) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uprobes_state {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file {
    _private: [u8; 0],
}
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
pub struct vma_flags_t {
    pub __vma_flags: [::core::ffi::c_ulong; 1],
}
pub type vm_flags_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct futex_mm_data {}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_dl_entity {
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
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timerqueue_linked_node {
    pub node: rb_node_linked,
    pub expires: ktime_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rb_node_linked {
    pub node: rb_node,
    pub prev: *mut rb_node_linked,
    pub next: *mut rb_node_linked,
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
pub struct __call_single_node {
    pub llist: llist_node,
    pub c2rust_unnamed: C2Rust_Unnamed_30,
    pub src: u16_0,
    pub dst: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_30 {
    pub u_flags: ::core::ffi::c_uint,
    pub a_flags: atomic_t,
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
#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct C2Rust_Unnamed_31(pub C2Rust_C2Rust_Unnamed_31_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_C2Rust_Unnamed_31_Inner {
    pub x: ::core::ffi::c_long,
}
#[allow(dead_code, non_upper_case_globals)]
const C2Rust_C2Rust_Unnamed_31_PADDING: usize = ::core::mem::size_of::<C2Rust_Unnamed_31>()
    - ::core::mem::size_of::<C2Rust_C2Rust_Unnamed_31_Inner>();
pub type refcount_saturation_type = ::core::ffi::c_uint;
pub const REFCOUNT_DEC_LEAK: refcount_saturation_type = 4;
pub const REFCOUNT_SUB_UAF: refcount_saturation_type = 3;
pub const REFCOUNT_ADD_UAF: refcount_saturation_type = 2;
pub const REFCOUNT_ADD_OVF: refcount_saturation_type = 1;
pub const REFCOUNT_ADD_NOT_ZERO_OVF: refcount_saturation_type = 0;
pub type C2Rust_Unnamed_32 = ::core::ffi::c_uint;
pub const ___GFP_LAST_BIT: C2Rust_Unnamed_32 = 25;
pub const ___GFP_NO_OBJ_EXT_BIT: C2Rust_Unnamed_32 = 24;
pub const ___GFP_ZEROTAGS_BIT: C2Rust_Unnamed_32 = 23;
pub const ___GFP_ACCOUNT_BIT: C2Rust_Unnamed_32 = 22;
pub const ___GFP_THISNODE_BIT: C2Rust_Unnamed_32 = 21;
pub const ___GFP_HARDWALL_BIT: C2Rust_Unnamed_32 = 20;
pub const ___GFP_NOMEMALLOC_BIT: C2Rust_Unnamed_32 = 19;
pub const ___GFP_COMP_BIT: C2Rust_Unnamed_32 = 18;
pub const ___GFP_MEMALLOC_BIT: C2Rust_Unnamed_32 = 17;
pub const ___GFP_NORETRY_BIT: C2Rust_Unnamed_32 = 16;
pub const ___GFP_NOFAIL_BIT: C2Rust_Unnamed_32 = 15;
pub const ___GFP_RETRY_MAYFAIL_BIT: C2Rust_Unnamed_32 = 14;
pub const ___GFP_NOWARN_BIT: C2Rust_Unnamed_32 = 13;
pub const ___GFP_WRITE_BIT: C2Rust_Unnamed_32 = 12;
pub const ___GFP_KSWAPD_RECLAIM_BIT: C2Rust_Unnamed_32 = 11;
pub const ___GFP_DIRECT_RECLAIM_BIT: C2Rust_Unnamed_32 = 10;
pub const ___GFP_UNUSED_BIT: C2Rust_Unnamed_32 = 9;
pub const ___GFP_ZERO_BIT: C2Rust_Unnamed_32 = 8;
pub const ___GFP_FS_BIT: C2Rust_Unnamed_32 = 7;
pub const ___GFP_IO_BIT: C2Rust_Unnamed_32 = 6;
pub const ___GFP_HIGH_BIT: C2Rust_Unnamed_32 = 5;
pub const ___GFP_RECLAIMABLE_BIT: C2Rust_Unnamed_32 = 4;
pub const ___GFP_MOVABLE_BIT: C2Rust_Unnamed_32 = 3;
pub const ___GFP_DMA32_BIT: C2Rust_Unnamed_32 = 2;
pub const ___GFP_HIGHMEM_BIT: C2Rust_Unnamed_32 = 1;
pub const ___GFP_DMA_BIT: C2Rust_Unnamed_32 = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct objpool_slot {
    pub head: uint32_t,
    pub tail: uint32_t,
    pub last: uint32_t,
    pub mask: uint32_t,
    pub entries: [*mut ::core::ffi::c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct objpool_head {
    pub obj_size: ::core::ffi::c_int,
    pub nr_objs: ::core::ffi::c_int,
    pub nr_possible_cpus: ::core::ffi::c_int,
    pub capacity: ::core::ffi::c_int,
    pub gfp: gfp_t,
    pub r#ref: refcount_t,
    pub flags: ::core::ffi::c_ulong,
    pub cpu_slots: *mut *mut objpool_slot,
    pub release: objpool_fini_cb,
    pub context: *mut ::core::ffi::c_void,
}
pub type objpool_fini_cb =
    Option<unsafe extern "C" fn(*mut objpool_head, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
pub type objpool_init_obj_cb = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmalloc_token_t {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmem_cache {
    _private: [u8; 0],
}
pub type kmem_buckets = [*mut kmem_cache; 14];
pub type kmalloc_cache_type = ::core::ffi::c_uint;
pub const NR_KMALLOC_TYPES: kmalloc_cache_type = 1;
pub const KMALLOC_RECLAIM: kmalloc_cache_type = 0;
pub const KMALLOC_PARTITION_END: kmalloc_cache_type = 0;
pub const KMALLOC_PARTITION_START: kmalloc_cache_type = 0;
pub const KMALLOC_CGROUP: kmalloc_cache_type = 0;
pub const KMALLOC_DMA: kmalloc_cache_type = 0;
pub const KMALLOC_NORMAL: kmalloc_cache_type = 0;
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct alloc_tag(pub C2Rust_alloc_tag_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_alloc_tag_Inner {
    pub ct: codetag,
    pub counters: *mut alloc_tag_counters,
}
#[allow(dead_code, non_upper_case_globals)]
const C2Rust_alloc_tag_PADDING: usize =
    ::core::mem::size_of::<alloc_tag>() - ::core::mem::size_of::<C2Rust_alloc_tag_Inner>();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alloc_tag_counters {
    pub bytes: u64_0,
    pub calls: u64_0,
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct codetag(pub C2Rust_codetag_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_codetag_Inner {
    pub flags: ::core::ffi::c_uint,
    pub lineno: ::core::ffi::c_uint,
    pub modname: *const ::core::ffi::c_char,
    pub function: *const ::core::ffi::c_char,
    pub filename: *const ::core::ffi::c_char,
}
#[allow(dead_code, non_upper_case_globals)]
const C2Rust_codetag_PADDING: usize =
    ::core::mem::size_of::<codetag>() - ::core::mem::size_of::<C2Rust_codetag_Inner>();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_33 {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_34 {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_35 {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_36 {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_37 {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_38 {}
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
pub const SIZE_MAX: size_t = !(0 as ::core::ffi::c_int as size_t);
#[inline(always)]
unsafe extern "C" fn __must_check_overflow(mut overflow: bool_0) -> bool_0 {
    unsafe {
        return overflow as ::core::ffi::c_int as ::core::ffi::c_long != 0;
    }
}
#[inline(always)]
unsafe extern "C" fn size_mul(mut factor1: size_t, mut factor2: size_t) -> size_t {
    unsafe {
        let mut bytes: size_t = 0;
        let (c2rust_result, c2rust_overflowed) = factor1.overflowing_mul(factor2);
        *&raw mut bytes = c2rust_result;
        if __must_check_overflow(c2rust_overflowed) {
            return SIZE_MAX;
        }
        return bytes;
    }
}
#[inline(always)]
unsafe extern "C" fn size_add(mut addend1: size_t, mut addend2: size_t) -> size_t {
    unsafe {
        let mut bytes: size_t = 0;
        let (c2rust_result, c2rust_overflowed) = addend1.overflowing_add(addend2);
        *&raw mut bytes = c2rust_result;
        if __must_check_overflow(c2rust_overflowed) {
            return SIZE_MAX;
        }
        return bytes;
    }
}
pub const RISCV_ISA_EXT_ZBB: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const RISCV_ISA_EXT_ZACAS: ::core::ffi::c_int = 73 as ::core::ffi::c_int;
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
unsafe extern "C" fn riscv_has_extension_likely(ext: ::core::ffi::c_ulong) -> bool_0 {
    unsafe {
        extern "C" {
            #[link_name = "__compiletime_assert_1"]
            fn __compiletime_assert_1_0() -> !;
        }
        if !(ext < 128 as ::core::ffi::c_ulong) {
            __compiletime_assert_1_0();
        }
        return __riscv_has_extension_likely(STANDARD_EXT as ::core::ffi::c_ulong, ext);
    }
}
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn arch_atomic_set(mut v: *mut atomic_t, mut i: ::core::ffi::c_int) {
    unsafe {
        extern "C" {
            #[link_name = "__compiletime_assert_4"]
            fn __compiletime_assert_4_0() -> !;
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
            __compiletime_assert_4_0();
        }
        ::core::ptr::write_volatile(
            (&raw mut (*v).counter as *mut ::core::ffi::c_int),
            i as ::core::ffi::c_int,
        );
    }
}
#[inline(always)]
unsafe extern "C" fn arch_atomic_fetch_sub_relaxed(
    mut i: ::core::ffi::c_int,
    mut v: *mut atomic_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ret: ::core::ffi::c_int = 0;
        asm!(
            "\tamoadd.w {1}, {2}, 0({0})\n", in (reg) & raw mut (* v).counter,
            lateout(reg) ret, inlateout(reg) - i => _, options(preserves_flags)
        );
        return ret;
    }
}
#[inline(always)]
unsafe extern "C" fn raw_atomic_set(mut v: *mut atomic_t, mut i: ::core::ffi::c_int) {
    unsafe {
        arch_atomic_set(v, i);
    }
}
#[inline(always)]
unsafe extern "C" fn raw_atomic_fetch_sub_release(
    mut i: ::core::ffi::c_int,
    mut v: *mut atomic_t,
) -> ::core::ffi::c_int {
    unsafe {
        asm!("\n", options(preserves_flags));
        return arch_atomic_fetch_sub_relaxed(i, v);
    }
}
#[inline(always)]
unsafe extern "C" fn instrument_read_write(mut v: *const ::core::ffi::c_void, mut size: size_t) {
    unsafe {
        kasan_check_write(v, size as ::core::ffi::c_uint);
        kcsan_check_access(v, size, KCSAN_ACCESS_COMPOUND | KCSAN_ACCESS_WRITE);
    }
}
#[inline(always)]
unsafe extern "C" fn instrument_atomic_check_alignment(
    mut v: *const ::core::ffi::c_void,
    mut size: size_t,
) {
}
#[inline(always)]
unsafe extern "C" fn instrument_atomic_write(mut v: *const ::core::ffi::c_void, mut size: size_t) {
    unsafe {
        kasan_check_write(v, size as ::core::ffi::c_uint);
        kcsan_check_access(
            ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                ((v as *mut ::core::ffi::c_void).expose_provenance() as ::core::ffi::c_ulong)
                    .wrapping_add(0 as ::core::ffi::c_ulong) as usize,
            ) as *const ::core::ffi::c_void,
            size,
            KCSAN_ACCESS_ATOMIC | KCSAN_ACCESS_WRITE,
        );
        instrument_atomic_check_alignment(v, size);
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
#[inline(always)]
unsafe extern "C" fn atomic_set(mut v: *mut atomic_t, mut i: ::core::ffi::c_int) {
    unsafe {
        instrument_atomic_write(
            v as *const ::core::ffi::c_void,
            ::core::mem::size_of::<atomic_t>() as size_t,
        );
        raw_atomic_set(v, i);
    }
}
#[inline(always)]
unsafe extern "C" fn atomic_fetch_sub_release(
    mut i: ::core::ffi::c_int,
    mut v: *mut atomic_t,
) -> ::core::ffi::c_int {
    unsafe {
        instrument_atomic_read_write(
            v as *const ::core::ffi::c_void,
            ::core::mem::size_of::<atomic_t>() as size_t,
        );
        return raw_atomic_fetch_sub_release(i, v);
    }
}
#[inline]
unsafe extern "C" fn refcount_set(mut r: *mut refcount_t, mut n: ::core::ffi::c_int) {
    unsafe {
        atomic_set(&raw mut (*r).refs, n);
    }
}
#[inline]
unsafe extern "C" fn __refcount_sub_and_test(
    mut i: ::core::ffi::c_int,
    mut r: *mut refcount_t,
    mut oldp: *mut ::core::ffi::c_int,
) -> bool_0 {
    unsafe {
        let mut old: ::core::ffi::c_int = atomic_fetch_sub_release(i, &raw mut (*r).refs);
        if !oldp.is_null() {
            *oldp = old;
        }
        if old > 0 as ::core::ffi::c_int && old == i {
            asm!("\n", options(preserves_flags));
            return r#true as ::core::ffi::c_int != 0;
        }
        if (old <= 0 as ::core::ffi::c_int || old - i < 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            refcount_warn_saturate(r, REFCOUNT_SUB_UAF);
        }
        return r#false as ::core::ffi::c_int != 0;
    }
}
#[inline]
unsafe extern "C" fn refcount_sub_and_test(
    mut i: ::core::ffi::c_int,
    mut r: *mut refcount_t,
) -> bool_0 {
    unsafe {
        return __refcount_sub_and_test(i, r, ::core::ptr::null_mut::<::core::ffi::c_int>());
    }
}
#[inline]
unsafe extern "C" fn __refcount_dec_and_test(
    mut r: *mut refcount_t,
    mut oldp: *mut ::core::ffi::c_int,
) -> bool_0 {
    unsafe {
        return __refcount_sub_and_test(1 as ::core::ffi::c_int, r, oldp);
    }
}
#[inline]
unsafe extern "C" fn refcount_dec_and_test(mut r: *mut refcount_t) -> bool_0 {
    unsafe {
        return __refcount_dec_and_test(r, ::core::ptr::null_mut::<::core::ffi::c_int>());
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
pub const CONFIG_NR_CPUS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NR_CPUS: ::core::ffi::c_int = CONFIG_NR_CPUS;
#[inline]
unsafe extern "C" fn fls_long(mut l: ::core::ffi::c_ulong) -> ::core::ffi::c_uint {
    unsafe {
        if ::core::mem::size_of::<::core::ffi::c_ulong>() == 4 as usize {
            return ({
                let mut x_: ::core::ffi::c_ulong = l as ::core::ffi::c_ulong;
                if false {
                    if x_ != 0 as ::core::ffi::c_ulong {
                        32 as ::core::ffi::c_int
                            - (x_ as ::core::ffi::c_uint).leading_zeros() as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }
                } else {
                    variable_fls(x_ as ::core::ffi::c_uint)
                }
            }) as ::core::ffi::c_uint;
        }
        return fls64(l as __u64) as ::core::ffi::c_uint;
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
#[inline]
unsafe extern "C" fn __roundup_pow_of_two(mut n: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    unsafe {
        return (1 as ::core::ffi::c_ulong) << fls_long(n.wrapping_sub(1 as ::core::ffi::c_ulong));
    }
}
pub const PAGE_SHIFT: ::core::ffi::c_int = CONFIG_PAGE_SHIFT;
pub const nr_cpu_ids: ::core::ffi::c_uint = NR_CPUS as ::core::ffi::c_uint;
#[inline(always)]
unsafe extern "C" fn cpu_possible(mut cpu: ::core::ffi::c_uint) -> bool_0 {
    unsafe {
        return cpu == 0 as ::core::ffi::c_uint;
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
pub const OBJPOOL_NR_OBJECT_MAX: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 24 as ::core::ffi::c_int;
pub const OBJPOOL_OBJECT_SIZE_MAX: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 16 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn __objpool_try_get_slot(
    mut pool: *mut objpool_head,
    mut cpu: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut slot: *mut objpool_slot = *(*pool).cpu_slots.offset(cpu as isize);
        let mut head: uint32_t = ({
            let mut ___p1: uint32_t = ({
                extern "C" {
                    #[link_name = "__compiletime_assert_73"]
                    fn __compiletime_assert_73_0() -> !;
                }
                if !(::core::mem::size_of::<uint32_t>()
                    == ::core::mem::size_of::<::core::ffi::c_char>()
                    || ::core::mem::size_of::<uint32_t>()
                        == ::core::mem::size_of::<::core::ffi::c_short>()
                    || ::core::mem::size_of::<uint32_t>()
                        == ::core::mem::size_of::<::core::ffi::c_int>()
                    || ::core::mem::size_of::<uint32_t>()
                        == ::core::mem::size_of::<::core::ffi::c_long>()
                    || ::core::mem::size_of::<uint32_t>()
                        == ::core::mem::size_of::<::core::ffi::c_longlong>())
                {
                    __compiletime_assert_73_0();
                }
                ::core::ptr::read_volatile::<uint32_t>((&raw mut (*slot).head as *const uint32_t))
            });
            asm!("\n", options(preserves_flags));
            ___p1
        });
        while head
            != ({
                extern "C" {
                    #[link_name = "__compiletime_assert_74"]
                    fn __compiletime_assert_74_0() -> !;
                }
                if !(::core::mem::size_of::<uint32_t>()
                    == ::core::mem::size_of::<::core::ffi::c_char>()
                    || ::core::mem::size_of::<uint32_t>()
                        == ::core::mem::size_of::<::core::ffi::c_short>()
                    || ::core::mem::size_of::<uint32_t>()
                        == ::core::mem::size_of::<::core::ffi::c_int>()
                    || ::core::mem::size_of::<uint32_t>()
                        == ::core::mem::size_of::<::core::ffi::c_long>()
                    || ::core::mem::size_of::<uint32_t>()
                        == ::core::mem::size_of::<::core::ffi::c_longlong>())
                {
                    __compiletime_assert_74_0();
                }
                ::core::ptr::read_volatile::<uint32_t>((&raw mut (*slot).last as *const uint32_t))
            })
        {
            let mut obj: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if ({
                extern "C" {
                    #[link_name = "__compiletime_assert_75"]
                    fn __compiletime_assert_75_0() -> !;
                }
                if !(::core::mem::size_of::<uint32_t>()
                    == ::core::mem::size_of::<::core::ffi::c_char>()
                    || ::core::mem::size_of::<uint32_t>()
                        == ::core::mem::size_of::<::core::ffi::c_short>()
                    || ::core::mem::size_of::<uint32_t>()
                        == ::core::mem::size_of::<::core::ffi::c_int>()
                    || ::core::mem::size_of::<uint32_t>()
                        == ::core::mem::size_of::<::core::ffi::c_long>()
                    || ::core::mem::size_of::<uint32_t>()
                        == ::core::mem::size_of::<::core::ffi::c_longlong>())
                {
                    __compiletime_assert_75_0();
                }
                ::core::ptr::read_volatile::<uint32_t>((&raw mut (*slot).last as *const uint32_t))
            })
            .wrapping_sub(head as uint32_t)
            .wrapping_sub(1 as uint32_t)
                >= (*pool).nr_objs as uint32_t
            {
                head = ({
                    extern "C" {
                        #[link_name = "__compiletime_assert_76"]
                        fn __compiletime_assert_76_0() -> !;
                    }
                    if !(::core::mem::size_of::<uint32_t>()
                        == ::core::mem::size_of::<::core::ffi::c_char>()
                        || ::core::mem::size_of::<uint32_t>()
                            == ::core::mem::size_of::<::core::ffi::c_short>()
                        || ::core::mem::size_of::<uint32_t>()
                            == ::core::mem::size_of::<::core::ffi::c_int>()
                        || ::core::mem::size_of::<uint32_t>()
                            == ::core::mem::size_of::<::core::ffi::c_long>()
                        || ::core::mem::size_of::<uint32_t>()
                            == ::core::mem::size_of::<::core::ffi::c_longlong>())
                    {
                        __compiletime_assert_76_0();
                    }
                    ::core::ptr::read_volatile::<uint32_t>(
                        (&raw mut (*slot).head as *const uint32_t),
                    )
                }) as uint32_t;
            } else {
                obj = ({
                    extern "C" {
                        #[link_name = "__compiletime_assert_77"]
                        fn __compiletime_assert_77_0() -> !;
                    }
                    if !(::core::mem::size_of::<*mut ::core::ffi::c_void>()
                        == ::core::mem::size_of::<::core::ffi::c_char>()
                        || ::core::mem::size_of::<*mut ::core::ffi::c_void>()
                            == ::core::mem::size_of::<::core::ffi::c_short>()
                        || ::core::mem::size_of::<*mut ::core::ffi::c_void>()
                            == ::core::mem::size_of::<::core::ffi::c_int>()
                        || ::core::mem::size_of::<*mut ::core::ffi::c_void>()
                            == ::core::mem::size_of::<::core::ffi::c_long>()
                        || ::core::mem::size_of::<*mut ::core::ffi::c_void>()
                            == ::core::mem::size_of::<::core::ffi::c_longlong>())
                    {
                        __compiletime_assert_77_0();
                    }
                    ::core::ptr::read_volatile::<*mut ::core::ffi::c_void>(
                        ((&raw mut (*slot).entries as *mut *mut ::core::ffi::c_void)
                            .offset((head & (*slot).mask) as isize)
                            as *const *mut ::core::ffi::c_void),
                    )
                }) as *mut ::core::ffi::c_void;
                if ({
                    let mut __ai_ptr: *mut uint32_t = &raw mut (*slot).head;
                    let mut __ai_oldp: *mut uint32_t = &raw mut head;
                    instrument_atomic_read_write(
                        __ai_ptr as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<uint32_t>() as size_t,
                    );
                    instrument_read_write(
                        __ai_oldp as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<uint32_t>() as size_t,
                    );
                    ({
                        let mut ___op: *mut uint32_t = __ai_oldp as *mut uint32_t;
                        let mut ___o: uint32_t = *___op as uint32_t;
                        let mut ___r: uint32_t = 0;
                        ___r = ({
                            let mut __ptr: *mut uint32_t = __ai_ptr as *mut uint32_t;
                            let mut __old: uint32_t = ___o as uint32_t;
                            let mut __new: uint32_t =
                                (head as uint32_t).wrapping_add(1 as uint32_t);
                            let mut __ret: uint32_t = 0;
                            match ::core::mem::size_of::<uint32_t>() {
                                1 => {
                                    '_c2rust_label_7: {
                                        let mut __ptr32b: *mut u32_0 =
                                            ::core::ptr::with_exposed_provenance_mut::<u32_0>(
                                                (__ptr.expose_provenance() as ulong
                                                    & !(0x3 as ::core::ffi::c_int) as ulong)
                                                    as usize,
                                            );
                                        let mut __s: ulong = (__ptr.expose_provenance()
                                            as ulong
                                            & (0x4 as ulong).wrapping_sub(
                                                ::core::mem::size_of::<uint32_t>() as ulong,
                                            ))
                                        .wrapping_mul(BITS_PER_BYTE as ulong);
                                        let mut __mask: ulong = (::core::mem::size_of::<
                                            C2Rust_Unnamed_38,
                                        >() as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                            .wrapping_add(
                                                ((1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                    << (8 as usize)
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<::core::ffi::c_ulong>(),
                                                        )
                                                        .wrapping_sub(1 as usize)
                                                        .wrapping_sub(
                                                            ((-1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                                < 1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                                as ::core::ffi::c_int as usize,
                                                        ))
                                                    .wrapping_sub(1 as ::core::ffi::c_ulong)
                                                    .wrapping_add(
                                                        (1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                            << (8 as usize)
                                                                .wrapping_mul(
                                                                    ::core::mem::size_of::<::core::ffi::c_ulong>(),
                                                                )
                                                                .wrapping_sub(1 as usize)
                                                                .wrapping_sub(
                                                                    ((-1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                                        < 1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                                        as ::core::ffi::c_int as usize,
                                                                ),
                                                    ) << 0 as ::core::ffi::c_int
                                                    & ((1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                        << (8 as usize)
                                                            .wrapping_mul(
                                                                ::core::mem::size_of::<::core::ffi::c_ulong>(),
                                                            )
                                                            .wrapping_sub(1 as usize)
                                                            .wrapping_sub(
                                                                ((-1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                                    < 1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                                    as ::core::ffi::c_int as usize,
                                                            ))
                                                        .wrapping_sub(1 as ::core::ffi::c_ulong)
                                                        .wrapping_add(
                                                            (1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                                << (8 as usize)
                                                                    .wrapping_mul(
                                                                        ::core::mem::size_of::<::core::ffi::c_ulong>(),
                                                                    )
                                                                    .wrapping_sub(1 as usize)
                                                                    .wrapping_sub(
                                                                        ((-1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                                            < 1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                                            as ::core::ffi::c_int as usize,
                                                                    ),
                                                        )
                                                        >> ::core::mem::size_of::<::core::ffi::c_ulong>()
                                                            .wrapping_mul(BITS_PER_BYTE as usize)
                                                            .wrapping_sub(1 as usize)
                                                            .wrapping_sub(
                                                                ::core::mem::size_of::<uint32_t>()
                                                                    .wrapping_mul(8 as usize)
                                                                    .wrapping_sub(1 as usize),
                                                            ),
                                            ) << __s;
                                        let mut __newx: ulong = (__new as ulong) << __s;
                                        let mut __oldx: ulong = (__old as ulong) << __s;
                                        let mut __retx: ulong = 0;
                                        let mut __rc: ulong = 0;
                                        asm!(
                                            "0:\tlr.w {0}, 0({2})\n", "\tand  {1}, {0}, {5:}\n",
                                            "\tbne  {1}, {3:}, 1f\n", "\tand  {1}, {0}, {6:}\n",
                                            "\tor   {1}, {1}, {4:}\n", "\tsc.w {1}, {1}, 0({2})\n",
                                            "\tbnez {1}, 0b\n", "1:\n", "\n", out(reg) __retx, out(reg)
                                            __rc, in (reg) & raw mut * __ptr32b, inlateout(reg) __oldx
                                            as ::core::ffi::c_long => _, inlateout(reg) __newx => _,
                                            inlateout(reg) __mask => _, inlateout(reg) ! __mask => _,
                                            options(preserves_flags)
                                        );
                                        __ret =
                                            ((__retx & __mask) >> __s) as uint32_t as uint32_t;
                                    };
                                }
                                2 => {
                                    '_c2rust_label_8: {
                                        let mut __ptr32b: *mut u32_0 =
                                            ::core::ptr::with_exposed_provenance_mut::<u32_0>(
                                                (__ptr.expose_provenance() as ulong
                                                    & !(0x3 as ::core::ffi::c_int) as ulong)
                                                    as usize,
                                            );
                                        let mut __s: ulong = (__ptr.expose_provenance()
                                            as ulong
                                            & (0x4 as ulong).wrapping_sub(
                                                ::core::mem::size_of::<uint32_t>() as ulong,
                                            ))
                                        .wrapping_mul(BITS_PER_BYTE as ulong);
                                        let mut __mask: ulong = (::core::mem::size_of::<
                                            C2Rust_Unnamed_37,
                                        >() as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                            .wrapping_add(
                                                ((1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                    << (8 as usize)
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<::core::ffi::c_ulong>(),
                                                        )
                                                        .wrapping_sub(1 as usize)
                                                        .wrapping_sub(
                                                            ((-1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                                < 1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                                as ::core::ffi::c_int as usize,
                                                        ))
                                                    .wrapping_sub(1 as ::core::ffi::c_ulong)
                                                    .wrapping_add(
                                                        (1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                            << (8 as usize)
                                                                .wrapping_mul(
                                                                    ::core::mem::size_of::<::core::ffi::c_ulong>(),
                                                                )
                                                                .wrapping_sub(1 as usize)
                                                                .wrapping_sub(
                                                                    ((-1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                                        < 1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                                        as ::core::ffi::c_int as usize,
                                                                ),
                                                    ) << 0 as ::core::ffi::c_int
                                                    & ((1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                        << (8 as usize)
                                                            .wrapping_mul(
                                                                ::core::mem::size_of::<::core::ffi::c_ulong>(),
                                                            )
                                                            .wrapping_sub(1 as usize)
                                                            .wrapping_sub(
                                                                ((-1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                                    < 1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                                    as ::core::ffi::c_int as usize,
                                                            ))
                                                        .wrapping_sub(1 as ::core::ffi::c_ulong)
                                                        .wrapping_add(
                                                            (1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                                << (8 as usize)
                                                                    .wrapping_mul(
                                                                        ::core::mem::size_of::<::core::ffi::c_ulong>(),
                                                                    )
                                                                    .wrapping_sub(1 as usize)
                                                                    .wrapping_sub(
                                                                        ((-1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                                            < 1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                                                                            as ::core::ffi::c_int as usize,
                                                                    ),
                                                        )
                                                        >> ::core::mem::size_of::<::core::ffi::c_ulong>()
                                                            .wrapping_mul(BITS_PER_BYTE as usize)
                                                            .wrapping_sub(1 as usize)
                                                            .wrapping_sub(
                                                                ::core::mem::size_of::<uint32_t>()
                                                                    .wrapping_mul(8 as usize)
                                                                    .wrapping_sub(1 as usize),
                                                            ),
                                            ) << __s;
                                        let mut __newx: ulong = (__new as ulong) << __s;
                                        let mut __oldx: ulong = (__old as ulong) << __s;
                                        let mut __retx: ulong = 0;
                                        let mut __rc: ulong = 0;
                                        asm!(
                                            "0:\tlr.w {0}, 0({2})\n", "\tand  {1}, {0}, {5:}\n",
                                            "\tbne  {1}, {3:}, 1f\n", "\tand  {1}, {0}, {6:}\n",
                                            "\tor   {1}, {1}, {4:}\n", "\tsc.w {1}, {1}, 0({2})\n",
                                            "\tbnez {1}, 0b\n", "1:\n", "\n", out(reg) __retx, out(reg)
                                            __rc, in (reg) & raw mut * __ptr32b, inlateout(reg) __oldx
                                            as ::core::ffi::c_long => _, inlateout(reg) __newx => _,
                                            inlateout(reg) __mask => _, inlateout(reg) ! __mask => _,
                                            options(preserves_flags)
                                        );
                                        __ret =
                                            ((__retx & __mask) >> __s) as uint32_t as uint32_t;
                                    };
                                }
                                4 => {
                                    '_c2rust_label_9: {
                                        let mut __rc: ::core::ffi::c_uint = 0;
                                        asm!(
                                            "0:\tlr.w {0}, 0({2})\n", "\tbne  {0}, {3:}, 1f\n",
                                            "\tsc.w {1}, {4:}, 0({2})\n", "\tbnez {1}, 0b\n", "1:\n",
                                            "\n", out(reg) __ret, out(reg) __rc, in (reg) & raw mut *
                                            __ptr, inlateout(reg) __old as ::core::ffi::c_long as
                                            ::core::ffi::c_int as ::core::ffi::c_long => _,
                                            inlateout(reg) __new => _, options(preserves_flags)
                                        );
                                    };
                                }
                                8 => {
                                    '_c2rust_label_10: {
                                        let mut __rc: ::core::ffi::c_uint = 0;
                                        asm!(
                                            "0:\tlr.d {0}, 0({2})\n", "\tbne  {0}, {3:}, 1f\n",
                                            "\tsc.d {1}, {4:}, 0({2})\n", "\tbnez {1}, 0b\n", "1:\n",
                                            "\n", out(reg) __ret, out(reg) __rc, in (reg) & raw mut *
                                            __ptr, inlateout(reg) __old => _, inlateout(reg) __new => _,
                                            options(preserves_flags)
                                        );
                                    };
                                }
                                _ => {
                                    extern "C" {
                                        #[link_name = "__compiletime_assert_78"]
                                        fn __compiletime_assert_78_0() -> !;
                                    }
                                    if true {
                                        __compiletime_assert_78_0();
                                    }
                                }
                            }
                            __ret
                        }) as uint32_t;
                        if (___r != ___o) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                            *___op = ___r as uint32_t;
                        }
                        (___r == ___o) as ::core::ffi::c_int as ::core::ffi::c_long
                    })
                }) != 0
                {
                    return obj;
                }
            }
        }
        return NULL;
    }
}
#[inline]
unsafe extern "C" fn objpool_pop(mut pool: *mut objpool_head) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut obj: *mut ::core::ffi::c_void = NULL;
        let mut flags: ::core::ffi::c_ulong = 0;
        let mut start: ::core::ffi::c_int = 0;
        let mut cpu: ::core::ffi::c_int = 0;
        ({
            let mut __dummy: ::core::ffi::c_ulong = 0;
            let mut __dummy2: ::core::ffi::c_ulong = 0;
            &raw mut __dummy;
            &raw mut __dummy2;
        });
        flags = arch_local_irq_save();
        start = 0 as ::core::ffi::c_int;
        cpu = 0 as ::core::ffi::c_int;
        while cpu < 1 as ::core::ffi::c_int {
            obj = __objpool_try_get_slot(pool, cpu);
            if !obj.is_null() {
                break;
            }
            cpu += 1;
        }
        ({
            let mut __dummy: ::core::ffi::c_ulong = 0;
            let mut __dummy2: ::core::ffi::c_ulong = 0;
            &raw mut __dummy;
            &raw mut __dummy2;
        });
        arch_local_irq_restore(flags);
        return obj;
    }
}
#[inline]
unsafe extern "C" fn mem_alloc_profiling_enabled() -> bool_0 {
    unsafe {
        return r#false as ::core::ffi::c_int != 0;
    }
}
pub const KMALLOC_SHIFT_HIGH: ::core::ffi::c_int = PAGE_SHIFT + 1 as ::core::ffi::c_int;
pub const KMALLOC_SHIFT_LOW: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const KMALLOC_MAX_CACHE_SIZE: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << KMALLOC_SHIFT_HIGH;
pub const KMALLOC_MIN_SIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << KMALLOC_SHIFT_LOW;
#[inline(always)]
unsafe extern "C" fn kmalloc_type(
    mut flags: gfp_t,
    mut token: kmalloc_token_t,
) -> kmalloc_cache_type {
    unsafe {
        if (flags
            & (((1 as ::core::ffi::c_ulong) << ___GFP_RECLAIMABLE_BIT as ::core::ffi::c_int)
                as gfp_t
                | (if false {
                    ((1 as ::core::ffi::c_ulong) << ___GFP_DMA_BIT as ::core::ffi::c_int) as gfp_t
                } else {
                    0 as gfp_t
                })
                | (if false {
                    ((1 as ::core::ffi::c_ulong) << ___GFP_ACCOUNT_BIT as ::core::ffi::c_int)
                        as gfp_t
                } else {
                    0 as gfp_t
                }))
            == 0 as gfp_t) as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            return KMALLOC_NORMAL;
        }
        if false
            && flags
                & ((1 as ::core::ffi::c_ulong) << ___GFP_DMA_BIT as ::core::ffi::c_int) as gfp_t
                != 0
        {
            return KMALLOC_DMA;
        }
        if true
            || flags
                & ((1 as ::core::ffi::c_ulong) << ___GFP_RECLAIMABLE_BIT as ::core::ffi::c_int)
                    as gfp_t
                != 0
        {
            return KMALLOC_RECLAIM;
        } else {
            return KMALLOC_CGROUP;
        };
    }
}
#[inline(always)]
unsafe extern "C" fn __kmalloc_index(
    mut size: size_t,
    mut size_is_constant: bool_0,
) -> ::core::ffi::c_uint {
    unsafe {
        if size == 0 {
            return 0 as ::core::ffi::c_uint;
        }
        if size <= KMALLOC_MIN_SIZE as size_t {
            return KMALLOC_SHIFT_LOW as ::core::ffi::c_uint;
        }
        if KMALLOC_MIN_SIZE <= 32 as ::core::ffi::c_int
            && size > 64 as size_t
            && size <= 96 as size_t
        {
            return 1 as ::core::ffi::c_uint;
        }
        if KMALLOC_MIN_SIZE <= 64 as ::core::ffi::c_int
            && size > 128 as size_t
            && size <= 192 as size_t
        {
            return 2 as ::core::ffi::c_uint;
        }
        if size <= 8 as size_t {
            return 3 as ::core::ffi::c_uint;
        }
        if size <= 16 as size_t {
            return 4 as ::core::ffi::c_uint;
        }
        if size <= 32 as size_t {
            return 5 as ::core::ffi::c_uint;
        }
        if size <= 64 as size_t {
            return 6 as ::core::ffi::c_uint;
        }
        if size <= 128 as size_t {
            return 7 as ::core::ffi::c_uint;
        }
        if size <= 256 as size_t {
            return 8 as ::core::ffi::c_uint;
        }
        if size <= 512 as size_t {
            return 9 as ::core::ffi::c_uint;
        }
        if size <= 1024 as size_t {
            return 10 as ::core::ffi::c_uint;
        }
        if size <= (2 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as size_t {
            return 11 as ::core::ffi::c_uint;
        }
        if size <= (4 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as size_t {
            return 12 as ::core::ffi::c_uint;
        }
        if size <= (8 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as size_t {
            return 13 as ::core::ffi::c_uint;
        }
        if size <= (16 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as size_t {
            return 14 as ::core::ffi::c_uint;
        }
        if size <= (32 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as size_t {
            return 15 as ::core::ffi::c_uint;
        }
        if size <= (64 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as size_t {
            return 16 as ::core::ffi::c_uint;
        }
        if size <= (128 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as size_t {
            return 17 as ::core::ffi::c_uint;
        }
        if size <= (256 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as size_t {
            return 18 as ::core::ffi::c_uint;
        }
        if size <= (512 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as size_t {
            return 19 as ::core::ffi::c_uint;
        }
        if size <= (1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as size_t {
            return 20 as ::core::ffi::c_uint;
        }
        if size
            <= (2 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int)
                as size_t
        {
            return 21 as ::core::ffi::c_uint;
        }
        if true && size_is_constant as ::core::ffi::c_int != 0 {
            extern "C" {
                #[link_name = "__compiletime_assert_160"]
                fn __compiletime_assert_160_0() -> !;
            }
            if true {
                __compiletime_assert_160_0();
            }
        } else {
            asm!("ebreak\n", "\n", options(preserves_flags));
            unreachable!();
        }
        return -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
}
#[inline(always)]
unsafe extern "C" fn _kmalloc_noprof(
    mut size: size_t,
    mut flags: gfp_t,
    mut token: kmalloc_token_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        if false && size != 0 {
            let mut index: ::core::ffi::c_uint = 0;
            if size > KMALLOC_MAX_CACHE_SIZE {
                return __kmalloc_large_noprof(size, flags);
            }
            index = __kmalloc_index(size, r#true as ::core::ffi::c_int != 0);
            return __kmalloc_cache_noprof(
                kmalloc_caches[kmalloc_type(flags, token) as usize][index as usize],
                flags,
                size,
            );
        }
        return __kmalloc_noprof(size as size_t, flags);
    }
}
#[inline(always)]
unsafe extern "C" fn _kmalloc_node_noprof(
    mut size: size_t,
    mut flags: gfp_t,
    mut node: ::core::ffi::c_int,
    mut token: kmalloc_token_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        if false && size != 0 {
            let mut index: ::core::ffi::c_uint = 0;
            if size > KMALLOC_MAX_CACHE_SIZE {
                return __kmalloc_large_node_noprof(size, flags, node);
            }
            index = __kmalloc_index(size, r#true as ::core::ffi::c_int != 0);
            return __kmalloc_cache_node_noprof(
                kmalloc_caches[kmalloc_type(flags, token) as usize][index as usize],
                flags,
                node,
                size,
            );
        }
        return __kmalloc_node_noprof(size as size_t, flags, node);
    }
}
#[inline]
unsafe extern "C" fn _kzalloc_noprof(
    mut size: size_t,
    mut flags: gfp_t,
    mut token: kmalloc_token_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return _kmalloc_noprof(
            size,
            flags | ((1 as ::core::ffi::c_ulong) << ___GFP_ZERO_BIT as ::core::ffi::c_int) as gfp_t,
            token,
        );
    }
}
unsafe extern "C" fn objpool_init_percpu_slot(
    mut pool: *mut objpool_head,
    mut slot: *mut objpool_slot,
    mut nodes: ::core::ffi::c_int,
    mut context: *mut ::core::ffi::c_void,
    mut objinit: objpool_init_obj_cb,
) -> ::core::ffi::c_int {
    unsafe {
        let mut obj: *mut ::core::ffi::c_void =
            (&raw mut (*slot).entries as *mut *mut ::core::ffi::c_void)
                .offset((*pool).capacity as isize) as *mut ::core::ffi::c_void;
        let mut i: ::core::ffi::c_int = 0;
        (*slot).mask = ((*pool).capacity - 1 as ::core::ffi::c_int) as uint32_t;
        i = 0 as ::core::ffi::c_int;
        while i < nodes {
            if objinit.is_some() {
                let mut rc: ::core::ffi::c_int =
                    objinit.expect("non-null function pointer")(obj, context);
                if rc != 0 {
                    return rc;
                }
            }
            *(&raw mut (*slot).entries as *mut *mut ::core::ffi::c_void)
                .offset(((*slot).tail & (*slot).mask) as isize) = obj;
            obj = obj.offset((*pool).obj_size as isize);
            (*slot).tail = (*slot).tail.wrapping_add(1);
            (*slot).last = (*slot).tail;
            (*pool).nr_objs += 1;
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn objpool_init_percpu_slots(
    mut pool: *mut objpool_head,
    mut nr_objs: ::core::ffi::c_int,
    mut context: *mut ::core::ffi::c_void,
    mut objinit: objpool_init_obj_cb,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut cpu_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while (i as ::core::ffi::c_uint) < nr_cpu_ids {
            let mut slot: *mut objpool_slot = ::core::ptr::null_mut::<objpool_slot>();
            let mut nodes: ::core::ffi::c_int = 0;
            let mut size: ::core::ffi::c_int = 0;
            let mut rc: ::core::ffi::c_int = 0;
            if cpu_possible(i as ::core::ffi::c_uint) {
                nodes = nr_objs / (*pool).nr_possible_cpus;
                if cpu_count < nr_objs % (*pool).nr_possible_cpus {
                    nodes += 1;
                }
                cpu_count += 1;
                size = size_add(
                    ::core::mem::size_of::<objpool_slot>() as size_t,
                    size_mul(
                        (*pool).capacity as size_t,
                        (::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
                            .wrapping_add(::core::mem::size_of::<C2Rust_Unnamed_33>()
                                as ::core::ffi::c_int
                                as size_t),
                    ),
                )
                .wrapping_add(((*pool).obj_size * nodes) as size_t)
                    as ::core::ffi::c_int;
                slot = ::core::ptr::null_mut::<objpool_slot>();
                if (*pool).gfp
                    & (((1 as ::core::ffi::c_ulong) << ___GFP_HIGH_BIT as ::core::ffi::c_int)
                        as gfp_t
                        | ((1 as ::core::ffi::c_ulong)
                            << ___GFP_KSWAPD_RECLAIM_BIT as ::core::ffi::c_int)
                            as gfp_t
                        | (((1 as ::core::ffi::c_ulong)
                            << ___GFP_DIRECT_RECLAIM_BIT as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_ulong)
                                << ___GFP_KSWAPD_RECLAIM_BIT as ::core::ffi::c_int)
                            as gfp_t
                            | ((1 as ::core::ffi::c_ulong) << ___GFP_IO_BIT as ::core::ffi::c_int)
                                as gfp_t
                            | ((1 as ::core::ffi::c_ulong) << ___GFP_FS_BIT as ::core::ffi::c_int)
                                as gfp_t))
                    != ((1 as ::core::ffi::c_ulong) << ___GFP_HIGH_BIT as ::core::ffi::c_int)
                        as gfp_t
                        | ((1 as ::core::ffi::c_ulong)
                            << ___GFP_KSWAPD_RECLAIM_BIT as ::core::ffi::c_int)
                            as gfp_t
                {
                    slot = ({
                        ({
                            let mut _res: *mut ::core::ffi::c_void =
                                ::core::ptr::null_mut::<::core::ffi::c_void>();
                            if mem_alloc_profiling_enabled() {
                                let mut _old: *mut alloc_tag = ::core::ptr::null_mut::<alloc_tag>();
                                _old = ::core::ptr::null_mut::<alloc_tag>();
                                _res = __vmalloc_node_noprof(
                                    size as ::core::ffi::c_ulong,
                                    ::core::mem::size_of::<*mut ::core::ffi::c_void>()
                                        as ::core::ffi::c_ulong,
                                    (*pool).gfp,
                                    0 as ::core::ffi::c_int,
                                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                )
                                    as *mut ::core::ffi::c_void;
                            } else {
                                _res = __vmalloc_node_noprof(
                                    size as ::core::ffi::c_ulong,
                                    ::core::mem::size_of::<*mut ::core::ffi::c_void>()
                                        as ::core::ffi::c_ulong,
                                    (*pool).gfp,
                                    0 as ::core::ffi::c_int,
                                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                )
                                    as *mut ::core::ffi::c_void;
                            }
                            _res
                        })
                    }) as *mut objpool_slot;
                }
                if slot.is_null() {
                    slot = ({
                        ({
                            let mut _res: *mut ::core::ffi::c_void =
                                ::core::ptr::null_mut::<::core::ffi::c_void>();
                            if mem_alloc_profiling_enabled() {
                                let mut _old: *mut alloc_tag = ::core::ptr::null_mut::<alloc_tag>();
                                _old = ::core::ptr::null_mut::<alloc_tag>();
                                _res = _kmalloc_node_noprof(
                                    size as size_t,
                                    (*pool).gfp,
                                    0 as ::core::ffi::c_int,
                                    kmalloc_token_t {},
                                )
                                    as *mut ::core::ffi::c_void;
                            } else {
                                _res = _kmalloc_node_noprof(
                                    size as size_t,
                                    (*pool).gfp,
                                    0 as ::core::ffi::c_int,
                                    kmalloc_token_t {},
                                )
                                    as *mut ::core::ffi::c_void;
                            }
                            _res
                        })
                    }) as *mut objpool_slot;
                    if slot.is_null() {
                        return -ENOMEM;
                    }
                }
                memset(
                    slot as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    size as size_t,
                );
                *(*pool).cpu_slots.offset(i as isize) = slot;
                rc = objpool_init_percpu_slot(pool, slot, nodes, context, objinit);
                if rc != 0 {
                    return rc;
                }
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn objpool_fini_percpu_slots(mut pool: *mut objpool_head) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if (*pool).cpu_slots.is_null() {
            return;
        }
        i = 0 as ::core::ffi::c_int;
        while (i as ::core::ffi::c_uint) < nr_cpu_ids {
            kvfree(*(*pool).cpu_slots.offset(i as isize) as *const ::core::ffi::c_void);
            i += 1;
        }
        kfree((*pool).cpu_slots as *const ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn objpool_init(
    mut pool: *mut objpool_head,
    mut nr_objs: ::core::ffi::c_int,
    mut object_size: ::core::ffi::c_int,
    mut gfp: gfp_t,
    mut context: *mut ::core::ffi::c_void,
    mut objinit: objpool_init_obj_cb,
    mut release: objpool_fini_cb,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut capacity: ::core::ffi::c_int = 0;
        let mut slot_size: ::core::ffi::c_int = 0;
        if nr_objs <= 0 as ::core::ffi::c_int
            || nr_objs as ::core::ffi::c_ulong > OBJPOOL_NR_OBJECT_MAX
            || object_size <= 0 as ::core::ffi::c_int
            || object_size as ::core::ffi::c_ulong > OBJPOOL_OBJECT_SIZE_MAX
        {
            return -EINVAL;
        }
        object_size = object_size
            + (::core::mem::size_of::<::core::ffi::c_long>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
            & !(::core::mem::size_of::<::core::ffi::c_long>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int);
        capacity = (if false {
            if nr_objs == 1 as ::core::ffi::c_int {
                1 as ::core::ffi::c_ulong
            } else {
                (1 as ::core::ffi::c_ulong)
                    << (if false {
                        (if (nr_objs - 1 as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int
                        } else {
                            63 as ::core::ffi::c_int
                                - ((nr_objs - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong)
                                    .leading_zeros()
                                    as ::core::ffi::c_int
                        })
                    } else {
                        (if ::core::mem::size_of::<::core::ffi::c_int>() <= 4 as usize {
                            __ilog2_u32((nr_objs - 1 as ::core::ffi::c_int) as u32_0)
                        } else {
                            __ilog2_u64((nr_objs - 1 as ::core::ffi::c_int) as u64_0)
                        })
                    }) + 1 as ::core::ffi::c_int
            }
        } else {
            __roundup_pow_of_two(nr_objs as ::core::ffi::c_ulong)
        }) as ::core::ffi::c_int;
        if capacity == 0 {
            return -EINVAL;
        }
        memset(
            pool as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<objpool_head>() as size_t,
        );
        (*pool).nr_possible_cpus = 1 as ::core::ffi::c_int;
        (*pool).obj_size = object_size;
        (*pool).capacity = capacity;
        (*pool).gfp = gfp
            & !(((1 as ::core::ffi::c_ulong) << ___GFP_ZERO_BIT as ::core::ffi::c_int) as gfp_t);
        (*pool).context = context;
        (*pool).release = release;
        slot_size = (nr_cpu_ids as usize).wrapping_mul(::core::mem::size_of::<*mut objpool_slot>())
            as ::core::ffi::c_int;
        (*pool).cpu_slots = ({
            ({
                let mut _res: *mut ::core::ffi::c_void =
                    ::core::ptr::null_mut::<::core::ffi::c_void>();
                if mem_alloc_profiling_enabled() {
                    let mut _old: *mut alloc_tag = ::core::ptr::null_mut::<alloc_tag>();
                    _old = ::core::ptr::null_mut::<alloc_tag>();
                    _res = _kzalloc_noprof(slot_size as size_t, (*pool).gfp, kmalloc_token_t {})
                        as *mut ::core::ffi::c_void;
                } else {
                    _res = _kzalloc_noprof(slot_size as size_t, (*pool).gfp, kmalloc_token_t {})
                        as *mut ::core::ffi::c_void;
                }
                _res
            })
        }) as *mut *mut objpool_slot;
        if (*pool).cpu_slots.is_null() {
            return -ENOMEM;
        }
        rc = objpool_init_percpu_slots(pool, nr_objs, context, objinit);
        if rc != 0 {
            objpool_fini_percpu_slots(pool);
        } else {
            refcount_set(
                &raw mut (*pool).r#ref,
                (*pool).nr_objs + 1 as ::core::ffi::c_int,
            );
        }
        return rc;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_objpool_init: ; .asciz \"GPL\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad objpool_init ; .previous"
);
#[no_mangle]
pub unsafe extern "C" fn objpool_free(mut pool: *mut objpool_head) {
    unsafe {
        if (*pool).cpu_slots.is_null() {
            return;
        }
        objpool_fini_percpu_slots(pool);
        if (*pool).release.is_some() {
            (*pool).release.expect("non-null function pointer")(
                pool as *mut objpool_head,
                (*pool).context,
            );
        }
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_objpool_free: ; .asciz \"GPL\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad objpool_free ; .previous"
);
#[no_mangle]
pub unsafe extern "C" fn objpool_drop(
    mut obj: *mut ::core::ffi::c_void,
    mut pool: *mut objpool_head,
) -> ::core::ffi::c_int {
    unsafe {
        if obj.is_null() || pool.is_null() {
            return -EINVAL;
        }
        if refcount_dec_and_test(&raw mut (*pool).r#ref) {
            objpool_free(pool);
            return 0 as ::core::ffi::c_int;
        }
        return -EAGAIN;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_objpool_drop: ; .asciz \"GPL\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad objpool_drop ; .previous"
);
#[no_mangle]
pub unsafe extern "C" fn objpool_fini(mut pool: *mut objpool_head) {
    unsafe {
        let mut count: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while !objpool_pop(pool).is_null() {
            count += 1;
        }
        if refcount_sub_and_test(count, &raw mut (*pool).r#ref) {
            objpool_free(pool);
        }
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_objpool_fini: ; .asciz \"GPL\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad objpool_fini ; .previous"
);
pub const CONFIG_PAGE_SHIFT: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
