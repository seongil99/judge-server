use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::Command;

fn main() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
    };

    let input_file = File::open("test_code/input.txt").unwrap();
    let mut buf_reader = BufReader::new(input_file);
    let mut input_text = String::new();
    buf_reader.read_to_string(&mut input_text).unwrap();
    println!("input_text: {}", input_text);

    Command::new("cmd") // windows env
        .args(["/C", "gcc -o test.exe test_code/test.c"])
        .output()
        .expect("failed to execute process");

    let output = Command::new("cmd")
        .args(["/C", "test.exe"])
        .arg(input_text)
        .output()
        .expect("failed to execute process");

    // let output = String::from_utf8_lossy(&output.stdout);
    println!("output: {:?}", output.stdout);
}
