use seccompiler::{BpfProgram, SeccompAction, SeccompFilter};
use std::{convert::TryInto, ffi::c_char};

//todo get accurate memory size of a.out

fn main() {
    //compile main.c to a.out with gcc
    unsafe {
        libc::system("gcc main.c\0".as_ptr() as *const c_char);
    }

    let rlim_mem = libc::rlimit {
        rlim_cur: 250000000,
        rlim_max: libc::RLIM_INFINITY,
    };
    let rlim_cpu = libc::rlimit {
        rlim_cur: 3,
        rlim_max: libc::RLIM_INFINITY,
    };

    unsafe {
        libc::setrlimit(libc::RLIMIT_AS, &rlim_mem);
        libc::setrlimit(libc::RLIMIT_CPU, &rlim_cpu);
    }

    let mut ruse: libc::rusage = unsafe { std::mem::zeroed() };

    let pid = unsafe { libc::fork() };
    if pid == 0 {
        // Child process

        // Load the seccomp filter
        let filter: BpfProgram = SeccompFilter::new(
            vec![(libc::SYS_open, vec![])].into_iter().collect(),
            SeccompAction::Allow,
            SeccompAction::Trap,
            std::env::consts::ARCH.try_into().unwrap(),
        )
        .unwrap()
        .try_into()
        .unwrap();
        seccompiler::apply_filter(&filter).unwrap();

        unsafe {
            //run a.out
            libc::execve(
                "/usr/bin/time\0".as_ptr() as *const c_char,
                [
                    "/usr/bin/time\0".as_ptr() as *const c_char,
                    "./a.out\0".as_ptr() as *const c_char,
                    std::ptr::null(),
                ]
                .as_ptr(),
                std::ptr::null(),
            );
        }
    } else if pid > 0 {
        // Parent process
        let mut status: libc::c_int = 0;
        unsafe { libc::wait(&mut status) };

        unsafe { libc::getrusage(libc::RUSAGE_CHILDREN, &mut ruse) };

        println!("memory usage: {} kb", ruse.ru_maxrss);
        println!("time usage: {} ms", ruse.ru_utime.tv_usec / 1000);

        println!("Child exited with status {}", status);
    } else {
        panic!("Fork failed");
    }
}
