use crate::int;

// POSIX.1

pub const _SC_ARG_MAX: int = 1;
pub const _SC_CHILD_MAX: int = 2;
pub const _SC_CLK_TCK: int = 3;
pub const _SC_NGROUPS_MAX: int = 4;
pub const _SC_OPEN_MAX: int = 5;
pub const _SC_JOB_CONTROL: int = 6;
pub const _SC_SAVED_IDS: int = 7;
pub const _SC_VERSION: int = 8;
pub const _SC_BC_BASE_MAX: int = 9;
pub const _SC_BC_DIM_MAX: int = 10;
pub const _SC_BC_SCALE_MAX: int = 11;
pub const _SC_BC_STRING_MAX: int = 12;
pub const _SC_COLL_WEIGHTS_MAX: int = 13;
pub const _SC_EXPR_NEST_MAX: int = 14;
pub const _SC_LINE_MAX: int = 15;
pub const _SC_RE_DUP_MAX: int = 16;
pub const _SC_2_VERSION: int = 17;
pub const _SC_2_C_BIND: int = 18;
pub const _SC_2_C_DEV: int = 19;
pub const _SC_2_CHAR_TERM: int = 20;
pub const _SC_2_FORT_DEV: int = 21;
pub const _SC_2_FORT_RUN: int = 22;
pub const _SC_2_LOCALEDEF: int = 23;
pub const _SC_2_SW_DEV: int = 24;
pub const _SC_2_UPE: int = 25;
pub const _SC_STREAM_MAX: int = 26;
pub const _SC_TZNAME_MAX: int = 27;
pub const _SC_ASYNCHRONOUS_IO: int = 28;
pub const _SC_PAGESIZE: int = 29;
pub const _SC_MEMLOCK: int = 30;
pub const _SC_MEMLOCK_RANGE: int = 31;
pub const _SC_MEMORY_PROTECTION: int = 32;
pub const _SC_MESSAGE_PASSING: int = 33;
pub const _SC_PRIORITIZED_IO: int = 34;
pub const _SC_PRIORITY_SCHEDULING: int = 35;
pub const _SC_REALTIME_SIGNALS: int = 36;
pub const _SC_SEMAPHORES: int = 37;
pub const _SC_FSYNC: int = 38;
pub const _SC_SHARED_MEMORY_OBJECTS: int = 39;
pub const _SC_SYNCHRONIZED_IO: int = 40;
pub const _SC_TIMERS: int = 41;
pub const _SC_AIO_LISTIO_MAX: int = 42;
pub const _SC_AIO_MAX: int = 43;
pub const _SC_AIO_PRIO_DELTA_MAX: int = 44;
pub const _SC_DELAYTIMER_MAX: int = 45;
pub const _SC_MQ_OPEN_MAX: int = 46;
pub const _SC_MAPPED_FILES: int = 47;
pub const _SC_RTSIG_MAX: int = 48;
pub const _SC_SEM_NSEMS_MAX: int = 49;
pub const _SC_SEM_VALUE_MAX: int = 50;
pub const _SC_SIGQUEUE_MAX: int = 51;
pub const _SC_TIMER_MAX: int = 52;
pub const _SC_IOV_MAX: int = 56;
pub const _SC_NPROCESSORS_CONF: int = 57;
pub const _SC_NPROCESSORS_ONLN: int = 58;
pub const _SC_2_PBS: int = 59;
pub const _SC_2_PBS_ACCOUNTING: int = 60;
pub const _SC_2_PBS_CHECKPOINT: int = 61;
pub const _SC_2_PBS_LOCATE: int = 62;
pub const _SC_2_PBS_MESSAGE: int = 63;
pub const _SC_2_PBS_TRACK: int = 64;
pub const _SC_ADVISORY_INFO: int = 65;
pub const _SC_BARRIERS: int = 66;
pub const _SC_CLOCK_SELECTION: int = 67;
pub const _SC_CPUTIME: int = 68;
pub const _SC_FILE_LOCKING: int = 69;
pub const _SC_GETGR_R_SIZE_MAX: int = 70;
pub const _SC_GETPW_R_SIZE_MAX: int = 71;
pub const _SC_HOST_NAME_MAX: int = 72;
pub const _SC_LOGIN_NAME_MAX: int = 73;
pub const _SC_MONOTONIC_CLOCK: int = 74;
pub const _SC_MQ_PRIO_MAX: int = 75;
pub const _SC_READER_WRITER_LOCKS: int = 76;
pub const _SC_REGEXP: int = 77;
pub const _SC_SHELL: int = 78;
pub const _SC_SPAWN: int = 79;
pub const _SC_SPIN_LOCKS: int = 80;
pub const _SC_SPORADIC_SERVER: int = 81;
pub const _SC_THREAD_ATTR_STACKADDR: int = 82;
pub const _SC_THREAD_ATTR_STACKSIZE: int = 83;
pub const _SC_THREAD_CPUTIME: int = 84;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: int = 85;
pub const _SC_THREAD_KEYS_MAX: int = 86;
pub const _SC_THREAD_PRIO_INHERIT: int = 87;
pub const _SC_THREAD_PRIO_PROTECT: int = 88;
pub const _SC_THREAD_PRIORITY_SCHEDULING: int = 89;
pub const _SC_THREAD_PROCESS_SHARED: int = 90;
pub const _SC_THREAD_SAFE_FUNCTIONS: int = 91;
pub const _SC_THREAD_SPORADIC_SERVER: int = 92;
pub const _SC_THREAD_STACK_MIN: int = 93;
pub const _SC_THREAD_THREADS_MAX: int = 94;
pub const _SC_TIMEOUTS: int = 95;
pub const _SC_THREADS: int = 96;
pub const _SC_TRACE: int = 97;
pub const _SC_TRACE_EVENT_FILTER: int = 98;
pub const _SC_TRACE_INHERIT: int = 99;
pub const _SC_TRACE_LOG: int = 100;
pub const _SC_TTY_NAME_MAX: int = 101;
pub const _SC_TYPED_MEMORY_OBJECTS: int = 102;
pub const _SC_V6_ILP32_OFF32: int = 103;
pub const _SC_V6_ILP32_OFFBIG: int = 104;
pub const _SC_V6_LP64_OFF64: int = 105;
pub const _SC_V6_LPBIG_OFFBIG: int = 106;
pub const _SC_ATEXIT_MAX: int = 107;
pub const _SC_XOPEN_CRYPT: int = 108;
pub const _SC_XOPEN_ENH_I18N: int = 109;
pub const _SC_XOPEN_LEGACY: int = 110;
pub const _SC_XOPEN_REALTIME: int = 111;
pub const _SC_XOPEN_REALTIME_THREADS: int = 112;
pub const _SC_XOPEN_SHM: int = 113;
pub const _SC_XOPEN_UNIX: int = 115;
pub const _SC_XOPEN_VERSION: int = 116;
pub const _SC_IPV6: int = 118;
pub const _SC_RAW_SOCKETS: int = 119;
pub const _SC_SYMLOOP_MAX: int = 120;
pub const _SC_XOPEN_XCU_VERSION: int = 121;
pub const _SC_PAGE_SIZE: int = _SC_PAGESIZE;
pub const _SC_XOPEN_STREAMS: int = 114;
pub const _SC_XBS5_ILP32_OFF32: int = 122;
pub const _SC_XBS5_ILP32_OFFBIG: int = 123;
pub const _SC_XBS5_LP64_OFF64: int = 124;
pub const _SC_XBS5_LPBIG_OFFBIG: int = 125;
pub const _SC_SS_REPL_MAX: int = 126;
pub const _SC_TRACE_EVENT_NAME_MAX: int = 127;
pub const _SC_TRACE_NAME_MAX: int = 128;
pub const _SC_TRACE_SYS_MAX: int = 129;
pub const _SC_TRACE_USER_EVENT_MAX: int = 130;
pub const _SC_PASS_MAX: int = 131;
