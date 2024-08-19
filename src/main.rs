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

            let (result, token_output_string) = scan_token(&file_contents);
            println!("{}", token_output_string);
            exit(result)
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

pub fn scan_token(file_contents: &str) -> (i32, String) {
    let mut line_number = 1;
    let mut result = 0;
    let mut chars = file_contents.chars();

    let mut token_output_string = String::new();

    while let Some(c) = chars.next() {
        match c {
            '(' => {
                token_output_string.push_str("LEFT_PAREN ( null\n");
            }
            ')' => {
                token_output_string.push_str("RIGHT_PAREN ) null\n");
            }
            '{' => {
                token_output_string.push_str("LEFT_BRACE { null\n");
            }
            '}' => {
                token_output_string.push_str("RIGHT_BRACE } null\n");
            }
            '*' => {
                token_output_string.push_str("STAR * null\n");
            }
            '.' => {
                token_output_string.push_str("DOT . null\n");
            }
            ',' => {
                token_output_string.push_str("COMMA , null\n");
            }
            '+' => {
                token_output_string.push_str("PLUS + null\n");
            }
            '-' => {
                token_output_string.push_str("MINUS - null\n");
            }
            '/' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('/') {
                    while let Some(c) = chars.next() {
                        if c == '\n' {
                            line_number += 1;
                            break;
                        }
                    }
                } else {
                    token_output_string.push_str("SLASH / null\n");
                }
            }
            ';' => {
                token_output_string.push_str("SEMICOLON ; null\n");
            }
            '\n' => line_number += 1,
            '=' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('=') {
                    token_output_string.push_str("EQUAL_EQUAL == null\n");

                    chars.next();
                } else {
                    token_output_string.push_str("EQUAL = null\n");
                }
            }
            '!' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('=') {
                    token_output_string.push_str("BANG_EQUAL != null\n");
                    chars.next();
                } else {
                    token_output_string.push_str("BANG ! null\n");
                }
            }
            '<' => {
                let mut peekable = chars.clone().peekable();
                let next_char = peekable.next();

                if next_char == Some('=') {
                    token_output_string.push_str("LESS_EQUAL <= null\n");
                    chars.next();
                } else if next_char == Some('|') {
                    while let Some(c) = chars.next() {
                        if c == '>' {
                            break;
                        }
                    }
                } else {
                    token_output_string.push_str(&format!("LESS {} null\n", c));
                }
            }
            '>' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('=') {
                    token_output_string.push_str("GREATER >= null\n");
                    chars.next();
                } else {
                    token_output_string.push_str(&format!("GREATER {} null\n", c));
                }
            }
            '"' => {
                let mut word = String::new();
                let mut is_complete_string = false;
                while let Some(c) = chars.next() {
                    if c == '"' {
                        is_complete_string = true;
                        break;
                    }
                    word.push(c);
                }


                if is_complete_string {
                    token_output_string.push_str(&format!("STRING \"{}\" {}\n", word, word));
                } else {
                    token_output_string.push_str(&format!("[line {}] Error: Unterminated string.\n", line_number));
                    result = 65;
                }
            }

            '0'..='9' => {
                let mut number = String::new();
                number.push(c);

                while let Some(c) = chars.next() {
                    if c.is_digit(10) {
                        number.push(c);
                    } else if c == '.' {
                        number.push('.');
                    } else {
                        break;
                    }
                }
                token_output_string.push_str(&format!("NUMBER {} {}\n", number, number));
            }
            ' ' => {}
            '\t' => {}
            _ => {
                token_output_string.push_str(&format!("[line {}] Error: Unexpected character: {}\n", line_number, c));
                result = 65;
            }
        }
    };

    token_output_string.push_str("EOF  null");
    return (result, token_output_string);
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
