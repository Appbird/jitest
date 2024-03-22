mod cmd;

use std::{fs, process::exit};
use colored::*;
use console::Term;
use clap::Parser;
use regex::{self, Regex};
use std::time::Instant;

use cmd::*;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct CLI {
    /// path to directory which includes test target (such as `a.out`).
    target:String,
    
    /// whether this program add -B option when executing diff
    #[arg(short = 'B', long)]
    ignore_blank_lines:bool,

    /// whether you give input manually
    #[arg(short = 'd', long)]
    direct_input:bool
}

fn error_report(header_name:&str, detail:&str) {
    let header_text = format!("[[{}]]", header_name);
    eprintln!("{}", header_text.underline().bold().red());
    eprintln!("{detail}");
}

fn enumrate_test_cases(target_directory:&str) -> Vec<String> {
    let mut test_cases = Vec::<String>::new();
    let re = Regex::new(r"test(?P<suffix>[0-9]*)").unwrap();
    let entries = fs::read_dir(target_directory).unwrap();
    
    for entry in entries {
        let correct_entry   = entry.unwrap();
        let file_name_os    = correct_entry.file_name();
        let file_name           = file_name_os.to_str().unwrap();
        if let Some(captures) = re.captures(file_name) {
            test_cases.push(captures["suffix"].to_string())
        }
    }
    return test_cases
}

fn display_cases(input_path:&str, output_path:&str, exp_path:&str, err_path:&str) {
    let input_txt       = fs::read_to_string(&input_path).unwrap();
    let output_txt      = fs::read_to_string(&output_path).unwrap_or_default();
    let expected_txt    = fs::read_to_string(&exp_path).unwrap();
    
    println!("{} {}\n{}", "[in]".underline().bold(),  &input_path, &input_txt);
    println!("{} {}\n{}", "[out]".underline().bold(), &output_path, &output_txt);
    println!("{} {}\n{}", "[exp]".underline().bold(), &exp_path, &expected_txt);
    println!("{} -> {}\n\n", "[err]".underline().bold(), &err_path);
}

fn test_with_cases(
    term:&Term,
    test_cases:&Vec<String>,
    args:&CLI,
    exec_file:&str
) {
    let testcase_count = test_cases.len();
    let mut accepted_count = 0;
    for (number, case) in test_cases.iter().enumerate() {
        let testcase_number = number + 1;
        let input       = format!("{}/test{case}", &args.target);
        let output      = format!("{}/out{case}", &args.target);
        let expected    = format!("{}/exp{case}", &args.target);
        let errput      = format!("{}/err{case}", &args.target);

        println!("{} #{}", "[ WJ ]".on_white().black().bold(), testcase_number);
        let start_time = Instant::now();
        {
            execute(&exec_file, &input, &output, &errput).unwrap_or_else(
                |e| { error_report("Runtime Error", &e.to_string()); exit(1); }
            );
        }
        let duration = start_time.elapsed();
        
        let diff_result = diff(&expected, &output, args.ignore_blank_lines).unwrap_or_else(
            |e| { error_report("Failed to compare", &e.to_string()); exit(1); }
        );
        term.clear_last_lines(1).unwrap();
        if diff_result.status.success() {
            println!("{} #{testcase_number} -- {:.3} sec", "[ AC ]".on_green().white().bold(), duration.as_secs_f64());
            accepted_count += 1;
        } else {
            println!("{} #{testcase_number} -- {:.3} sec", "[ WA ]".on_yellow().white().bold(), duration.as_secs_f64());
            display_cases(&input, &output, &expected, &errput);
        }
    }
    if accepted_count == testcase_count {
        println!("\n{}", "[[✅ All Accepted!]]".green().bold().underline());
    }
}

fn main() {
    env_logger::init();
    let term:Term = Term::stdout();
    let args = CLI::parse();
    
    let test_cases = enumrate_test_cases(&args.target);
    println!("{} test cases exist", test_cases.len());

    let compile_file    = format!("{}/p.cpp", &args.target);
    let exec_file       = format!("{}/p.out", &args.target);
    compile(&compile_file, &exec_file).unwrap_or_else(
        |e| { error_report("Compile Error", &e.to_string()); exit(1); }
    );
    println!("\n{}\n", "[[✅ Compile Completed]]".green().bold().underline());
    if args.direct_input {
        execute_direct(&exec_file).unwrap();
    } else {
        test_with_cases(&term, &test_cases, &args, &exec_file);
    }
    
}
