mod lexer;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum CommandType {
    Tokenize,
}

impl FromStr for CommandType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tokenize" => Ok(CommandType::Tokenize),
            _ => Err(()),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = CommandType::from_str(&args[1]).unwrap();
    let filename = &args[2]; // Validate


    match command {
        CommandType::Tokenize => {
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
            writeln!(io::stderr(), "Unknown command: {:?}", command).unwrap();
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
                    token_output_string.push_str("GREATER_EQUAL >= null\n");
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
                    writeln!(io::stderr(), "[line {}] Error: Unterminated string.", line_number).unwrap();
                    result = 65;
                }
            }

            '0'..='9' => {
                let mut number = String::new();
                let mut output_number = String::new();

                let mut has_dot_appeared = false; // to keep track of fraction number
                let mut is_decimal = false;
                number.push(c);

                let mut peekable = chars.clone().peekable();
                while let Some(c) = peekable.peek() {
                    if c.is_digit(10) {
                        number.push(*c);
                        chars.next();
                    } else if *c == '.' && has_dot_appeared == false {
                        // now check the second character if it is not a digit then break;
                        let mut peekable_clone = peekable.clone();
                        peekable_clone.next();

                        if let Some(c) = peekable_clone.peek() {
                            // println!("hello world {}", c);
                            if !c.is_digit(10) {
                                break;
                            }
                        } else {
                            break;
                        }

                        has_dot_appeared = true;
                        number.push('.');
                        chars.next();
                    } else {
                        break;
                    }

                    peekable.next();
                }
                if !has_dot_appeared {
                    output_number.push_str(&format!("{}.0", number));
                } else {
                    // Format the number, removing unnecessary trailing zeros
                    let parsed_number: f64 = number.parse::<f64>().unwrap();

                    // Format the number, removing unnecessary trailing zeros
                    let output_number_str = if parsed_number.fract() == 0.0 {
                        format!("{:.1}", number) // No decimal part, format as integer
                    } else {
                        format!("{}", number) // Retain full precision
                    };
                    output_number.push_str(&format!("{}", output_number_str));
                }

                token_output_string.push_str(&format!("NUMBER {} {}\n", number, output_number));
            }
            ' ' => {}
            '\t' => {}
            'A'..='z' => {
                let mut word = String::new();
                word.push(c);

                let mut peekable = chars.clone().peekable();
                while let Some(char) = peekable.peek() {
                    if char.is_alphabetic() || char.is_digit(10) || *char == '_' {
                        word.push(*char);
                        chars.next();
                    } else {
                        break;
                    }

                    peekable.next();
                }

                token_output_string.push_str(&format!("IDENTIFIER {} null\n", word));
            }
            _ => {
                writeln!(io::stderr(), "[line {}] Error: Unexpected character: {}", line_number, c).unwrap();
                result = 65;
            }
        }
    };

    token_output_string.push_str("EOF  null");
    return (result, token_output_string);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_token_number_with_dots() {
        let file_contents = "1234.1234.1234.";
        let (result, token_output_string) = scan_token(file_contents);
        println!("{}", token_output_string);
        let expected_output_token_string = "NUMBER 1234.1234 1234.1234\nDOT . null\nNUMBER 1234 1234.0\nDOT . null\nEOF  null";
        assert_eq!(result, 0);
        assert_eq!(token_output_string, expected_output_token_string);
    }
}

