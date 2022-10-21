use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::Command;
use std::str;

fn main() {
    // test code
    if cfg!(target_os = "windows") {
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

    if cfg!(target_os = "windows") {
        let input_file = File::open("test_code/input.txt").unwrap();
        let mut buf_reader = BufReader::new(input_file);
        let mut input_text = String::new();
        buf_reader.read_to_string(&mut input_text).unwrap();
        println!("input_text:  {}", input_text);

        Command::new("cmd") // windows env
            .args(["/C", "gcc -o test.exe test_code/test.c"])
            .output()
            .expect("failed to execute process");

        let mut child = Command::new("cmd") // windows env
            .args(["/C", "test.exe"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        {
            let stdin = child.stdin.as_mut().expect("failed to open stdin");
            stdin
                .write_all(input_text.as_bytes())
                .expect("failed to write to stdin");
        }

        let output = child.wait_with_output().expect("failed to wait on child");

        let output_text = match str::from_utf8(&output.stdout) {
            Ok(v) => v.trim(),
            Err(_) => panic!("failed to convert output to string: {:?}", output.stdout),
        };
        println!("output_text: {}", output_text);

        let answer_file = File::open("test_code/answer.txt").unwrap();
        let mut buf_reader = BufReader::new(answer_file);
        let mut answer_text = String::new();
        buf_reader.read_to_string(&mut answer_text).unwrap();

        assert_eq!(output_text, answer_text.trim());

        println!("Success!");
    } else {
        let input_file = File::open("test_code/input.txt").unwrap();
        let mut buf_reader = BufReader::new(input_file);
        let mut input_text = String::new();
        buf_reader.read_to_string(&mut input_text).unwrap();
        println!("input_text:  {}", input_text);

        Command::new("sh") // linux env
            .arg("-c")
            .arg("gcc -o test.exe test_code/test.c")
            .output()
            .expect("failed to execute process");

        let mut child = Command::new("sh")
            .arg("-c") // linux env
            .arg("test.exe")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        {
            let stdin = child.stdin.as_mut().expect("failed to open stdin");
            stdin
                .write_all(input_text.as_bytes())
                .expect("failed to write to stdin");
        }

        let output = child.wait_with_output().expect("failed to wait on child");

        let output_text = match str::from_utf8(&output.stdout) {
            Ok(v) => v.trim(),
            Err(_) => panic!("failed to convert output to string: {:?}", output.stdout),
        };
        println!("output_text: {}", output_text);

        let answer_file = File::open("test_code/answer.txt").unwrap();
        let mut buf_reader = BufReader::new(answer_file);
        let mut answer_text = String::new();
        buf_reader.read_to_string(&mut answer_text).unwrap();

        assert_eq!(output_text, answer_text.trim());

        println!("Success!");
    }
}
