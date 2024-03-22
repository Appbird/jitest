use std::{fs::{self, File}, process::{exit, Command, Output, Stdio}};
use colored::*;
use console::Term;
use clap::Parser;
use regex::{self, Regex};

#[derive(Parser)]
struct CLI {
    // path to directory which includes test target (such as `a.out`).
    target:String,
    #[arg(short = 'B', long)]
    ignore_blank_lines:bool,
    #[arg(short = 'd', long)]
    direct_input:bool
}

type OutputThrowable = Result<Output, std::io::Error>;

fn compile(file:&str, out:&str) -> OutputThrowable{
    Command::new("g++")
        .arg("-Wall")
        .arg("-Wextra")
        .arg("--std=c++20")
        .arg("-o")
            .arg(out)
        .arg(file)
        .output()
}
fn execute(file:&str, input:&str, output:&str) -> OutputThrowable{
    let input_file = File::open(input).unwrap();
    let output_file = File::create(output).unwrap();
    Command::new(file)
        .stdin(Stdio::from(input_file))
        .stdout(Stdio::from(output_file))
        .output()
}
fn execute_direct(file:&str) -> OutputThrowable{
    Command::new(file)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
}
fn diff(expected:&str, actual:&str, ignore_blank_lines:bool) -> OutputThrowable {
    let mut cmd = Command::new("diff");
    cmd.args([actual, expected]);
    if ignore_blank_lines { cmd.arg("-B"); }
    
    cmd.output()
}
fn error_report(header_name:&str, detail:&str) {
    let header_text = format!("[[{}]]", header_name);
    eprintln!("{}", header_text.underline().bold().red());
    eprintln!("{detail}");
}

fn main() {
    env_logger::init();
    let term:Term = Term::stdout();
    let args = CLI::parse();
    let re = Regex::new(r"test(?P<suffix>[0-9]*)").unwrap();
    let mut test_cases = Vec::<String>::new();
    let entries = fs::read_dir(&args.target).unwrap();
    
    for entry in entries {
        let correct_entry   = entry.unwrap();
        let file_name_os    = correct_entry.file_name();
        let file_name           = file_name_os.to_str().unwrap();
        if let Some(captures) = re.captures(file_name) {
            test_cases.push(captures["suffix"].to_string())
        }
    }
    
    let testcase_count = test_cases.len();
    println!("{} test cases exists", test_cases.len());
    let compile_file    = format!("{}/p.cpp", &args.target);
    let exec_file       = format!("{}/p.out", &args.target);
    compile(&compile_file, &exec_file).unwrap_or_else(
        |e| { error_report("Compile Error", &e.to_string()); exit(1); }
    );
    
    let mut accepted_count = 0;
    for (number, case) in test_cases.iter().enumerate() {
        let testcase_number = number + 1;
        let input       = format!("{}/test{}", &args.target, case);
        let output      = format!("{}/out{}", &args.target, case);
        let expected    = format!("{}/exp{}", &args.target, case);

        println!("{} #{}", "[ Waiting Judge ... ]".on_white().black().bold(), testcase_number);
        {
            execute(&exec_file, &input, &output).unwrap_or_else(
                |e| { error_report("Runtime Error", &e.to_string()); exit(1); }
            );
        }
        
        let diff_result = diff(&expected, &output, args.ignore_blank_lines).unwrap_or_else(
            |e| { error_report("Failed to compare", &e.to_string()); exit(1); }
        );
        term.clear_last_lines(1).unwrap();
        if diff_result.status.success() {
            println!("{} #{}", "[Accepted]".on_green().white().bold(), testcase_number);
            accepted_count += 1;
        } else {
            println!("{} #{}", "[Wrong Answer]".on_yellow().white().bold(), testcase_number);
            let input_txt       = fs::read_to_string(&input).unwrap();
            let output_txt      = fs::read_to_string(&output).unwrap_or_default();
            let expected_txt    = fs::read_to_string(&expected).unwrap();
            println!("{} {}\n{}", "[in]".bold(),  &input, &input_txt);
            println!("{} {}\n{}", "[out]".bold(), &output, &output_txt);
            println!("{} {}\n{}", "[exp]".bold(), &expected, &expected_txt);
        }
    }
    if accepted_count == testcase_count {
        println!("\n{}", "[[✅ All accepted!]]".green().bold().underline());
    }
    
    
    
}
