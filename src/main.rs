use seccompiler::{BpfProgram, SeccompAction, SeccompFilter};
use std::{
    convert::TryInto,
    ffi::c_char,
    fs::File,
    io::{BufReader, Read},
};

// This program does not work on aarch64.
// because the syscall number is different.

fn main() {
    //compile main.c to a.out with gcc
    unsafe {
        libc::system("gcc main.c\0".as_ptr() as *const c_char);
    }

    // set rlimit
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

    // init rusage
    let mut ruse: libc::rusage = unsafe { std::mem::zeroed() };

    // open input file and output file
    let fd_in = unsafe { libc::open("input1\0".as_ptr() as *const c_char, libc::O_RDONLY) };
    let fd_out = unsafe {
        libc::open(
            "output1\0".as_ptr() as *const c_char,
            libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC,
            libc::S_IRUSR | libc::S_IWUSR | libc::S_IRGRP | libc::S_IWGRP | libc::S_IROTH, // 0644
        )
    };

    let pid = unsafe { libc::fork() };
    if pid == 0 {
        // Child process

        // set seccomp
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

        let pid_c = unsafe { libc::fork() };
        if pid_c == 0 {
            unsafe {
                // link stdin and stdout
                libc::dup2(fd_in, libc::STDIN_FILENO);
                libc::dup2(fd_out, libc::STDOUT_FILENO);

                //run a.out
                libc::execve(
                    "a.out\0".as_ptr() as *const c_char,
                    std::ptr::null(),
                    std::ptr::null(),
                );

                // get time using /usr/bin/time
                // libc::execve(
                //     "/usr/bin/time\0".as_ptr() as *const c_char,
                //     [
                //         "/usr/bin/time\0".as_ptr() as *const c_char,
                //         "./a.out\0".as_ptr() as *const c_char,
                //         std::ptr::null(),
                //     ]
                //     .as_ptr(),
                //     std::ptr::null(),
                // );
            }
        } else if pid_c > 0 {
            // wait for child process
            let mut status: libc::c_int = 0;
            unsafe { libc::wait(&mut status) };

            // close file descriptors
            unsafe {
                libc::close(fd_in);
                libc::close(fd_out);
            }

            println!("main.c exited with status {}", status);

            // get resource usage from child process
            unsafe { libc::getrusage(libc::RUSAGE_CHILDREN, &mut ruse) };

            println!("memory usage: {} kb", ruse.ru_maxrss);
            println!(
                "time usage: {}{} ms",
                ruse.ru_utime.tv_sec,
                ruse.ru_utime.tv_usec / 1000
            );

            // release seccomp filter
            unsafe {
                libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);
            }
        } else {
            panic!("fork failed");
        }
    } else if pid > 0 {
        // Parent process
        // wait for child process
        let mut status: libc::c_int = 0;
        unsafe { libc::wait(&mut status) };

        println!("Child exited with status {}", status);

        // compare output1 to answer1
        let output_file = File::open("output1").unwrap();
        let mut buf_reader = BufReader::new(output_file);
        let mut output_text = String::new();
        buf_reader.read_to_string(&mut output_text).unwrap();

        let answer_file = File::open("answer1").unwrap();
        let mut buf_reader = BufReader::new(answer_file);
        let mut answer_text = String::new();
        buf_reader.read_to_string(&mut answer_text).unwrap();

        if output_text == answer_text {
            println!("Correct answer");
        } else {
            println!("Wrong answer");
        }
    } else {
        panic!("Fork failed");
    }
}
