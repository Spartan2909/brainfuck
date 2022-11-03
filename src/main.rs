use std::fs;
use std::env;
use std::process;

use console::Term;
use either::Either;

fn find_matching_bracket(start_index: usize, program: &str) -> usize {
    let mut open_brackets = 0;

    let start_char = &program[start_index..start_index+1];

    for i in match start_char {
        "[" => Either::Left(start_index..program.len()),
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

fn check_brackets_match(program: &str) -> (bool, usize) {
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
    if open_brackets > 0 {
        mismatched = last_open_bracket;
    } else if open_brackets < 0 {
        mismatched = last_close_bracket;
    }

    return (brackets_match, mismatched);
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

    let char_num = program[last_newline..location+1].chars().count();

    return (line_num, char_num);
}

fn overflow_error(location: usize, problem: &str, overflow_location: &str, program: &str) -> String {
    let readable_location = find_location(location, program);
    return format!("{}:{} - Overflow Error - {} at {}", readable_location.0, readable_location.1, problem, overflow_location);
}

fn syntax_error(location: usize, problem: &str, program: &str) -> String {
    let readable_location = find_location(location, program);
    return format!("{}:{} - Syntax Error - {}", readable_location.0, readable_location.1, problem);
}

fn parsing_error(location: usize, program: &str) -> String {
    let readable_location = find_location(location, program);
    return format!("{}:{} - Parsing Error", readable_location.0, readable_location.1);
}

fn file_error() -> String {
    return String::from("File handling error");
}

fn raise_error(error: String) {
    println!("{}", &error);
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut program = "";
    let s;

    if let Ok(contents) = fs::read_to_string(file_path) {
        s = contents.to_owned();
        program = s.as_str();
    } else {
        raise_error(file_error());
    }

    let check_match = check_brackets_match(&program);
    if !check_match.0 {
        raise_error(syntax_error(check_match.1, "Unmatched bracket", &program))
    }

    let mut ptr = 0 as usize;
    let mut arr: [u8; u16::MAX as usize] = [0; u16::MAX as usize];
    let mut i: usize = 0;
    while i < program.len() {
        let current_char = program.chars().nth(i);
        let mut item = '0';
        if current_char.is_some() {
            item = current_char.expect("Internal error");
        } else {
            raise_error(parsing_error(i, &program))
        }

        match item {
            '>' => {
                if ptr < u16::MAX as usize {
                    ptr += 1;
                } else {
                    raise_error(overflow_error(i, "Overflow", "pointer", &program));
                }
            }
            '<' => {
                if ptr > 0 {
                    ptr -= 1;
                } else {
                    raise_error(overflow_error(i, "Underflow", "pointer", &program));
                }
            }
            '+' => {
                if arr[ptr] < u8::MAX {
                    arr[ptr] += 1;
                } else {
                    let overflow_location = String::from("array index ") + &i.to_string();
                    raise_error(overflow_error(i, "Overflow", &overflow_location, &program));
                }
            }
            '-' => {
                if arr[ptr] > 0 {
                    arr[ptr] -= 1;
                } else {
                    let overflow_location = String::from("array index ") + &i.to_string();
                    raise_error(overflow_error(i, "Underflow", &overflow_location, &program));
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
                }
            }
            ']' => {
                if arr[ptr] != 0 {
                    i = find_matching_bracket(i, &program);
                }
            }
            _ => {}
        }

        i += 1;
    }
}
