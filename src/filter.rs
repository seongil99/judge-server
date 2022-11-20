use libc::c_long;
use seccompiler::{BpfProgram, SeccompAction, SeccompFilter, SeccompRule};

pub struct SyscallFilter {
    syscalls: Vec<c_long>,
}

#[allow(dead_code)]
impl SyscallFilter {
    pub fn new() -> Self {
        Self {
            syscalls: Vec::new(),
        }
    }

    pub fn add_syscall(&mut self, syscall: c_long) {
        self.syscalls.push(syscall);
    }

    pub fn add_syscalls(&mut self, syscalls: Vec<c_long>) {
        self.syscalls.extend(syscalls);
    }

    pub fn load(&self) {
        let mut list: Vec<(i64, Vec<SeccompRule>)> = Vec::new();
        for syscall in &self.syscalls {
            list.push((*syscall, vec![]));
        }

        let filter: BpfProgram = SeccompFilter::new(
            list.into_iter().collect(),
            SeccompAction::Allow,
            SeccompAction::Trap,
            std::env::consts::ARCH.try_into().unwrap(),
        )
        .unwrap()
        .try_into()
        .unwrap();
        seccompiler::apply_filter(&filter).unwrap();
    }

    pub fn release(&self) {
        unsafe {
            libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);
        }
    }
}
