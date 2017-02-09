extern crate ansi_term;
extern crate regex;

use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

use ansi_term::Colour;
use regex::Regex;

// Constants
/// Error exit code.
const EXIT_ERROR: i32 = 1;

/// Regex enum containing a `regex::Regex` and a `ansi_term::Colour`.
#[derive(Debug)]
enum RegexData {
    RegExp(Regex),
    Col(Colour),
}

fn main() {
    if env::args().count() < 2 {
        println!("No regexp file specified.");
        std::process::exit(EXIT_ERROR);
    }

    let file: String = env::args().nth(1).unwrap();
    let file_data = read_file(&file);

    println!("Data: {:#?}", file_data);
}

/// Read a file and generate a corresponding `Vec<RegexData>` containing read lines.
///
/// Has built-in validation to make sure that the number of capture groups
/// match. Matches whole line if no specific capture group specified.
fn read_file(filename: &String) -> Vec<RegexData> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(err) => {
            // Could not open file
            println!("Failed to read file {}.", filename);
            println!("Error: {}.", err);
            std::process::exit(EXIT_ERROR);
        }
    };

    let mut items = Vec::new();
    let reader = BufReader::new(&file);

    let mut captures = 0;
    for (idx, line) in reader.lines().enumerate() {
        let line_text = line.unwrap();
        match line_text.chars().nth(0) {
            Some('/') => {
                // Regex
                let rexp = make_regex(&line_text[1..], idx + 1);
                if captures != 0 {
                    // Invalid, can't add more
                    println!("Expecting a colour on line {}, got a regexp.", idx + 1);
                    std::process::exit(EXIT_ERROR);
                } else {
                    captures += rexp.captures_len();
                    items.push(RegexData::RegExp(rexp));
                }
            }
            Some('=') => {
                // Colour
                let rest_text = &line_text[1..];
                match rest_text {
                    "black" => items.push(RegexData::Col(Colour::Black)),
                    "red" => items.push(RegexData::Col(Colour::Red)),
                    "green" => items.push(RegexData::Col(Colour::Green)),
                    "yellow" => items.push(RegexData::Col(Colour::Yellow)),
                    "blue" => items.push(RegexData::Col(Colour::Blue)),
                    "purple" => items.push(RegexData::Col(Colour::Purple)),
                    "cyan" => items.push(RegexData::Col(Colour::Cyan)),
                    "white" => items.push(RegexData::Col(Colour::White)),
                    _ => {
                        // Invalid colour
                        println!("Invalid colour specified on line {}.", idx + 1);
                        println!("Input: {}", rest_text);
                        std::process::exit(EXIT_ERROR);
                    }
                }

                // If we got here, we have a valid colour, so count captures
                if captures == 0 {
                    println!("Extra colour given on line {}.", idx + 1);
                    println!("Input: {}", rest_text);
                    std::process::exit(EXIT_ERROR);
                }

                captures -= 1;
            }
            _ => {
                // Invalid
                println!("Warning: Invalid input on line {}.", idx + 1);
                println!("Input: {}", line_text);
            }
        }
    }

    // Done, check to see whether we have a valid number of captures
    if captures > 0 {
        println!("Expecting {} more colour statements.", captures);
        std::process::exit(EXIT_ERROR);
    }

    items
}

/// Create a regular expression from a string.
fn make_regex(rexp_str: &str, line: usize) -> Regex {
    match Regex::new(&*rexp_str) {
        Ok(rexp) => rexp,
        Err(err) => {
            // Could not compile regex
            println!("Regexp on line {} invalid:", line);
            println!("{}", err);
            std::process::exit(EXIT_ERROR);
        }
    }
}
