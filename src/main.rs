use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

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

            let result = scan_token(&file_contents);
            exit(result)
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

pub fn scan_token(file_contents: &str) -> i32 {
    let mut line_number = 1;
    let mut result = 0;
    let mut chars = file_contents.chars();

    while let Some(c) = chars.next() {
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
            '\n' => line_number += 1,
            '=' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('=') {
                    println!("EQUAL_EQUAL {}{} null", c, c);
                    chars.next();
                } else {
                    println!("EQUAL {} null", c);
                }
            }
            '!' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('=') {
                    println!("BANG_EQUAL != null");
                    chars.next();
                } else {
                    println!("BANG {} null", c);
                }
            }
            '<' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('=') {
                    println!("LESS_EQUAL <= null");
                    chars.next();
                } else {
                    println!("LESS {} null", c);
                }
            }
            '>' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('=') {
                    println!("GREATER_EQUAL >= null");
                    chars.next();
                } else {
                    println!("GREATER {} null", c);
                }
            }
            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", line_number, c);
                result = 65;
            }
        }
    };

    println!("EOF  null");
    return result;
}


// TODO
// i will use it later
enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Slash,
    Semicolon,
    Equal,
    EqualEqual,
    Bang,
}
