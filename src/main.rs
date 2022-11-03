use std::fs;
use std::env;

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let program = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut ptr = 0u16;
    let mut arr: [u8; 65535] = [0; 65535];
    let mut i: usize = 0;
    while i < program.len() {
        let item = program.chars().nth(i).expect("");
        println!("position {}, character {}", i, item);
        match item {
            '>' => {ptr += 1;}
            '<' => {ptr -= 1;}
            '+' => {arr[ptr as usize] += 1;}
            '-' => {arr[ptr as usize] -= 1;}
            '.' => {print!("{}", arr[ptr as usize] as char);}
            ',' => {
                let term = Term::stdout();
                if let Ok(input) = Term::read_char(&term) {
                    let mut b = [0u8; 4];
                    input.encode_utf8(&mut b);
                    arr[ptr as usize] = b[0]
                }
            }
            '[' => {
                if arr[ptr as usize] == 0 {
                    i = find_matching_bracket(i, &program);
                }
            }
            ']' => {
                if arr[ptr as usize] != 0 {
                    i = find_matching_bracket(i, &program);
                }
            }
            _ => {}
        }

        i += 1;
    }
}
