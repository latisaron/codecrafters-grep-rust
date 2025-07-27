use std::env;
use std::io;
use std::process;

fn match_recursively(input_line: &[u8], pattern: &[u8]) -> bool {
    // eprintln!("input line is {:?} pattern is {:?}", input_line, pattern);
    if pattern.is_empty() { 
        true
    } else if input_line.is_empty() {
        false
    } else if pattern.starts_with(b"\\d") {
        (b'0'..=b'9').contains(&input_line[0]) && match_recursively(&input_line[1..], &pattern[2..])
    } else if pattern.starts_with(b"\\w") {
        (
            input_line[0] == b'_' ||
            (b'0'..=b'9').contains(&input_line[0]) ||
            (b'A'..=b'Z').contains(&input_line[0]) ||
            (b'a'..=b'z').contains(&input_line[0])
        ) && match_recursively(&input_line[1..], &pattern[2..])
    } else {
        input_line[0] == pattern[0] && match_recursively(&input_line[1..], &pattern[1..])
    }
}

fn match_pattern(mut input_line: &[u8], pattern: &[u8]) -> bool {
    let pattern_length = pattern.len();
    if pattern.is_empty() {
        return true;
    } else {
        // println!("input line is {:?} and first is {:?} and {:?} and {:?}", input_line, input_line[0], input_line[0] == b'^', b'^');
        if pattern[0] == b'[' && pattern[pattern_length - 1] == b']' {
            if pattern[1] == b'^' {
                input_line.iter().any(|byte| pattern[1..(pattern_length - 1)].iter().all(|pattern_byte| byte != pattern_byte))
            } else {
                input_line.iter().any(|byte| pattern[1..(pattern_length - 1)].iter().any(|pattern_byte| byte == pattern_byte))
            }
        } else if pattern[0] == b'^' { 
            input_line.starts_with(&pattern[1..])
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

    let input_bytes = input_line.as_bytes();
    let pattern_bytes = pattern.as_bytes();

    // Uncomment this block to pass the first stage
    if match_pattern(input_bytes, pattern_bytes) {
        eprintln!("is good");
        process::exit(0)
    } else {
        eprintln!("is bad");
        process::exit(1)
    }
}
