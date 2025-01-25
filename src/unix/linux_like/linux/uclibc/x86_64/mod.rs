use crate::prelude::*;

pub type c_char = i8;
pub type wchar_t = c_int;
pub type c_long = i64;
pub type c_ulong = u64;
pub type time_t = c_long;

pub type clock_t = c_long;
pub type fsblkcnt_t = c_ulong;
pub type fsfilcnt_t = c_ulong;
pub type fsword_t = c_long;
pub type ino_t = c_ulong;
pub type off_t = c_long;
pub type suseconds_t = c_long;
// pthread_t is defined in l4re.rs/other.rs
pub type nlink_t = c_uint;
pub type blksize_t = c_long;
pub type blkcnt_t = c_long;

// [uClibc docs] Note stat64 has the same shape as stat for x86-64.
pub type stat64 = stat;
pub type fsblkcnt64_t = u64;
pub type fsfilcnt64_t = u64;
pub type __u64 = c_ulong;
pub type __s64 = c_long;

s! {
    pub struct cmsghdr {
        pub cmsg_len: crate::size_t,
        pub cmsg_level: c_int,
        pub cmsg_type: c_int,
    }

    pub struct msghdr {
        pub msg_name: *mut c_void,
        pub msg_namelen: crate::socklen_t,
        pub msg_iov: *mut crate::iovec,
        pub msg_iovlen: crate::size_t,
        pub msg_control: *mut c_void,
        pub msg_controllen: crate::size_t,
        pub msg_flags: c_int,
    }

    // pthread_attr_t is defined in l4re.rs/other.rs

    pub struct stat {
        pub st_dev: c_ulong,
        pub st_ino: crate::ino_t,
        // According to uclibc/libc/sysdeps/linux/x86_64/bits/stat.h, order of
        // nlink and mode are swapped on 64 bit systems.
        pub st_nlink: crate::nlink_t,
        pub st_mode: crate::mode_t,
        pub st_uid: crate::uid_t,
        pub st_gid: crate::gid_t,
        pub st_rdev: c_ulong, // dev_t
        pub st_size: off_t,   // file size
        pub st_blksize: crate::blksize_t,
        pub st_blocks: crate::blkcnt_t,
        pub st_atime: crate::time_t,
        pub st_atime_nsec: c_ulong,
        pub st_mtime: crate::time_t,
        pub st_mtime_nsec: c_ulong,
        pub st_ctime: crate::time_t,
        pub st_ctime_nsec: c_ulong,
        st_pad4: [c_long; 3],
    }

    pub struct flock {
        pub l_type: crate::c_short,
        pub l_whence: crate::c_short,
        pub l_start: off_t,
        pub l_len: off_t,
        pub l_pid: crate::pid_t,
    }

    pub struct sysinfo {
        pub uptime: c_long,
        pub loads: [c_ulong; 3],
        pub totalram: c_ulong,
        pub freeram: c_ulong,
        pub sharedram: c_ulong,
        pub bufferram: c_ulong,
        pub totalswap: c_ulong,
        pub freeswap: c_ulong,
        pub procs: c_ushort,
        pub pad: c_ushort,
        pub totalhigh: c_ulong,
        pub freehigh: c_ulong,
        pub mem_unit: c_uint,
        pub _f: [c_char; 0],
    }

    pub struct fsid_t {
        __val: [c_int; 2],
    }

    pub struct statfs {
        pub f_type: fsword_t,
        pub f_bsize: fsword_t,
        pub f_blocks: crate::fsblkcnt_t,
        pub f_bfree: crate::fsblkcnt_t,
        pub f_bavail: crate::fsblkcnt_t,
        pub f_files: crate::fsfilcnt_t,
        pub f_ffree: crate::fsfilcnt_t,
        pub f_fsid: crate::fsid_t,
        pub f_namelen: fsword_t,
        pub f_frsize: fsword_t,
        f_spare: [fsword_t; 4],
    }

    pub struct statfs64 {
        pub f_type: fsword_t,
        pub f_bsize: fsword_t,
        pub f_blocks: crate::fsblkcnt64_t,
        pub f_bfree: crate::fsblkcnt64_t,
        pub f_bavail: crate::fsblkcnt64_t,
        pub f_files: crate::fsfilcnt64_t,
        pub f_ffree: crate::fsfilcnt64_t,
        pub f_fsid: crate::fsid_t,
        pub f_namelen: fsword_t,
        pub f_frsize: fsword_t,
        pub f_flags: fsword_t,
        pub f_spare: [fsword_t; 4],
    }

    pub struct statvfs64 {
        pub f_bsize: c_ulong,
        pub f_frsize: c_ulong,
        pub f_blocks: u64,
        pub f_bfree: u64,
        pub f_bavail: u64,
        pub f_files: u64,
        pub f_ffree: u64,
        pub f_favail: u64,
        pub f_fsid: c_ulong,
        __f_unused: c_int,
        pub f_flag: c_ulong,
        pub f_namemax: c_ulong,
        __f_spare: [c_int; 6],
    }

    pub struct sigset_t {
        __val: [c_ulong; 1],
    }

    pub struct sigaction {
        pub sa_handler: crate::sighandler_t,
        pub sa_flags: c_ulong,
        pub sa_restorer: Option<extern "C" fn()>,
        pub sa_mask: crate::sigset_t,
    }

    pub struct termios {
        pub c_iflag: crate::tcflag_t,
        pub c_oflag: crate::tcflag_t,
        pub c_cflag: crate::tcflag_t,
        pub c_lflag: crate::tcflag_t,
        pub c_line: crate::cc_t,
        pub c_cc: [crate::cc_t; crate::NCCS],
        pub c_ispeed: crate::speed_t,
        pub c_ospeed: crate::speed_t,
    }

    pub struct siginfo_t {
        pub si_signo: c_int,
        pub si_errno: c_int,
        pub si_code: c_int,
        pub _pad: [c_int; 28],
    }

    pub struct stack_t {
        pub ss_sp: *mut c_void,
        pub ss_flags: c_int,
        pub ss_size: crate::size_t,
    }

    pub struct ipc_perm {
        pub __key: crate::key_t,
        pub uid: crate::uid_t,
        pub gid: crate::gid_t,
        pub cuid: crate::uid_t,
        pub cgid: crate::gid_t,
        pub mode: c_ushort,
        __pad1: c_ushort,
        pub __seq: c_ushort,
        __pad2: c_ushort,
        __unused1: c_ulong,
        __unused2: c_ulong,
    }

    pub struct msqid_ds {
        pub msg_perm: crate::ipc_perm,
        pub msg_stime: crate::time_t,
        pub msg_rtime: crate::time_t,
        pub msg_ctime: crate::time_t,
        __msg_cbytes: c_ulong,
        pub msg_qnum: crate::msgqnum_t,
        pub msg_qbytes: crate::msglen_t,
        pub msg_lspid: crate::pid_t,
        pub msg_lrpid: crate::pid_t,
        __ignored1: c_ulong,
        __ignored2: c_ulong,
    }

    pub struct shmid_ds {
        pub shm_perm: crate::ipc_perm,
        pub shm_segsz: crate::size_t,
        pub shm_atime: crate::time_t,
        pub shm_dtime: crate::time_t,
        pub shm_ctime: crate::time_t,
        pub shm_cpid: crate::pid_t,
        pub shm_lpid: crate::pid_t,
        pub shm_nattch: crate::shmatt_t,
        __unused4: c_ulong,
        __unused5: c_ulong,
    }

    pub struct __sched_param {
        __sched_priority: c_int,
    }

    pub struct sockaddr {
        pub sa_family: crate::sa_family_t,
        pub sa_data: [c_char; 14],
    }

    pub struct sockaddr_in {
        pub sin_family: crate::sa_family_t,
        pub sin_port: crate::in_port_t,
        pub sin_addr: crate::in_addr,
        pub sin_zero: [u8; 8],
    }

    pub struct sockaddr_in6 {
        pub sin6_family: crate::sa_family_t,
        pub sin6_port: crate::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: crate::in6_addr,
        pub sin6_scope_id: u32,
    }

    pub struct glob_t {
        pub gl_pathc: crate::size_t,
        pub gl_pathv: *mut *mut c_char,
        pub gl_offs: crate::size_t,
        pub gl_flags: c_int,
        __unused1: *mut c_void,
        __unused2: *mut c_void,
        __unused3: *mut c_void,
        __unused4: *mut c_void,
        __unused5: *mut c_void,
    }

    pub struct cpu_set_t {
        bits: [u64; 16],
    }

    pub struct sem_t {
        __size: [c_char; 32],
        __align: c_long,
    }
}

s_no_extra_traits! {
    #[allow(missing_debug_implementations)]
    pub struct dirent {
        pub d_ino: crate::ino64_t,
        pub d_off: crate::off64_t,
        pub d_reclen: u16,
        pub d_type: u8,
        pub d_name: [c_char; 256],
    }
}

// constants
pub const ENAMETOOLONG: c_int = 36; // File name too long
pub const ENOTEMPTY: c_int = 39; // Directory not empty
pub const ELOOP: c_int = 40; // Too many symbolic links encountered
pub const EADDRINUSE: c_int = 98; // Address already in use
pub const EADDRNOTAVAIL: c_int = 99; // Cannot assign requested address
pub const ENETDOWN: c_int = 100; // Network is down
pub const ENETUNREACH: c_int = 101; // Network is unreachable
pub const ECONNABORTED: c_int = 103; // Software caused connection abort
pub const ECONNREFUSED: c_int = 111; // Connection refused
pub const ECONNRESET: c_int = 104; // Connection reset by peer
pub const EDEADLK: c_int = 35; // Resource deadlock would occur
pub const ENOSYS: c_int = 38; // Function not implemented
pub const ENOTCONN: c_int = 107; // Transport endpoint is not connected
pub const ETIMEDOUT: c_int = 110; // connection timed out
pub const ESTALE: c_int = 116; // Stale file handle
pub const EHOSTUNREACH: c_int = 113; // No route to host
pub const EDQUOT: c_int = 122; // Quota exceeded
pub const EOPNOTSUPP: c_int = 0x5f;
pub const ENODATA: c_int = 0x3d;
pub const O_APPEND: c_int = 0o2000;
pub const O_ACCMODE: c_int = 0o003;
pub const O_CLOEXEC: c_int = 0x80000;
pub const O_CREAT: c_int = 0100;
pub const O_DIRECTORY: c_int = 0o200000;
pub const O_EXCL: c_int = 0o200;
pub const O_NOFOLLOW: c_int = 0x20000;
pub const O_NONBLOCK: c_int = 0o4000;
pub const O_TRUNC: c_int = 0o1000;
pub const NCCS: usize = 32;
pub const SIG_SETMASK: c_int = 2; // Set the set of blocked signals
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 40;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;
pub const SOCK_DGRAM: c_int = 2; // connectionless, unreliable datagrams
pub const SOCK_STREAM: c_int = 1; // …/common/bits/socket_type.h
pub const __SIZEOF_PTHREAD_COND_T: usize = 48;
pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: usize = 8;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: usize = 4;
pub const PIDFD_NONBLOCK: c_int = 0o4000;

pub const __SIZEOF_PTHREAD_ATTR_T: usize = 36;
pub const __SIZEOF_PTHREAD_COND_COMPAT_T: usize = 12;
pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 32;
pub const __SIZEOF_PTHREAD_BARRIER_T: usize = 20;
pub const MAP_HUGETLB: c_int = 0x040000;

// from linux/other/mod.rs

// autogenerated constants with hand tuned types

pub const B0: crate::speed_t = 0;
pub const B1000000: crate::speed_t = 0x1008;
pub const B110: crate::speed_t = 0x3;
pub const B115200: crate::speed_t = 0x1002;
pub const B1152000: crate::speed_t = 0x1009;
pub const B1200: crate::speed_t = 0x9;
pub const B134: crate::speed_t = 0x4;
pub const B150: crate::speed_t = 0x5;
pub const B1500000: crate::speed_t = 0x100a;
pub const B1800: crate::speed_t = 0xa;
pub const B19200: crate::speed_t = 0xe;
pub const B200: crate::speed_t = 0x6;
pub const B2000000: crate::speed_t = 0x100b;
pub const B230400: crate::speed_t = 0x1003;
pub const B2400: crate::speed_t = 0xb;
pub const B2500000: crate::speed_t = 0x100c;
pub const B300: crate::speed_t = 0x7;
pub const B3000000: crate::speed_t = 0x100d;
pub const B3500000: crate::speed_t = 0x100e;
pub const B38400: crate::speed_t = 0xf;
pub const B4000000: crate::speed_t = 0x100f;
pub const B460800: crate::speed_t = 0x1004;
pub const B4800: crate::speed_t = 0xc;
pub const B50: crate::speed_t = 0x1;
pub const B500000: crate::speed_t = 0x1005;
pub const B57600: crate::speed_t = 0x1001;
pub const B576000: crate::speed_t = 0x1006;
pub const B600: crate::speed_t = 0x8;
pub const B75: crate::speed_t = 0x2;
pub const B921600: crate::speed_t = 0x1007;
pub const B9600: crate::speed_t = 0xd;
pub const BS1: c_int = 0x2000;
pub const BSDLY: c_int = 0x2000;
pub const CBAUD: crate::tcflag_t = 0x100f;
pub const CBAUDEX: crate::tcflag_t = 0x1000;
pub const CIBAUD: crate::tcflag_t = 0x100f0000;
pub const CLOCAL: crate::tcflag_t = 0x800;
pub const CPU_SETSIZE: c_int = 0x400;
pub const CR1: c_int = 0x200;
pub const CR2: c_int = 0x400;
pub const CR3: c_int = 0x600;
pub const CRDLY: c_int = 0x600;
pub const CREAD: crate::tcflag_t = 0x80;
pub const CS6: crate::tcflag_t = 0x10;
pub const CS7: crate::tcflag_t = 0x20;
pub const CS8: crate::tcflag_t = 0x30;
pub const CSIZE: crate::tcflag_t = 0x30;
pub const CSTOPB: crate::tcflag_t = 0x40;
pub const EADV: c_int = 0x44;
pub const EAFNOSUPPORT: c_int = 0x61;
pub const EALREADY: c_int = 0x72;
pub const EBADE: c_int = 0x34;
pub const EBADFD: c_int = 0x4d;
pub const EBADMSG: c_int = 0x4a;
pub const EBADR: c_int = 0x35;
pub const EBADRQC: c_int = 0x38;
pub const EBADSLT: c_int = 0x39;
pub const EBFONT: c_int = 0x3b;
pub const ECANCELED: c_int = 0x7d;
pub const ECHOCTL: crate::tcflag_t = 0x200;
pub const ECHOE: crate::tcflag_t = 0x10;
pub const ECHOK: crate::tcflag_t = 0x20;
pub const ECHOKE: crate::tcflag_t = 0x800;
pub const ECHONL: crate::tcflag_t = 0x40;
pub const ECHOPRT: crate::tcflag_t = 0x400;
pub const ECHRNG: c_int = 0x2c;
pub const ECOMM: c_int = 0x46;
pub const EDESTADDRREQ: c_int = 0x59;
pub const EDOTDOT: c_int = 0x49;
pub const EFD_CLOEXEC: c_int = 0x80000;
pub const EFD_NONBLOCK: c_int = 0x800;
pub const EHOSTDOWN: c_int = 0x70;
pub const EHWPOISON: c_int = 0x85;
pub const EIDRM: c_int = 0x2b;
pub const EILSEQ: c_int = 0x54;
pub const EINPROGRESS: c_int = 0x73;
pub const EISCONN: c_int = 0x6a;
pub const EISNAM: c_int = 0x78;
pub const EKEYEXPIRED: c_int = 0x7f;
pub const EKEYREJECTED: c_int = 0x81;
pub const EKEYREVOKED: c_int = 0x80;
pub const EL2HLT: c_int = 0x33;
pub const EL2NSYNC: c_int = 0x2d;
pub const EL3HLT: c_int = 0x2e;
pub const EL3RST: c_int = 0x2f;
pub const ELIBACC: c_int = 0x4f;
pub const ELIBBAD: c_int = 0x50;
pub const ELIBEXEC: c_int = 0x53;
pub const ELIBMAX: c_int = 0x52;
pub const ELIBSCN: c_int = 0x51;
pub const ELNRNG: c_int = 0x30;
pub const EMEDIUMTYPE: c_int = 0x7c;
pub const EMSGSIZE: c_int = 0x5a;
pub const EMULTIHOP: c_int = 0x48;
pub const ENAVAIL: c_int = 0x77;
pub const ENETRESET: c_int = 0x66;
pub const ENOANO: c_int = 0x37;
pub const ENOBUFS: c_int = 0x69;
pub const ENOCSI: c_int = 0x32;
pub const ENOKEY: c_int = 0x7e;
pub const ENOLCK: c_int = 0x25;
pub const ENOLINK: c_int = 0x43;
pub const ENOMEDIUM: c_int = 0x7b;
pub const ENOMSG: c_int = 0x2a;
pub const ENONET: c_int = 0x40;
pub const ENOPKG: c_int = 0x41;
pub const ENOPROTOOPT: c_int = 0x5c;
pub const ENOSR: c_int = 0x3f;
pub const ENOSTR: c_int = 0x3c;
pub const ENOTNAM: c_int = 0x76;
pub const ENOTRECOVERABLE: c_int = 0x83;
pub const ENOTSOCK: c_int = 0x58;
pub const ENOTUNIQ: c_int = 0x4c;
pub const EOVERFLOW: c_int = 0x4b;
pub const EOWNERDEAD: c_int = 0x82;
pub const EPFNOSUPPORT: c_int = 0x60;
pub const EPOLL_CLOEXEC: c_int = 0x80000;
pub const EPROTO: c_int = 0x47;
pub const EPROTONOSUPPORT: c_int = 0x5d;
pub const EPROTOTYPE: c_int = 0x5b;
pub const EREMCHG: c_int = 0x4e;
pub const EREMOTE: c_int = 0x42;
pub const EREMOTEIO: c_int = 0x79;
pub const ERESTART: c_int = 0x55;
pub const ERFKILL: c_int = 0x84;
pub const ESHUTDOWN: c_int = 0x6c;
pub const ESOCKTNOSUPPORT: c_int = 0x5e;
pub const ESRMNT: c_int = 0x45;
pub const ESTRPIPE: c_int = 0x56;
pub const ETIME: c_int = 0x3e;
pub const ETOOMANYREFS: c_int = 0x6d;
pub const EUCLEAN: c_int = 0x75;
pub const EUNATCH: c_int = 0x31;
pub const EUSERS: c_int = 0x57;
pub const EXFULL: c_int = 0x36;
pub const FF1: c_int = 0x8000;
pub const FFDLY: c_int = 0x8000;
pub const FLUSHO: crate::tcflag_t = 0x1000;
pub const F_GETLK: c_int = 0x5;
pub const F_SETLK: c_int = 0x6;
pub const F_SETLKW: c_int = 0x7;
pub const HUPCL: crate::tcflag_t = 0x400;
pub const ICANON: crate::tcflag_t = 0x2;
pub const IEXTEN: crate::tcflag_t = 0x8000;
pub const ISIG: crate::tcflag_t = 0x1;
pub const IXOFF: crate::tcflag_t = 0x1000;
pub const IXON: crate::tcflag_t = 0x400;
pub const MAP_ANON: c_int = 0x20;
pub const MAP_ANONYMOUS: c_int = 0x20;
pub const MAP_DENYWRITE: c_int = 0x800;
pub const MAP_EXECUTABLE: c_int = 0x1000;
pub const MAP_GROWSDOWN: c_int = 0x100;
pub const MAP_LOCKED: c_int = 0x2000;
pub const MAP_NONBLOCK: c_int = 0x10000;
pub const MAP_NORESERVE: c_int = 0x4000;
pub const MAP_POPULATE: c_int = 0x8000;
pub const MAP_STACK: c_int = 0x20000;
pub const NLDLY: crate::tcflag_t = 0x100;
pub const NOFLSH: crate::tcflag_t = 0x80;
pub const OLCUC: crate::tcflag_t = 0x2;
pub const ONLCR: crate::tcflag_t = 0x4;
pub const O_ASYNC: c_int = 0o20000;
pub const O_DIRECT: c_int = 0x10000;
pub const O_DSYNC: c_int = O_SYNC;
pub const O_FSYNC: c_int = O_SYNC;
pub const O_LARGEFILE: c_int = 0o400000;
pub const O_NDELAY: c_int = O_NONBLOCK;
pub const O_NOATIME: c_int = 0o1000000;
pub const O_NOCTTY: c_int = 0x100;
pub const O_PATH: c_int = 0o10000000;
pub const O_RSYNC: c_int = O_SYNC;
pub const O_SYNC: c_int = 0o10000;
pub const PARENB: crate::tcflag_t = 0x100;
pub const PARODD: crate::tcflag_t = 0x200;
pub const PENDIN: crate::tcflag_t = 0x4000;
pub const POLLWRBAND: crate::c_short = 0x200;
pub const POLLWRNORM: crate::c_short = 0x100;
pub const PTHREAD_STACK_MIN: crate::size_t = 16384;
pub const RTLD_GLOBAL: c_int = 0x00100;
// These are typed unsigned to match sigaction
pub const SA_NOCLDSTOP: c_ulong = 0x1;
pub const SA_NOCLDWAIT: c_ulong = 0x2;
pub const SA_SIGINFO: c_ulong = 0x4;
pub const SA_NODEFER: c_ulong = 0x40000000;
pub const SA_ONSTACK: c_ulong = 0x8000000;
pub const SA_RESETHAND: c_ulong = 0x80000000;
pub const SA_RESTART: c_ulong = 0x10000000;
pub const SFD_CLOEXEC: c_int = 0x80000;
pub const SFD_NONBLOCK: c_int = 0x800;
pub const SIGBUS: c_int = 0x7;
pub const SIGCHLD: c_int = 0x11;
pub const SIGCONT: c_int = 0x12;
pub const SIGIO: c_int = 0x1d;
pub const SIGPROF: c_int = 0x1b;
pub const SIGPWR: c_int = 0x1e;
pub const SIGSTKFLT: c_int = 0x10;
pub const SIGSTKSZ: crate::size_t = 8192;
pub const SIGSTOP: c_int = 0x13;
pub const SIGSYS: c_int = 0x1f;
pub const SIGTSTP: c_int = 0x14;
pub const SIGTTIN: c_int = 0x15;
pub const SIGTTOU: c_int = 0x16;
pub const SIGURG: c_int = 0x17;
pub const SIGUSR1: c_int = 0xa;
pub const SIGUSR2: c_int = 0xc;
pub const SIGVTALRM: c_int = 0x1a;
pub const SIGWINCH: c_int = 0x1c;
pub const SIGXCPU: c_int = 0x18;
pub const SIGXFSZ: c_int = 0x19;
pub const SIG_BLOCK: c_int = 0;
pub const SIG_UNBLOCK: c_int = 0x1;
pub const SOCK_NONBLOCK: c_int = 0o0004000;
pub const SOCK_SEQPACKET: c_int = 0x5;
pub const TAB1: c_int = 0x800;
pub const TAB2: c_int = 0x1000;
pub const TAB3: c_int = 0x1800;
pub const TABDLY: c_int = 0x1800;
pub const TCSADRAIN: c_int = 0x1;
pub const TCSAFLUSH: c_int = 0x2;
pub const TCSANOW: c_int = 0;
pub const TOSTOP: crate::tcflag_t = 0x100;
pub const VDISCARD: usize = 0xd;
pub const VEOF: usize = 0x4;
pub const VEOL: usize = 0xb;
pub const VEOL2: usize = 0x10;
pub const VMIN: usize = 0x6;
pub const VREPRINT: usize = 0xc;
pub const VSTART: usize = 0x8;
pub const VSTOP: usize = 0x9;
pub const VSUSP: usize = 0xa;
pub const VSWTC: usize = 0x7;
pub const VT1: c_int = 0x4000;
pub const VTDLY: c_int = 0x4000;
pub const VTIME: usize = 0x5;
pub const VWERASE: usize = 0xe;
pub const XTABS: crate::tcflag_t = 0x1800;
pub const MADV_SOFT_OFFLINE: c_int = 101;

cfg_if! {
    if #[cfg(target_os = "l4re")] {
        mod l4re;
        pub use self::l4re::*;
    } else {
        mod other;
        pub use other::*;
    }
}
