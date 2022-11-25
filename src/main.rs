use std::{fs, fmt, error::Error, env};

use clap::Parser;
use console::Term;
use either::Either;
use regex::Regex;

pub mod text;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = text::HELP_GENERAL, arg_required_else_help(true))]
struct Cli {
    /// The path to the file to be executed. Can be relative or absolute
    #[arg(default_value = None)]
    path: Option<String>,

    /// Display help for a particular error
    #[arg(long = "error-help")]
    error_help: Option<String>,

    /// Start a REPL session
    #[arg(short, long)]
    repl: bool,

    /// Start in unsafe mode
    #[arg(short = 'u', long = "unsafe")]
    is_unsafe: bool,

    /// Use extended syntax
    #[arg(short, long)]
    extended: bool,
}

#[derive(Debug)]
struct ExecutionError {
    details: String,
    code: u8,
} 

impl ExecutionError {
    fn new_overflow(location: usize, problem: &str, overflow_location: &str, program: &str) -> ExecutionError {
        let readable_location = find_location(location, program);
        let details = format!("{}:{} - Overflow Error - {} at {}", readable_location.0, readable_location.1, problem, overflow_location);
        ExecutionError{details, code: 75}
    }

    fn new_syntax(location: usize, problem: &str, program: &str) -> ExecutionError {
        let readable_location = find_location(location, program);
        let details = format!("{}:{} - Syntax Error - {}", readable_location.0, readable_location.1, problem);
        ExecutionError{details, code: 1}
    }

    fn new_parsing(location: usize, program: &str) -> ExecutionError {
        let readable_location = find_location(location, program);
        let details = format!("{}:{} - Parsing Error", readable_location.0, readable_location.1);
        ExecutionError{details, code: 1}
    }

    fn new_file(message: &str) -> ExecutionError {
        let details = format!("File Handling Error - {}", message);
        ExecutionError{details, code: 2}
    }

    fn new_iter(max_iters: u16) -> ExecutionError {
        let details = format!("Iteration Error - Exceeded max number of iterations ({})", max_iters);
        ExecutionError{details, code: 1}
    }
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ExecutionError {
    fn description(&self) -> &str {
        &self.details
    }
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

fn check_brackets_match(program: &str) -> Result<(), ExecutionError> {
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

    if open_brackets == 0 {
        return Ok(());
    }
    
    let mut mismatched = 0;
    let mut problem_char = '❌';
    if open_brackets > 0 {
        mismatched = last_open_bracket;
        problem_char = '[';
    } else if open_brackets < 0 {
        mismatched = last_close_bracket;
        problem_char = ']';
    }

    return Err(ExecutionError::new_syntax(mismatched, &format!("Unmatched bracket '{}'", problem_char), &program))
}

fn find_location(location: usize, program: &str) -> (usize, i32) {
    let line_num = program[0..location].matches("\n").count() + 1;

    let mut last_newline = -1;
    let mut i = (location as i32) - 1;
    while i >= 0 {
        if &program[i as usize..(i+1) as usize] == "\n" {
            last_newline = i;
            i = 0;
        }
        i -= 1;
    }

    let char_num = location as i32 - last_newline;

    return (line_num, char_num);
}

fn read_file(file_path: &str) -> Result<String, ExecutionError> {
    if let Ok(contents) = fs::read_to_string(file_path) {
        Ok(contents.to_owned())
    } else {
        Err(ExecutionError::new_file("The specified file was not found"))
    }
}

fn interpret(program: String, mut ptr: usize, mut arr: [u8; usize::pow(2, 16)], re: &Regex, safe: bool) -> Result<(usize, [u8; usize::pow(2, 16)]), ExecutionError> {
    let max_iters: u16 = u16::MAX;
    let mut num_iters: u32 = 0;

    let mut i: usize = 0;
    while i < program.len() {
        if num_iters > max_iters as u32 {
            return Err(ExecutionError::new_iter(max_iters));
        }
        
        let current_char_option = program.chars().nth(i);
        let current_char;
        match current_char_option {
            Some(value) => {
                current_char = value;
            },
            None => {
                return Err(ExecutionError::new_parsing(i, &program));
            }
        }

        match current_char {
            current_char if re.is_match(&current_char.to_string()) => {},
            '>' => {
                if ptr < u16::MAX as usize {
                    ptr += 1;
                } else {
                    if safe {
                        return Err(ExecutionError::new_overflow(i, "Overflow", "data pointer", &program));
                    } else {
                        ptr = 0;
                    }
                }
            },
            '<' => {
                if ptr > 0 {
                    ptr -= 1;
                } else {
                    if safe {
                        return Err(ExecutionError::new_overflow(i, "Underflow", "data pointer", &program));
                    } else {
                        ptr = u16::max as usize;
                    }
                }
            },
            '+' => {
                if arr[ptr] < u8::MAX {
                    arr[ptr] += 1;
                } else {
                    if safe {
                        let overflow_location = String::from("array index ") + &ptr.to_string();
                        return Err(ExecutionError::new_overflow(i, "Overflow", &overflow_location, &program));
                    } else {
                        arr[ptr] = 0;
                    }
                }
            },
            '-' => {
                if arr[ptr] > 0 {
                    arr[ptr] -= 1;
                } else {
                    if safe {
                        let overflow_location = String::from("array index ") + &ptr.to_string();
                        return Err(ExecutionError::new_overflow(i, "Underflow", &overflow_location, &program));
                    } else {
                        arr[ptr] = u8::MAX;
                    }
                }
            },
            '.' => {print!("{}", arr[ptr] as char);}
            ',' => {
                let term = Term::stdout();
                if let Ok(input) = term.read_char() {
                    let mut b = [0u8; 4];
                    input.encode_utf8(&mut b);
                    if safe {
                        arr[ptr] = b[0];
                    } else {
                        for (j, byte) in b.iter().enumerate() {
                            if *byte == 0 {break;}

                            arr[ptr + j] = *byte;
                        }
                    }
                }
            },
            ':' => {print!("{}", arr[ptr]);},
            ';' => {
                let term = Term::stdout();
                if let Ok(input) = term.read_char() {
                    let mut b = [0u8; 4];
                    input.encode_utf8(&mut b);
                    if safe {
                        arr[ptr] += b[0];
                    } else {
                        for (j, byte) in b.iter().enumerate() {
                            if *byte == 0 {break;}

                            arr[ptr + j] += *byte;
                        }
                    }
                }
            },
            '[' => {
                if arr[ptr] == 0 {
                    i = find_matching_bracket(i, &program);
                    num_iters = 0;
                }
            },
            ']' => {
                if arr[ptr] != 0 {
                    i = find_matching_bracket(i, &program);
                    num_iters += 1;
                }
            },
            _ => {}
        }

        i += 1;
    }

    return Ok((ptr, arr));
}

fn execute_file(program: String, safe: bool, re_base: &str) -> Result<(), ExecutionError> {
    let re = Regex::new(re_base).unwrap();

    if let Err(err) = check_brackets_match(&program) {
        return Err(err);
    }

    let ptr: usize = 0;
    let arr: [u8; usize::pow(2, 16)] = [0; usize::pow(2, 16)];

    match interpret(program, ptr, arr, &re, safe) {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}

fn main() -> Result<(), u8> {
    let cli = Cli::parse();

    let safe = !cli.is_unsafe;
    if !safe {
        println!("Starting in unsafe mode.");
    }

    let re_base;
    if cli.extended {
        re_base = r"[^+-><\[\],.;:]";
        println!("Using extended syntax.")
    } else {
        re_base = r"[^+-><\[\],.]";
    }

    let result = if let Some(error_help) = cli.error_help.as_deref() {
        println!("{}", match error_help {
            "overflow" | "underflow" | "overflow error" => text::HELP_OVERFLOW,
            "syntax" | "syntax error" => text::HELP_SYNTAX,
            "file" | "file handling" | "file handling error" => text::HELP_FILE,
            "parsing" | "parsing error" => text::HELP_PARSING,
            "internal" | "internal error" => text::HELP_INTERNAL,
            "iteration" | "iteration error" => text::HELP_ITER,
            _ => "Unknown error type"
        });

        Ok(())
    } else if let Some(path) = cli.path.as_deref() {
        let program_result = read_file(path);
        match program_result {
            Ok(program) => execute_file(program, safe, re_base),
            Err(err) => Err(err)
        }
    } else if cli.repl {
        let exit_command = match env::consts::OS {
            "macos" => "Command + C",
            _ => "Ctrl + C",
        };
        println!("REPL session activated. Press {} to exit.", exit_command);

        let re = Regex::new(re_base).unwrap();

        let mut ptr: usize = 0;
        let mut arr: [u8; usize::pow(2, 16)] = [0; usize::pow(2, 16)];

        loop {
            let term = Term::stdout();
            if let Ok(input) = Term::read_line(&term) {
                let result = interpret(input, ptr, arr, &re, safe);
                match result {
                    Ok((ptr_tmp, arr_tmp)) => {
                        ptr = ptr_tmp;
                        arr = arr_tmp;
                    },
                    Err(err) => {
                        eprintln!("{}", err.details);
                    }
                }
            }
        }
    } else {
        Err(ExecutionError::new_file("failed"))
    };

    match result {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("{}", err.details);
            Err(err.code)
        }
    }
}
