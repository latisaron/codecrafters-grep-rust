use std::env;
use std::io;
use std::process;

fn match_recursively(input_line: &str, pattern: &str) -> bool {
    // eprintln!("input line is {:?} pattern is {:?}", input_line, pattern);
    if pattern.is_empty() {
        true
    } else if input_line.is_empty() {
        false
    } else if pattern.starts_with("\\d") {
        if let Some(character) = input_line.chars().next() { 
            matches!(character, '0'..='9') && match_recursively(&input_line[1..], &pattern[2..])
        } else {
            false
        }
    } else if pattern.starts_with("\\w") {
        if let Some(character) = input_line.chars().next() { 
            (character == '_' || matches!(character, '0'..='9') || matches!(character, 'a'..='z') || matches!(character, 'A'..='Z')) &&
                match_recursively(&input_line[1..], &pattern[2..])
        } else {
            false
        }
    } else if input_line[0..1] == pattern[0..1] && match_recursively(&input_line[1..], &pattern[1..]) {
        true
    } else {
        false
    }
}

fn match_pattern(mut input_line: &str, pattern: &str) -> bool {
    if pattern.is_empty() {
        return true;
    } else {
        if pattern.starts_with("[^") && pattern.ends_with(']') { 
            input_line.chars().any(|c| pattern[2..(pattern.len() - 1)].chars().all(|pc| pc != c))
        } else if pattern.starts_with('[') && pattern.ends_with(']') { 
            pattern[1..(pattern.len() - 1)].chars().any(|c| input_line.contains(c))
        } else {
            while !input_line.is_empty() {
                if match_recursively(input_line, pattern) {
                    return true;
                }
                input_line = &input_line[1..];
            }
            false
        }
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!!!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line, &pattern) {
        eprintln!("is good");
        process::exit(0)
    } else {
        eprintln!("is bad");
        process::exit(1)
    }
}
