use std::{fs, process};

use clap::Parser;
use console::Term;
use either::Either;
use regex::Regex;

pub mod text;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = text::HELP_GENERAL)]
#[command(arg_required_else_help(true))]
struct Cli {
    /// The path to the file to be executed. Can be relative or absolute
    #[arg(default_value = None)]
    path: Option<String>,

    /// Display help for a particular error
    #[arg(short, long = "error-help")]
    error_help: Option<String>,

    /// Reads the program from the standard input
    #[arg(short = 'i')]
    direct_input: bool,
}

fn find_matching_bracket(start_index: usize, program: &str) -> usize {
    let mut open_brackets = 0;

    let start_char = &program[start_index..start_index+1];

    for i in match start_char {
        "]" => Either::Right((0..start_index+1).rev()),
        _ => Either::Left(start_index..program.len()),
    } {
        match program.chars().nth(i).expect("") {
            '[' => {open_brackets += 1}
            ']' => {open_brackets -= 1}
            _ => {}
        };

        if open_brackets == 0 {
            return i;
        }
    };

    return 0 as usize;
}

fn check_brackets_match(program: &str) -> (bool, usize, &str) {
    let mut open_brackets = 0;
    let mut last_open_bracket = 0;
    let mut last_close_bracket = 0;
    for (i, item) in program.chars().enumerate() {
        if item == '[' {
            last_open_bracket = i;
            open_brackets += 1;
        } else if item == ']' {
            last_close_bracket = i;
            open_brackets -= 1;
        }
    }

    let brackets_match = open_brackets == 0;
    
    let mut mismatched = 0;
    let mut problem_char = "";
    if open_brackets > 0 {
        mismatched = last_open_bracket;
        problem_char = "[";
    } else if open_brackets < 0 {
        mismatched = last_close_bracket;
        problem_char = "]";
    }

    return (brackets_match, mismatched, problem_char);
}

fn find_location(location: usize, program: &str) -> (usize, usize) {
    let line_num = program[0..location].matches("\n").count() + 1;

    let mut last_newline = 0 as usize;
    let mut i = (location as i32) - 1;
    while i >= 0 {
        if &program[i as usize..(i+1) as usize] == "\n" {
            last_newline = i as usize;
            i = 0;
        }
        i -= 1;
    }

    let char_num = program[last_newline..location].chars().count();

    return (line_num, char_num);
}

fn overflow_error(location: usize, problem: &str, overflow_location: &str, program: &str) -> String {
    let readable_location = find_location(location, program);
    eprintln!("{}:{} - Overflow Error - {} at {}", readable_location.0, readable_location.1, problem, overflow_location);
    process::exit(1);
}

fn syntax_error(location: usize, problem: &str, program: &str) -> String {
    let readable_location = find_location(location, program);
    eprintln!("{}:{} - Syntax Error - {}", readable_location.0, readable_location.1, problem);
    process::exit(1);
}

fn parsing_error(location: usize, program: &str) -> String {
    let readable_location = find_location(location, program);
    eprintln!("{}:{} - Parsing Error", readable_location.0, readable_location.1);
    process::exit(1);
}

fn file_error(message: &str) -> String {
    eprintln!("File Handling Error - {}", message);
    process::exit(1);
}

fn iter_error(max_iters: u16) {
    eprintln!("Iteration Error - Exceeded max number of iterations ({})", max_iters);
    process::exit(1);
}

fn read_file(file_path: &str) -> String {
    let mut program = String::from("");

    if let Ok(contents) = fs::read_to_string(file_path) {
        program = contents.to_owned();
    } else {
        file_error("The specified file was not found");
    }

    return program;
}

fn execute(program: String) {
    let re = Regex::new(r"[^+-><\[\],.]").unwrap();

    let check_match = check_brackets_match(&program);
    if !check_match.0 {
        syntax_error(check_match.1, &format!("Unmatched bracket '{}'", check_match.2), &program);
    }

    let mut ptr = 0 as usize;
    let mut arr: [u8; usize::pow(2, 16)] = [0; usize::pow(2, 16)];
    let mut num_iters: u32 = 0;
    let max_iters: u16 = u16::MAX;
    
    let mut i: usize = 0;
    while i < program.len() {
        if num_iters > max_iters as u32 {
            iter_error(max_iters);
        }
        
        let current_char = program.chars().nth(i);
        let mut item = '0';
        if current_char.is_some() {
            item = current_char.expect("Internal error");
        } else {
            parsing_error(i, &program);
        }

        match item {
            item if re.is_match(&item.to_string()) => {}
            '>' => {
                if ptr < u16::MAX as usize {
                    ptr += 1;
                } else {
                    overflow_error(i, "Overflow", "data pointer", &program);
                }
            }
            '<' => {
                if ptr > 0 {
                    ptr -= 1;
                } else {
                    overflow_error(i, "Underflow", "data pointer", &program);
                }
            }
            '+' => {
                if arr[ptr] < u8::MAX {
                    arr[ptr] += 1;
                } else {
                    let overflow_location = String::from("array index ") + &ptr.to_string();
                    overflow_error(i, "Overflow", &overflow_location, &program);
                }
            }
            '-' => {
                if arr[ptr] > 0 {
                    arr[ptr] -= 1;
                } else {
                    let overflow_location = String::from("array index ") + &ptr.to_string();
                    overflow_error(i, "Underflow", &overflow_location, &program);
                }
            }
            '.' => {print!("{}", arr[ptr] as char);}
            ',' => {
                let term = Term::stdout();
                if let Ok(input) = Term::read_char(&term) {
                    let mut b = [0u8; 4];
                    input.encode_utf8(&mut b);
                    arr[ptr] = b[0];
                }
            }
            '[' => {
                if arr[ptr] == 0 {
                    i = find_matching_bracket(i, &program);
                    num_iters = 0;
                }
            }
            ']' => {
                if arr[ptr] != 0 {
                    i = find_matching_bracket(i, &program);
                    num_iters += 1;
                }
            }
            _ => {}
        }

        i += 1;
    }
}

fn main() {
    let cli = Cli::parse();

    if let Some(error_help) = cli.error_help.as_deref() {
        println!("{}", match error_help {
            "overflow" | "underflow" | "overflow error" => text::HELP_OVERFLOW,
            "syntax" | "syntax error" => text::HELP_SYNTAX,
            "file" | "file handling" | "file handling error" => text::HELP_FILE,
            "parsing" | "parsing error" => text::HELP_PARSING,
            "internal" | "internal error" => text::HELP_INTERNAL,
            "iteration" | "iteration error" => text::HELP_ITER,
            _ => "Unknown error type"
        });
    } else if let Some(path) = cli.path.as_deref() {
        let program = read_file(path);
        execute(program);
    } else if cli.direct_input {
        let term = Term::stdout();
        if let Ok(input) = Term::read_line(&term) {
            execute(input);
        }
    }
}
