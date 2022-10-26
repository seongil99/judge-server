use nix::{libc, unistd};

enum SeccompAction {
    Allow,
    Trap,
    Errno(u32),
    Trace(u32),
    Kill,
}

impl SeccompAction {
    fn to_u32(&self) -> u32 {
        match self {
            SeccompAction::Allow => seccomp_sys::SCMP_ACT_ALLOW,
            SeccompAction::Trap => seccomp_sys::SCMP_ACT_TRAP,
            SeccompAction::Errno(errno) => seccomp_sys::SCMP_ACT_ERRNO(*errno),
            SeccompAction::Trace(sig) => seccomp_sys::SCMP_ACT_TRACE(*sig),
            SeccompAction::Kill => seccomp_sys::SCMP_ACT_KILL,
        }
    }
}

pub struct Process {
    uid: u32,
    pid: u32,
    ppid: u32,
    c: u32,
    stime: u32,
    tty: String,
    time: u32,
    cmd: String,
}

impl Process {
    pub fn new(
        uid: u32,
        pid: u32,
        ppid: u32,
        c: u32,
        stime: u32,
        tty: String,
        time: u32,
        cmd: String,
    ) -> Self {
        Process {
            uid,
            pid,
            ppid,
            c,
            stime,
            tty,
            time,
            cmd,
        }
    }
}
