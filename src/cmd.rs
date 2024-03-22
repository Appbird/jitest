use std::{fs::File, process::{Command, Output, Stdio}};


type OutputThrowable = Result<Output, std::io::Error>;

pub fn compile(file:&str, out:&str) -> OutputThrowable{
    Command::new("g++")
        .arg("-Wall")
        .arg("-Wextra")
        .arg("--std=c++20")
        .arg("-o")
            .arg(out)
        .arg(file)
        .output()
}
pub fn execute(file:&str, input:&str, output:&str, errput:&str) -> OutputThrowable{
    let input_file = File::open(input).unwrap();
    let output_file = File::create(output).unwrap();
    let errput_file = File::create(errput).unwrap();
    Command::new(file)
        .stdin(Stdio::from(input_file))
        .stdout(Stdio::from(output_file))
        .stderr(Stdio::from(errput_file))
        .output()
}
pub fn execute_direct(file:&str) -> OutputThrowable{
    Command::new(file)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
}
pub fn diff(expected:&str, actual:&str, ignore_blank_lines:bool) -> OutputThrowable {
    let mut cmd = Command::new("diff");
    cmd.args([actual, expected]);
    if ignore_blank_lines { cmd.arg("-B").arg("-b"); }
    
    cmd.output()
}