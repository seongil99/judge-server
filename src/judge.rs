use std::{
    convert::TryInto,
    ffi::c_char,
    fs::File,
    io::{BufReader, Read, Write},
};

use seccompiler::{BpfProgram, SeccompAction, SeccompFilter};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Problem {
    language: String,
    code: String,
}

impl Problem {
    pub fn from_payload(paylod: &Vec<u8>) -> Problem {
        let data: Problem = serde_json::from_slice(&paylod).unwrap();
        data
    }

    pub fn write_code_file(&self) {
        let mut file = File::create("main.c").unwrap();
        file.write_all(self.code.as_bytes()).unwrap();
    }
}

pub struct Limits {
    time: u64,
    memory: u64,
}

impl Limits {
    pub fn new(time: u64, memory: u64) -> Self {
        Self { time, memory }
    }

    pub fn set_limits(&self) {
        let time = self.time;
        let memory = self.memory;
        unsafe {
            libc::setrlimit(
                libc::RLIMIT_CPU,
                &libc::rlimit {
                    rlim_cur: time,
                    rlim_max: time,
                },
            );
            libc::setrlimit(
                libc::RLIMIT_AS,
                &libc::rlimit {
                    rlim_cur: memory,
                    rlim_max: memory,
                },
            );
        }
    }
}

pub fn main() {
    //compile main.c to a.out with gcc
    unsafe {
        libc::system("gcc main.c\0".as_ptr() as *const c_char);
    }

    let input_files_path = "test_cases/1/input";
    let input_files = std::fs::read_dir(input_files_path).unwrap();
    let input_len = input_files.count();

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

    println!("input_len: {}", input_len);

    for i in 0..input_len {
        println!("test case : {}", i);

        let input_path = String::from("test_cases/1/input/input") + &i.to_string() + ".txt" + "\0";

        let output_path =
            String::from("test_cases/1/output/output") + &i.to_string() + ".txt" + "\0";

        // open input file and output file
        let fd_in = unsafe { libc::open(input_path.as_ptr() as *const c_char, libc::O_RDONLY) };
        let fd_out = unsafe {
            libc::open(
                output_path.as_ptr() as *const c_char,
                libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC,
                libc::S_IRUSR | libc::S_IWUSR | libc::S_IRGRP | libc::S_IWGRP | libc::S_IROTH, // 0644
            )
        };

        let pid = unsafe { libc::fork() };
        if pid == 0 {
            // Child process
            println!("child process {}", i);

            // set seccomp
            // Load the seccomp filter
            let filter: BpfProgram = SeccompFilter::new(
                vec![(libc::SYS_chroot, vec![])].into_iter().collect(),
                SeccompAction::Allow,
                SeccompAction::Trap,
                std::env::consts::ARCH.try_into().unwrap(),
            )
            .unwrap()
            .try_into()
            .unwrap();
            seccompiler::apply_filter(&filter).unwrap();

            // release seccomp filter
            unsafe {
                libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);
            }

            let pid_c = unsafe { libc::fork() };
            if pid_c == 0 {
                unsafe {
                    println!("child process {} child", i);
                    // link stdin and stdout
                    libc::dup2(fd_in, libc::STDIN_FILENO);
                    libc::dup2(fd_out, libc::STDOUT_FILENO);

                    //run a.out
                    libc::execve(
                        "a.out\0".as_ptr() as *const c_char,
                        std::ptr::null(),
                        std::ptr::null(),
                    );
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
            } else {
                panic!("fork failed");
            }
            unsafe { libc::exit(0) };
        } else if pid > 0 {
            // Parent process
            // wait for child process
            let mut status: libc::c_int = 0;
            unsafe { libc::wait(&mut status) };

            println!("Child exited with status {}", status);

            // compare output1 to answer1
            let output_path = String::from("test_cases/1/output/output") + &i.to_string() + ".txt";
            let answer_path = String::from("test_cases/1/answer/answer") + &i.to_string() + ".txt";
            let output_file = File::open(output_path).unwrap();
            let mut buf_reader = BufReader::new(output_file);
            let mut output_text = String::new();
            buf_reader.read_to_string(&mut output_text).unwrap();

            let answer_file = File::open(answer_path).unwrap();
            let mut buf_reader = BufReader::new(answer_file);
            let mut answer_text = String::new();
            buf_reader.read_to_string(&mut answer_text).unwrap();

            assert_eq!(output_text.trim_end(), answer_text.trim_end());
            println!("output{} is correct", i);
        } else {
            panic!("Fork failed");
        }
    }
}
