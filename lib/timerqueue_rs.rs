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
use ::kernel::warn_on;

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

extern "C" {
    fn rb_insert_color(_: *mut rb_node, _: *mut rb_root);
    fn rb_erase(_: *mut rb_node, _: *mut rb_root);
    fn rb_next(_: *const rb_node) -> *mut rb_node;
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
pub type ktime_t = s64;
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
opaque_marker!(task_struct);
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
    _private: [u8; 0],
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
pub type atomic_long_t = atomic64_t;
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
    pub c2rust_unnamed: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub si_signo: ::core::ffi::c_int,
    pub si_errno: ::core::ffi::c_int,
    pub si_code: ::core::ffi::c_int,
    pub _sifields: __sifields,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __sifields {
    pub _kill: C2Rust_Unnamed_11,
    pub _timer: C2Rust_Unnamed_10,
    pub _rt: C2Rust_Unnamed_9,
    pub _sigchld: C2Rust_Unnamed_8,
    pub _sigfault: C2Rust_Unnamed_3,
    pub _sigpoll: C2Rust_Unnamed_2,
    pub _sigsys: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub _call_addr: *mut ::core::ffi::c_void,
    pub _syscall: ::core::ffi::c_int,
    pub _arch: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub _band: ::core::ffi::c_long,
    pub _fd: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub _addr: *mut ::core::ffi::c_void,
    pub c2rust_unnamed: C2Rust_Unnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub _trapno: ::core::ffi::c_int,
    pub _addr_lsb: ::core::ffi::c_short,
    pub _addr_bnd: C2Rust_Unnamed_7,
    pub _addr_pkey: C2Rust_Unnamed_6,
    pub _perf: C2Rust_Unnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub _data: ::core::ffi::c_ulong,
    pub _type: __u32,
    pub _flags: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub _dummy_pkey: [::core::ffi::c_char; 8],
    pub _pkey: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub _dummy_bnd: [::core::ffi::c_char; 8],
    pub _lower: *mut ::core::ffi::c_void,
    pub _upper: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
    pub _status: ::core::ffi::c_int,
    pub _utime: __kernel_clock_t,
    pub _stime: __kernel_clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
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
pub struct C2Rust_Unnamed_10 {
    pub _tid: __kernel_timer_t,
    pub _overrun: ::core::ffi::c_int,
    pub _sigval: sigval_t,
    pub _sys_private: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_12 {
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
    _private: [u8; 0],
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
    pub c2rust_unnamed: C2Rust_Unnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_13 {
    pub futex: C2Rust_Unnamed_17,
    pub nanosleep: C2Rust_Unnamed_15,
    pub poll: C2Rust_Unnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
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
pub struct C2Rust_Unnamed_15 {
    pub clockid: clockid_t,
    pub r#type: timespec_type,
    pub c2rust_unnamed: C2Rust_Unnamed_16,
    pub expires: ktime_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_16 {
    pub rmtp: *mut __kernel_timespec,
    pub compat_rmtp: *mut old_timespec32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct old_timespec32 {
    _private: [u8; 0],
}
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
pub struct C2Rust_Unnamed_17 {
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
    _private: [u8; 0],
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
opaque_marker!(sched_dl_entity);
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
    pub c2rust_unnamed: C2Rust_Unnamed_18,
    pub src: u16_0,
    pub dst: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_18 {
    pub u_flags: ::core::ffi::c_uint,
    pub a_flags: atomic_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct llist_node {
    pub next: *mut llist_node,
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
#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rb_root_cached {
    pub rb_root: rb_root,
    pub rb_leftmost: *mut rb_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rb_root_linked {
    pub rb_root: rb_root,
    pub rb_leftmost: *mut rb_node_linked,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timerqueue_node {
    pub node: rb_node,
    pub expires: ktime_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timerqueue_head {
    pub rb_root: rb_root_cached,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timerqueue_linked_head {
    pub rb_root: rb_root_linked,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut riscv_current_is_tp: *mut task_struct = ::core::ptr::null_mut::<task_struct>();
#[no_mangle]
pub static mut current_stack_pointer: ::core::ffi::c_ulong = 0;
#[inline]
unsafe extern "C" fn rb_link_node(
    mut node: *mut rb_node,
    mut parent: *mut rb_node,
    mut rb_link: *mut *mut rb_node,
) {
    unsafe {
        (*node).0.__rb_parent_color = parent.expose_provenance() as ::core::ffi::c_ulong;
        (*node).0.rb_right = ::core::ptr::null_mut::<rb_node>();
        (*node).0.rb_left = (*node).0.rb_right;
        *rb_link = node;
    }
}
#[inline]
unsafe extern "C" fn rb_insert_color_cached(
    mut node: *mut rb_node,
    mut root: *mut rb_root_cached,
    mut leftmost: bool_0,
) {
    unsafe {
        if leftmost {
            (*root).rb_leftmost = node;
        }
        rb_insert_color(node, &raw mut (*root).rb_root);
    }
}
#[inline]
unsafe extern "C" fn rb_erase_cached(
    mut node: *mut rb_node,
    mut root: *mut rb_root_cached,
) -> *mut rb_node {
    unsafe {
        let mut leftmost: *mut rb_node = ::core::ptr::null_mut::<rb_node>();
        if (*root).rb_leftmost == node {
            (*root).rb_leftmost = rb_next(node);
            leftmost = (*root).rb_leftmost;
        }
        rb_erase(node, &raw mut (*root).rb_root);
        return leftmost;
    }
}
#[inline(always)]
unsafe extern "C" fn rb_add_cached(
    mut node: *mut rb_node,
    mut tree: *mut rb_root_cached,
    mut less: Option<unsafe extern "C" fn(*mut rb_node, *const rb_node) -> bool_0>,
) -> *mut rb_node {
    unsafe {
        let mut link: *mut *mut rb_node = &raw mut (*tree).rb_root.rb_node;
        let mut parent: *mut rb_node = ::core::ptr::null_mut::<rb_node>();
        let mut leftmost: bool_0 = r#true as ::core::ffi::c_int != 0;
        while !(*link).is_null() {
            parent = *link;
            if less.expect("non-null function pointer")(node, parent) {
                link = &raw mut (*parent).0.rb_left;
            } else {
                link = &raw mut (*parent).0.rb_right;
                leftmost = r#false as ::core::ffi::c_int != 0;
            }
        }
        rb_link_node(node, parent, link);
        rb_insert_color_cached(node, tree, leftmost);
        return if leftmost as ::core::ffi::c_int != 0 {
            node
        } else {
            ::core::ptr::null_mut::<rb_node>()
        };
    }
}
#[inline(always)]
unsafe extern "C" fn __rb_add(
    mut node: *mut rb_node,
    mut tree: *mut rb_root,
    mut less: Option<unsafe extern "C" fn(*mut rb_node, *const rb_node) -> bool_0>,
    mut linkop: Option<unsafe extern "C" fn(*mut rb_node, *mut rb_node, *mut *mut rb_node) -> ()>,
) {
    unsafe {
        let mut link: *mut *mut rb_node = &raw mut (*tree).rb_node;
        let mut parent: *mut rb_node = ::core::ptr::null_mut::<rb_node>();
        while !(*link).is_null() {
            parent = *link;
            if less.expect("non-null function pointer")(node, parent) {
                link = &raw mut (*parent).0.rb_left;
            } else {
                link = &raw mut (*parent).0.rb_right;
            }
        }
        linkop.expect("non-null function pointer")(node, parent, link);
        rb_link_node(node, parent, link);
        rb_insert_color(node, tree);
    }
}
#[inline]
unsafe extern "C" fn rb_link_linked_node(
    mut node: *mut rb_node,
    mut parent: *mut rb_node,
    mut link: *mut *mut rb_node,
) {
    unsafe {
        if parent.is_null() {
            return;
        }
        let mut nnew: *mut rb_node_linked = ({
            let mut __mptr: *mut ::core::ffi::c_void = node as *mut ::core::ffi::c_void;
            __mptr.offset(-(0 as usize as isize)) as *mut rb_node_linked
        });
        let mut npar: *mut rb_node_linked = ({
            let mut __mptr: *mut ::core::ffi::c_void = parent as *mut ::core::ffi::c_void;
            __mptr.offset(-(0 as usize as isize)) as *mut rb_node_linked
        });
        if link == &raw mut (*parent).0.rb_left {
            (*nnew).prev = (*npar).prev;
            (*nnew).next = npar;
            (*npar).prev = nnew;
            if !(*nnew).prev.is_null() {
                (*(*nnew).prev).next = nnew;
            }
        } else {
            (*nnew).next = (*npar).next;
            (*nnew).prev = npar;
            (*npar).next = nnew;
            if !(*nnew).next.is_null() {
                (*(*nnew).next).prev = nnew;
            }
        };
    }
}
#[inline(always)]
unsafe extern "C" fn rb_add_linked(
    mut node: *mut rb_node_linked,
    mut tree: *mut rb_root_linked,
    mut less: Option<unsafe extern "C" fn(*mut rb_node, *const rb_node) -> bool_0>,
) -> bool_0 {
    unsafe {
        __rb_add(
            &raw mut (*node).node,
            &raw mut (*tree).rb_root,
            less,
            Some(
                rb_link_linked_node
                    as unsafe extern "C" fn(*mut rb_node, *mut rb_node, *mut *mut rb_node) -> (),
            ),
        );
        if (*node).prev.is_null() {
            (*tree).rb_leftmost = node;
        }
        return (*node).prev.is_null();
    }
}
#[inline]
unsafe extern "C" fn __timerqueue_less(mut a: *mut rb_node, mut b: *const rb_node) -> bool_0 {
    unsafe {
        return (*({
            let mut __mptr: *mut ::core::ffi::c_void = a as *mut ::core::ffi::c_void;
            __mptr.offset(-(0 as usize as isize)) as *mut timerqueue_node
        }))
        .expires
            < (*({
                let mut __mptr: *mut ::core::ffi::c_void = b as *mut ::core::ffi::c_void;
                __mptr.offset(-(0 as usize as isize)) as *mut timerqueue_node
            }))
            .expires;
    }
}
#[no_mangle]
pub unsafe extern "C" fn timerqueue_add(
    mut head: *mut timerqueue_head,
    mut node: *mut timerqueue_node,
) -> bool_0 {
    unsafe {
        kernel::warn_on!(
            !((*node).node.0.__rb_parent_color
                == (&raw mut (*node).node).expose_provenance() as ::core::ffi::c_ulong)
        );
        return !rb_add_cached(
            &raw mut (*node).node,
            &raw mut (*head).rb_root,
            Some(__timerqueue_less as unsafe extern "C" fn(*mut rb_node, *const rb_node) -> bool_0),
        )
        .is_null();
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_timerqueue_add: ; .asciz \"GPL\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad timerqueue_add ; .previous"
);
#[no_mangle]
pub unsafe extern "C" fn timerqueue_del(
    mut head: *mut timerqueue_head,
    mut node: *mut timerqueue_node,
) -> bool_0 {
    unsafe {
        kernel::warn_on!(
            (*node).node.0.__rb_parent_color
                == (&raw mut (*node).node).expose_provenance() as ::core::ffi::c_ulong
        );
        rb_erase_cached(&raw mut (*node).node, &raw mut (*head).rb_root);
        (*node).node.0.__rb_parent_color =
            (&raw mut (*node).node).expose_provenance() as ::core::ffi::c_ulong;
        return !({
            extern "C" {
                #[link_name = "__compiletime_assert_91"]
                fn __compiletime_assert_91_0() -> !;
            }
            if !(::core::mem::size_of::<*mut rb_node>()
                == ::core::mem::size_of::<::core::ffi::c_char>()
                || ::core::mem::size_of::<*mut rb_node>()
                    == ::core::mem::size_of::<::core::ffi::c_short>()
                || ::core::mem::size_of::<*mut rb_node>()
                    == ::core::mem::size_of::<::core::ffi::c_int>()
                || ::core::mem::size_of::<*mut rb_node>()
                    == ::core::mem::size_of::<::core::ffi::c_long>()
                || ::core::mem::size_of::<*mut rb_node>()
                    == ::core::mem::size_of::<::core::ffi::c_longlong>())
            {
                __compiletime_assert_91_0();
            }
            ::core::ptr::read_volatile::<*mut rb_node>(
                (&raw mut (*head).rb_root.rb_root.rb_node as *const *mut rb_node),
            )
        })
        .is_null();
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_timerqueue_del: ; .asciz \"GPL\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad timerqueue_del ; .previous"
);
#[no_mangle]
pub unsafe extern "C" fn timerqueue_iterate_next(
    mut node: *mut timerqueue_node,
) -> *mut timerqueue_node {
    unsafe {
        let mut next: *mut rb_node = ::core::ptr::null_mut::<rb_node>();
        if node.is_null() {
            return ::core::ptr::null_mut::<timerqueue_node>();
        }
        next = rb_next(&raw mut (*node).node);
        if next.is_null() {
            return ::core::ptr::null_mut::<timerqueue_node>();
        }
        return ({
            let mut __mptr: *mut ::core::ffi::c_void = next as *mut ::core::ffi::c_void;
            __mptr.offset(-(0 as usize as isize)) as *mut timerqueue_node
        });
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_timerqueue_iterate_next: ; .asciz \"GPL\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad timerqueue_iterate_next ; .previous"
);
#[inline(always)]
unsafe extern "C" fn __tq_linked_less(mut a: *mut rb_node, mut b: *const rb_node) -> bool_0 {
    unsafe {
        return (*({
            let mut __mptr: *mut ::core::ffi::c_void = ({
                let mut __mptr_0: *mut ::core::ffi::c_void = a as *mut ::core::ffi::c_void;
                __mptr_0.offset(-(0 as usize as isize)) as *mut rb_node_linked
            }) as *mut ::core::ffi::c_void;
            __mptr.offset(-(0 as usize as isize)) as *mut timerqueue_linked_node
        }))
        .expires
            < (*({
                let mut __mptr: *mut ::core::ffi::c_void = ({
                    let mut __mptr_0: *mut ::core::ffi::c_void = b as *mut ::core::ffi::c_void;
                    __mptr_0.offset(-(0 as usize as isize)) as *mut rb_node_linked
                })
                    as *mut ::core::ffi::c_void;
                __mptr.offset(-(0 as usize as isize)) as *mut timerqueue_linked_node
            }))
            .expires;
    }
}
#[no_mangle]
pub unsafe extern "C" fn timerqueue_linked_add(
    mut head: *mut timerqueue_linked_head,
    mut node: *mut timerqueue_linked_node,
) -> bool_0 {
    unsafe {
        return rb_add_linked(
            &raw mut (*node).node,
            &raw mut (*head).rb_root,
            Some(__tq_linked_less as unsafe extern "C" fn(*mut rb_node, *const rb_node) -> bool_0),
        );
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_timerqueue_linked_add: ; .asciz \"GPL\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad timerqueue_linked_add ; .previous"
);
