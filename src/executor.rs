use judge::Status;
use serde::{Deserialize, Serialize};
use std::{
    cmp,
    ffi::c_char,
    fs::File,
    io::{BufReader, Read, Write},
};

use crate::filter;
use crate::judge;

#[derive(Serialize, Deserialize)]
pub struct TestCase {
    pub input: String,
    pub output: String,
}

#[derive(Deserialize, Serialize)]
pub struct Problem {
    answer_id: u64,
    language: String,
    code: String,
    testcases: Vec<TestCase>,
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

    pub fn write_testcase_file(&self) {
        for i in 0..self.testcases.len() {
            let mut file = File::create(format!("test_cases/input/input{}.txt", i)).unwrap();
            file.write_all(self.testcases[i].input.as_bytes()).unwrap();

            let mut file = File::create(format!("test_cases/output/output{}.txt", i)).unwrap();
            file.write_all(self.testcases[i].output.as_bytes()).unwrap();
        }
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

    let mut result_time_file = File::create("result/time.txt").unwrap();
    result_time_file.write_all("0".as_bytes()).unwrap();

    let mut result_memory_file = File::create("result/memory.txt").unwrap();
    result_memory_file.write_all("0".as_bytes()).unwrap();

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
            String::from("test_cases/1/result/result") + &i.to_string() + ".txt" + "\0";

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
            // set seccomp
            // Load the seccomp filter
            let mut filter = filter::SyscallFilter::new();
            filter.add_syscall(libc::SYS_chroot);
            filter.add_syscall(libc::SYS_chdir);
            filter.add_syscall(libc::SYS_fchdir);
            filter.add_syscall(libc::SYS_fchown);
            filter.load();

            // release seccomp filter
            filter.release();

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
                unsafe {
                    libc::getrusage(libc::RUSAGE_CHILDREN, &mut ruse);
                }

                let mut result_time_file = File::open("result/time.txt").unwrap();
                let mut result_time = String::new();
                result_time_file.read_to_string(&mut result_time).unwrap();
                let result_time: i64 = result_time.parse().unwrap();
                let time = ruse.ru_utime.tv_sec * 1000 + ruse.ru_utime.tv_usec / 1000;
                match result_time.cmp(&time) {
                    cmp::Ordering::Less => {
                        let mut result_time_file = File::create("result/time.txt").unwrap();
                        result_time_file
                            .write_all(time.to_string().as_bytes())
                            .unwrap();
                    }
                    _ => {}
                }

                let mut result_memory_file = File::open("result/memory.txt").unwrap();
                let mut result_memory = String::new();
                result_memory_file
                    .read_to_string(&mut result_memory)
                    .unwrap();
                let result_memory: i64 = result_memory.parse().unwrap();
                match result_memory.cmp(&ruse.ru_maxrss) {
                    cmp::Ordering::Less => {
                        let mut result_memory_file = File::create("result/memory.txt").unwrap();
                        result_memory_file
                            .write_all(ruse.ru_maxrss.to_string().as_bytes())
                            .unwrap();
                    }
                    _ => {}
                }
            } else {
                panic!("fork failed");
            }
            unsafe { libc::exit(0) };
        } else if pid > 0 {
            // Parent process
            // wait for child process
            let mut status: libc::c_int = 0;
            unsafe { libc::wait(&mut status) };
        } else {
            panic!("Fork failed");
        }
    }
}
