pub enum SeccompAction {
    Allow,
    Trap,
    Errno(u32),
    Trace(u32),
    Kill,
}

impl SeccompAction {
    pub fn to_u32(&self) -> u32 {
        match self {
            SeccompAction::Allow => seccomp_sys::SCMP_ACT_ALLOW,
            SeccompAction::Trap => seccomp_sys::SCMP_ACT_TRAP,
            SeccompAction::Errno(errno) => seccomp_sys::SCMP_ACT_ERRNO(*errno),
            SeccompAction::Trace(sig) => seccomp_sys::SCMP_ACT_TRACE(*sig),
            SeccompAction::Kill => seccomp_sys::SCMP_ACT_KILL,
        }
    }
}

pub struct SeccompRule {
    syscall: i32,
    action: SeccompAction,
}

pub struct SeccompFilter {
    ctx: *mut seccomp_sys::scmp_filter_ctx,
    rules: Vec<SeccompRule>,
    default_action: SeccompAction,
}

impl SeccompFilter {
    pub fn new(default_action: SeccompAction) -> Self {
        let ctx = unsafe { seccomp_sys::seccomp_init(default_action.to_u32()) };
        if ctx.is_null() {
            panic!("Failed to initialize seccomp context");
        }
        Self {
            ctx,
            rules: Vec::new(),
            default_action,
        }
    }

    pub fn add_rule(&mut self, syscall: i32, action: SeccompAction) {
        self.rules.push(SeccompRule { syscall, action });
    }

    pub fn load(&mut self) {
        for rule in self.rules.iter() {
            let ret = unsafe {
                seccomp_sys::seccomp_rule_add(
                    self.ctx,
                    seccomp_sys::SCMP_ACT_ALLOW,
                    rule.syscall,
                    0,
                )
            };
            if ret != 0 {
                panic!("Failed to add seccomp rule for syscall {}", rule.syscall);
            }
        }
        let ret = unsafe { seccomp_sys::seccomp_load(self.ctx) };
        if ret != 0 {
            panic!("Failed to load seccomp rules");
        }
    }
}
