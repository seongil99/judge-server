use std::fs::File;
use std::io::{BufReader, Read};
use std::process::Command;

use serde::{Deserialize, Serialize};

use tracing::info;

use crate::executor::Problem;

#[derive(Serialize, Deserialize)]
pub struct JudgeResult {
    result: String,
    time: u64,
    memory: u64,
    answer_id: u64,
}

impl JudgeResult {
    pub fn from_result_files(status: Status, answer_id: u64) -> Self {
        let mut result_file_memory = File::open("result/memory.txt").unwrap();
        let mut result_file_time = File::open("result/time.txt").unwrap();

        let mut memory = String::new();
        let mut time = String::new();

        result_file_memory.read_to_string(&mut memory).unwrap();
        result_file_time.read_to_string(&mut time).unwrap();

        let memory: u64 = memory.parse().unwrap();
        let time: u64 = time.parse().unwrap();

        let result_string = status.to_string();

        Self {
            answer_id,
            time,
            memory,
            result: result_string,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Status {
    Accepted,
    Proceeding,
    WrongAnswer,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    CompileError,
    RuntimeError,
    SystemError,
}

#[allow(dead_code)]
impl Status {
    pub fn to_string(&self) -> String {
        match self {
            Status::Accepted => "Accepted".to_string(),
            Status::Proceeding => "Proceeding".to_string(),
            Status::WrongAnswer => "WrongAnswer".to_string(),
            Status::TimeLimitExceeded => "TimeLimitExceeded".to_string(),
            Status::MemoryLimitExceeded => "MemoryLimitExceeded".to_string(),
            Status::CompileError => "CompileError".to_string(),
            Status::RuntimeError => "RuntimeError".to_string(),
            Status::SystemError => "SystemError".to_string(),
            _ => "".to_string(),
        }
    }
}

pub fn clean() {
    Command::new("rm")
        .arg("a.out")
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on rm a.out");

    Command::new("sh")
        .arg("-c")
        .arg("rm ./test_cases/*/*.txt")
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on rm test_cases/output/*");

    Command::new("rm")
        .arg("main.c")
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on rm main.c");
}

pub fn main(problem: &Problem) -> Result<Status, Box<dyn std::error::Error>> {
    let input_files_path = "test_cases/input";
    let input_files = std::fs::read_dir(input_files_path).unwrap();
    let input_files_txt: Vec<_> = input_files
        .filter_map(|e| e.ok())
        .filter(|e| match e.path().extension() {
            Some(ext) => ext == "txt",
            None => false,
        })
        .collect();
    let input_len = input_files_txt.len();

    #[cfg(debug_assertions)]
    info!(?input_len, "input_len");

    let mut judge_status = Status::Accepted;

    for i in 0..input_len {
        let output_path = String::from("test_cases/output/output") + &i.to_string() + ".txt";
        let answer_path = String::from("test_cases/result/result") + &i.to_string() + ".txt";

        let output_file = File::open(output_path).unwrap();
        let mut buf_reader = BufReader::new(output_file);
        let mut output_text = String::new();
        buf_reader.read_to_string(&mut output_text).unwrap();

        let answer_file = File::open(answer_path).unwrap();
        let mut buf_reader = BufReader::new(answer_file);
        let mut answer_text = String::new();
        buf_reader.read_to_string(&mut answer_text).unwrap();

        match output_text.trim_end() == answer_text.trim_end() {
            true => {}
            false => {
                judge_status = Status::WrongAnswer;
                break;
            }
        }
    }

    let mut memory_file = File::open("result/memory.txt").unwrap();
    let mut time_file = File::open("result/time.txt").unwrap();

    let mut memory_usage = String::new();
    let mut time_usage = String::new();

    let mut buf_reader = BufReader::new(memory_file);
    buf_reader.read_to_string(&mut memory_usage).unwrap();

    let mut buf_reader = BufReader::new(time_file);
    buf_reader.read_to_string(&mut time_usage).unwrap();

    let memory_usage: u64 = memory_usage.parse().unwrap();
    let time_usage: u64 = time_usage.parse().unwrap();

    if memory_usage > problem.memory_limit * 1000 {
        judge_status = Status::MemoryLimitExceeded
    };

    if time_usage > problem.time_limit * 1000 {
        judge_status = Status::TimeLimitExceeded
    };

    Ok(judge_status)
}
