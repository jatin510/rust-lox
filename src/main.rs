use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            scan_token(&file_contents);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

pub fn scan_token(file_content: &str) {
    file_content.chars().for_each(|c| {
        match c {
            '(' => println!("LEFT_PAREN {} null", c),
            ')' => println!("RIGHT_PAREN {} null", c),
            '{' => println!("LEFT_BRACE {} null", c),
            '}' => println!("RIGHT_BRACE {} null", c),
            '*' => println!("STAR {} null", c),
            '.' => println!("DOT {} null", c),
            ',' => println!("COMMA {} null", c),
            '+' => println!("PLUS {} null", c),
            '-' => println!("MINUS {} null", c),
            '/' => println!("SLASH {} null", c),
            ';' => println!("SEMICOLON {} null", c),
            _ => (println!("not handled yet {} null", c)),
        }
    });
    println!("EOF  null");
}

