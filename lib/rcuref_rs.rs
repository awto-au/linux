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
    fn _printk(fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn __riscv_isa_extension_available(
        isa_bitmap: *const ::core::ffi::c_ulong,
        bit: ::core::ffi::c_uint,
    ) -> bool_0;
    fn __riscv_has_extension_unlikely(
        vendor: ::core::ffi::c_ulong,
        ext: ::core::ffi::c_ulong,
    ) -> bool_0;
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
pub struct rcuref_t {
    pub refcnt: atomic_t,
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
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct task_struct {
    pub thread_info: thread_info,
    pub __state: ::core::ffi::c_uint,
    pub saved_state: ::core::ffi::c_uint,
    pub stack: *mut ::core::ffi::c_void,
    pub usage: refcount_t,
    pub flags: ::core::ffi::c_uint,
    pub ptrace: ::core::ffi::c_uint,
    pub on_cpu: u8_0,
    pub on_rq: u8_0,
    pub is_blocked: u8_0,
    pub __pad: u8_0,
    pub wake_entry: __call_single_node,
    pub wakee_flips: ::core::ffi::c_uint,
    pub wakee_flip_decay_ts: ::core::ffi::c_ulong,
    pub last_wakee: *mut task_struct,
    pub recent_used_cpu: ::core::ffi::c_int,
    pub wake_cpu: ::core::ffi::c_int,
    pub prio: ::core::ffi::c_int,
    pub static_prio: ::core::ffi::c_int,
    pub normal_prio: ::core::ffi::c_int,
    pub rt_priority: ::core::ffi::c_uint,
    pub se: sched_entity,
    pub rt: sched_rt_entity,
    pub dl: sched_dl_entity,
    pub dl_server: *mut sched_dl_entity,
    pub sched_class: *const sched_class,
    pub stats: sched_statistics,
    pub policy: ::core::ffi::c_uint,
    pub max_allowed_capacity: ::core::ffi::c_ulong,
    pub nr_cpus_allowed: ::core::ffi::c_int,
    pub cpus_ptr: *const cpumask_t,
    pub user_cpus_ptr: *mut cpumask_t,
    pub cpus_mask: cpumask_t,
    pub migration_pending: *mut ::core::ffi::c_void,
    pub migration_disabled: ::core::ffi::c_ushort,
    pub migration_flags: ::core::ffi::c_ushort,
    pub sched_info: sched_info,
    pub tasks: list_head,
    pub pushable_tasks: plist_node,
    pub pushable_dl_tasks: rb_node,
    pub mm: *mut mm_struct,
    pub active_mm: *mut mm_struct,
    pub exec_state: *mut task_exec_state,
    pub exit_state: ::core::ffi::c_int,
    pub exit_code: ::core::ffi::c_int,
    pub exit_signal: ::core::ffi::c_int,
    pub pdeath_signal: ::core::ffi::c_int,
    pub jobctl: ::core::ffi::c_ulong,
    pub personality: ::core::ffi::c_uint,
    #[bitfield(
        name = "sched_reset_on_fork",
        ty = "::core::ffi::c_uint",
        bits = "0..=0"
    )]
    #[bitfield(
        name = "sched_contributes_to_load",
        ty = "::core::ffi::c_uint",
        bits = "1..=1"
    )]
    #[bitfield(name = "sched_migrated", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "sched_task_hot", ty = "::core::ffi::c_uint", bits = "3..=3")]
    pub sched_reset_on_fork_sched_contributes_to_load_sched_migrated_sched_task_hot: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    #[bitfield(
        name = "sched_remote_wakeup",
        ty = "::core::ffi::c_uint",
        bits = "0..=0"
    )]
    #[bitfield(name = "in_execve", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "in_iowait", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "in_nf_duplicate", ty = "::core::ffi::c_uint", bits = "3..=3")]
    pub sched_remote_wakeup_in_execve_in_iowait_in_nf_duplicate: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
    pub atomic_flags: ::core::ffi::c_ulong,
    pub restart_block: restart_block,
    pub pid: pid_t,
    pub tgid: pid_t,
    pub real_parent: *mut task_struct,
    pub parent: *mut task_struct,
    pub children: list_head,
    pub sibling: list_head,
    pub group_leader: *mut task_struct,
    pub ptraced: list_head,
    pub ptrace_entry: list_head,
    pub thread_pid: *mut pid,
    pub pid_links: [hlist_node; 4],
    pub thread_node: list_head,
    pub vfork_done: *mut completion,
    pub set_child_tid: *mut ::core::ffi::c_int,
    pub clear_child_tid: *mut ::core::ffi::c_int,
    pub worker_private: *mut ::core::ffi::c_void,
    pub utime: u64_0,
    pub stime: u64_0,
    pub gtime: u64_0,
    pub prev_cputime: prev_cputime,
    pub nvcsw: ::core::ffi::c_ulong,
    pub nivcsw: ::core::ffi::c_ulong,
    pub start_time: u64_0,
    pub start_boottime: u64_0,
    pub min_flt: ::core::ffi::c_ulong,
    pub maj_flt: ::core::ffi::c_ulong,
    pub posix_cputimers: posix_cputimers,
    pub ptracer_cred: *const cred,
    pub real_cred: *const cred,
    pub cred: *const cred,
    pub comm: [::core::ffi::c_char; 16],
    pub nameidata: *mut nameidata,
    pub fs: *mut fs_struct,
    pub files: *mut files_struct,
    pub nsproxy: *mut nsproxy,
    pub signal: *mut signal_struct,
    pub sighand: *mut sighand_struct,
    pub blocked: sigset_t,
    pub real_blocked: sigset_t,
    pub saved_sigmask: sigset_t,
    pub pending: sigpending,
    pub sas_ss_sp: ::core::ffi::c_ulong,
    pub sas_ss_size: size_t,
    pub sas_ss_flags: ::core::ffi::c_uint,
    pub task_works: *mut callback_head,
    pub seccomp: seccomp,
    pub syscall_dispatch: syscall_user_dispatch,
    pub parent_exec_id: u64_0,
    pub self_exec_id: u64_0,
    pub alloc_lock: spinlock_t,
    pub pi_lock: raw_spinlock_t,
    pub wake_q: wake_q_node,
    pub blocked_on: *mut mutex,
    pub blocked_lock: raw_spinlock_t,
    pub blocked_donor: *mut task_struct,
    pub journal_info: *mut ::core::ffi::c_void,
    pub bio_list: *mut bio_list,
    pub plug: *mut blk_plug,
    pub reclaim_state: *mut reclaim_state,
    pub io_context: *mut io_context,
    pub ptrace_message: ::core::ffi::c_ulong,
    pub last_siginfo: *mut kernel_siginfo_t,
    pub ioac: task_io_accounting,
    pub futex: futex_sched_data,
    pub perf_recursion: [u8_0; 4],
    pub perf_event_ctxp: *mut perf_event_context,
    pub perf_event_mutex: mutex,
    pub perf_event_list: list_head,
    pub perf_ctx_data: *mut perf_ctx_data,
    pub rseq: rseq_data,
    pub mm_cid: sched_mm_cid,
    pub tlb_ubc: tlbflush_unmap_batch,
    pub splice_pipe: *mut pipe_inode_info,
    pub task_frag: page_frag,
    pub nr_dirtied: ::core::ffi::c_int,
    pub nr_dirtied_pause: ::core::ffi::c_int,
    pub dirty_paused_when: ::core::ffi::c_ulong,
    pub timer_slack_ns: u64_0,
    pub default_timer_slack_ns: u64_0,
    pub kunit_test: *mut kunit,
    pub kmap_ctrl: kmap_ctrl,
    pub rcu: callback_head,
    pub rcu_users: refcount_t,
    pub pagefault_disabled: ::core::ffi::c_int,
    pub oom_reaper_list: *mut task_struct,
    pub oom_reaper_timer: timer_list,
    pub stack_refcount: refcount_t,
    pub bpf_net_context: *mut bpf_net_context,
    pub thread: thread_struct,
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
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct sched_dl_entity {
    pub rb_node: rb_node,
    pub dl_runtime: u64_0,
    pub dl_deadline: u64_0,
    pub dl_period: u64_0,
    pub dl_bw: u64_0,
    pub dl_density: u64_0,
    pub runtime: s64,
    pub deadline: u64_0,
    pub flags: ::core::ffi::c_uint,
    #[bitfield(name = "dl_throttled", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "dl_yielded", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "dl_non_contending", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "dl_overrun", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "dl_server", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "dl_server_active", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "dl_defer", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "dl_defer_armed", ty = "::core::ffi::c_uint", bits = "7..=7")]
    #[bitfield(name = "dl_defer_running", ty = "::core::ffi::c_uint", bits = "8..=8")]
    #[bitfield(name = "dl_defer_idle", ty = "::core::ffi::c_uint", bits = "9..=9")]
    #[bitfield(name = "dl_bw_attached", ty = "::core::ffi::c_uint", bits = "10..=10")]
    pub dl_throttled_dl_yielded_dl_non_contending_dl_overrun_dl_server_dl_server_active_dl_defer_dl_defer_armed_dl_defer_running_dl_defer_idle_dl_bw_attached:
        [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub dl_timer: hrtimer,
    pub inactive_timer: hrtimer,
    pub rq: *mut rq,
    pub server_pick_task: dl_server_pick_f,
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
pub struct C2Rust_Unnamed_19 {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {}
#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct C2Rust_Unnamed_21(pub C2Rust_C2Rust_Unnamed_21_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_C2Rust_Unnamed_21_Inner {
    pub x: ::core::ffi::c_long,
}
#[allow(dead_code, non_upper_case_globals)]
const C2Rust_C2Rust_Unnamed_21_PADDING: usize = ::core::mem::size_of::<C2Rust_Unnamed_21>()
    - ::core::mem::size_of::<C2Rust_C2Rust_Unnamed_21_Inner>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn kasan_check_read(
    mut p: *const ::core::ffi::c_void,
    mut size: ::core::ffi::c_uint,
) -> bool_0 {
    unsafe {
        return r#true as ::core::ffi::c_int != 0;
    }
}
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
        return __riscv_isa_extension_available(::core::ptr::null(), ext as ::core::ffi::c_uint);
    }
}
#[inline(always)]
unsafe extern "C" fn arch_atomic_read(mut v: *const atomic_t) -> ::core::ffi::c_int {
    unsafe {
        return ({
            extern "C" {
                #[link_name = "__compiletime_assert_3"]
                fn __compiletime_assert_3_0() -> !;
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
                __compiletime_assert_3_0();
            }
            ::core::ptr::read_volatile::<::core::ffi::c_int>(
                (&raw const (*v).counter as *const ::core::ffi::c_int),
            )
        });
    }
}
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
unsafe extern "C" fn raw_atomic_read(mut v: *const atomic_t) -> ::core::ffi::c_int {
    unsafe {
        return arch_atomic_read(v);
    }
}
#[inline(always)]
unsafe extern "C" fn raw_atomic_set(mut v: *mut atomic_t, mut i: ::core::ffi::c_int) {
    unsafe {
        arch_atomic_set(v, i);
    }
}
#[inline(always)]
unsafe extern "C" fn raw_atomic_cmpxchg_release(
    mut v: *mut atomic_t,
    mut old: ::core::ffi::c_int,
    mut new: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return ({
            let mut __ptr: *mut ::core::ffi::c_int = &raw mut (*v).counter;
            let mut __old: ::core::ffi::c_int = old as ::core::ffi::c_int;
            let mut __new: ::core::ffi::c_int = new as ::core::ffi::c_int;
            let mut __ret: ::core::ffi::c_int = 0;
            match ::core::mem::size_of::<::core::ffi::c_int>() {
                1 => {
                    '_c2rust_label_0: {
                        if true
                            && true
                            && true
                            && riscv_has_extension_unlikely(
                                RISCV_ISA_EXT_ZABHA as ::core::ffi::c_ulong,
                            ) as ::core::ffi::c_int
                                != 0
                            && riscv_has_extension_unlikely(
                                RISCV_ISA_EXT_ZACAS as ::core::ffi::c_ulong,
                            ) as ::core::ffi::c_int
                                != 0
                        {
                            __ret = __old as ::core::ffi::c_int;
                            asm!(
                                "\tamocas.b {0}, {2:}, 0({1})\n", "\n", inout(reg) __ret, in
                                (reg) & raw mut * __ptr, inlateout(reg) __new => _,
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
                                & (0x4 as ulong)
                                    .wrapping_sub(
                                        ::core::mem::size_of::<::core::ffi::c_int>() as ulong
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
                                                ::core::mem::size_of::<::core::ffi::c_int>()
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
                                "\tor   {1}, {1}, {4:}\n", "\tsc.w {1}, {1}, 0({2})\n",
                                "\tbnez {1}, 0b\n", "1:\n", "\n", out(reg) __retx, out(reg)
                                __rc, in (reg) & raw mut * __ptr32b, inlateout(reg) __oldx
                                as ::core::ffi::c_long => _, inlateout(reg) __newx => _,
                                inlateout(reg) __mask => _, inlateout(reg) ! __mask => _,
                                options(preserves_flags)
                            );
                            __ret = ((__retx & __mask) >> __s) as ::core::ffi::c_int
                                as ::core::ffi::c_int;
                        }
                    };
                }
                2 => {
                    '_c2rust_label_1: {
                        if true
                            && true
                            && true
                            && riscv_has_extension_unlikely(
                                RISCV_ISA_EXT_ZABHA as ::core::ffi::c_ulong,
                            ) as ::core::ffi::c_int
                                != 0
                            && riscv_has_extension_unlikely(
                                RISCV_ISA_EXT_ZACAS as ::core::ffi::c_ulong,
                            ) as ::core::ffi::c_int
                                != 0
                        {
                            __ret = __old as ::core::ffi::c_int;
                            asm!(
                                "\tamocas.h {0}, {2:}, 0({1})\n", "\n", inout(reg) __ret, in
                                (reg) & raw mut * __ptr, inlateout(reg) __new => _,
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
                                & (0x4 as ulong)
                                    .wrapping_sub(
                                        ::core::mem::size_of::<::core::ffi::c_int>() as ulong
                                    ))
                            .wrapping_mul(BITS_PER_BYTE as ulong);
                            let mut __mask: ulong = (::core::mem::size_of::<C2Rust_Unnamed_19>()
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
                                                ::core::mem::size_of::<::core::ffi::c_int>()
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
                                "\tor   {1}, {1}, {4:}\n", "\tsc.w {1}, {1}, 0({2})\n",
                                "\tbnez {1}, 0b\n", "1:\n", "\n", out(reg) __retx, out(reg)
                                __rc, in (reg) & raw mut * __ptr32b, inlateout(reg) __oldx
                                as ::core::ffi::c_long => _, inlateout(reg) __newx => _,
                                inlateout(reg) __mask => _, inlateout(reg) ! __mask => _,
                                options(preserves_flags)
                            );
                            __ret = ((__retx & __mask) >> __s) as ::core::ffi::c_int
                                as ::core::ffi::c_int;
                        }
                    };
                }
                4 => {
                    '_c2rust_label_2: {
                        if true
                            && true
                            && riscv_has_extension_unlikely(
                                RISCV_ISA_EXT_ZACAS as ::core::ffi::c_ulong,
                            ) as ::core::ffi::c_int
                                != 0
                        {
                            __ret = __old as ::core::ffi::c_int;
                            asm!(
                                "\tamocas.w {0}, {2:}, 0({1})\n", "\n", inout(reg) __ret, in
                                (reg) & raw mut * __ptr, inlateout(reg) __new => _,
                                options(preserves_flags)
                            );
                        } else {
                            let mut __rc: ::core::ffi::c_uint = 0;
                            asm!(
                                "0:\tlr.w {0}, 0({2})\n", "\tbne  {0}, {3:}, 1f\n",
                                "\tsc.w {1}, {4:}, 0({2})\n", "\tbnez {1}, 0b\n", "1:\n",
                                "\n", out(reg) __ret, out(reg) __rc, in (reg) & raw mut *
                                __ptr, inlateout(reg) __old as ::core::ffi::c_long as
                                ::core::ffi::c_int as ::core::ffi::c_long => _,
                                inlateout(reg) __new => _, options(preserves_flags)
                            );
                        }
                    };
                }
                8 => {
                    '_c2rust_label_3: {
                        if true
                            && true
                            && riscv_has_extension_unlikely(
                                RISCV_ISA_EXT_ZACAS as ::core::ffi::c_ulong,
                            ) as ::core::ffi::c_int
                                != 0
                        {
                            __ret = __old as ::core::ffi::c_int;
                            asm!(
                                "\tamocas.d {0}, {2:}, 0({1})\n", "\n", inout(reg) __ret, in
                                (reg) & raw mut * __ptr, inlateout(reg) __new => _,
                                options(preserves_flags)
                            );
                        } else {
                            let mut __rc: ::core::ffi::c_uint = 0;
                            asm!(
                                "0:\tlr.d {0}, 0({2})\n", "\tbne  {0}, {3:}, 1f\n",
                                "\tsc.d {1}, {4:}, 0({2})\n", "\tbnez {1}, 0b\n", "1:\n",
                                "\n", out(reg) __ret, out(reg) __rc, in (reg) & raw mut *
                                __ptr, inlateout(reg) __old => _, inlateout(reg) __new => _,
                                options(preserves_flags)
                            );
                        }
                    };
                }
                _ => {
                    extern "C" {
                        #[link_name = "__compiletime_assert_15"]
                        fn __compiletime_assert_15_0() -> !;
                    }
                    if true {
                        __compiletime_assert_15_0();
                    }
                }
            }
            __ret
        });
    }
}
#[inline(always)]
unsafe extern "C" fn raw_atomic_try_cmpxchg_release(
    mut v: *mut atomic_t,
    mut old: *mut ::core::ffi::c_int,
    mut new: ::core::ffi::c_int,
) -> bool_0 {
    unsafe {
        let mut r: ::core::ffi::c_int = 0;
        let mut o: ::core::ffi::c_int = *old;
        r = raw_atomic_cmpxchg_release(v, o, new);
        if (r != o) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            *old = r;
        }
        return (r == o) as ::core::ffi::c_int as ::core::ffi::c_long != 0;
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
unsafe extern "C" fn instrument_atomic_read(mut v: *const ::core::ffi::c_void, mut size: size_t) {
    unsafe {
        kasan_check_read(v, size as ::core::ffi::c_uint);
        kcsan_check_access(v, size, KCSAN_ACCESS_ATOMIC);
        instrument_atomic_check_alignment(v, size);
    }
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
unsafe extern "C" fn atomic_read(mut v: *const atomic_t) -> ::core::ffi::c_int {
    unsafe {
        instrument_atomic_read(
            v as *const ::core::ffi::c_void,
            ::core::mem::size_of::<atomic_t>() as size_t,
        );
        return raw_atomic_read(v);
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
unsafe extern "C" fn atomic_try_cmpxchg_release(
    mut v: *mut atomic_t,
    mut old: *mut ::core::ffi::c_int,
    mut new: ::core::ffi::c_int,
) -> bool_0 {
    unsafe {
        instrument_atomic_read_write(
            v as *const ::core::ffi::c_void,
            ::core::mem::size_of::<atomic_t>() as size_t,
        );
        instrument_read_write(
            old as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        );
        return raw_atomic_try_cmpxchg_release(v, old, new);
    }
}
pub const RCUREF_MAXREF: ::core::ffi::c_uint = 0x7fffffff as ::core::ffi::c_uint;
pub const RCUREF_SATURATED: ::core::ffi::c_uint = 0xa0000000 as ::core::ffi::c_uint;
pub const RCUREF_RELEASED: ::core::ffi::c_uint = 0xc0000000 as ::core::ffi::c_uint;
pub const RCUREF_DEAD: ::core::ffi::c_uint = 0xe0000000 as ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn rcuref_get_slowpath(mut r#ref: *mut rcuref_t) -> bool_0 {
    unsafe {
        let mut cnt: ::core::ffi::c_uint =
            atomic_read(&raw mut (*r#ref).refcnt) as ::core::ffi::c_uint;
        if cnt >= RCUREF_RELEASED {
            atomic_set(&raw mut (*r#ref).refcnt, RCUREF_DEAD as ::core::ffi::c_int);
            return r#false as ::core::ffi::c_int != 0;
        }
        if kernel::warn_on!(cnt > 0x7fffffff as ::core::ffi::c_uint) {
            atomic_set(
                &raw mut (*r#ref).refcnt,
                RCUREF_SATURATED as ::core::ffi::c_int,
            );
        }
        return r#true as ::core::ffi::c_int != 0;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_rcuref_get_slowpath: ; .asciz \"GPL\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad rcuref_get_slowpath ; .previous"
);
#[no_mangle]
pub unsafe extern "C" fn rcuref_put_slowpath(
    mut r#ref: *mut rcuref_t,
    mut cnt: ::core::ffi::c_uint,
) -> bool_0 {
    unsafe {
        if (cnt == 0xffffffff as ::core::ffi::c_uint) as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            if !atomic_try_cmpxchg_release(
                &raw mut (*r#ref).refcnt,
                &raw mut cnt as *mut ::core::ffi::c_int,
                RCUREF_DEAD as ::core::ffi::c_int,
            ) {
                return r#false as ::core::ffi::c_int != 0;
            }
            asm!("\n", options(preserves_flags));
            return r#true as ::core::ffi::c_int != 0;
        }
        if kernel::warn_on!(cnt >= 0xc0000000 as ::core::ffi::c_uint) {
            atomic_set(&raw mut (*r#ref).refcnt, RCUREF_DEAD as ::core::ffi::c_int);
            return r#false as ::core::ffi::c_int != 0;
        }
        if cnt > RCUREF_MAXREF {
            atomic_set(
                &raw mut (*r#ref).refcnt,
                RCUREF_SATURATED as ::core::ffi::c_int,
            );
        }
        return r#false as ::core::ffi::c_int != 0;
    }
}
global_asm!(
    ".section \".export_symbol\",\"a\" ; __export_symbol_rcuref_put_slowpath: ; .asciz \"GPL\" ; .ascii \"\" \"\\0\" ; .balign 8 ; .quad rcuref_put_slowpath ; .previous"
);
