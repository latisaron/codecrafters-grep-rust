use std::env;
use std::io;
use std::process;

fn match_recursively(input_line: &str, pattern: &str) -> bool {
    if pattern.starts_with("\\d") {
        input_line.chars().any(|c| matches!(c, '0'..='9')) &&
            match_recursively(&input_line[1..], &pattern[2..])
    } else if pattern.starts_with("\\w") {
        input_line.chars().any(|c| c == '_' || matches!(c, '0'..='9') || matches!(c, 'a'..='z') || matches!(c, 'A'..='Z')) &&
            match_recursively(&input_line[1..], &pattern[2..])
    } else {
        input_line.starts_with(pattern)
    }
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.is_empty() {
        return true;
    } else {
        if pattern.chars().count() == 1 {
            return input_line.contains(pattern);
        } else if pattern.starts_with("\\d") {
            return match_recursively(&input_line, &pattern);
        } else if pattern.starts_with("\\w") {
            return match_recursively(&input_line, &pattern);
        } else if pattern.starts_with("[^") && pattern.ends_with(']') { 
            input_line.chars().any(|c| pattern[2..(pattern.len() - 1)].chars().all(|pc| pc != c))
        } else if pattern.starts_with('[') && pattern.ends_with(']') { 
            pattern[1..(pattern.len() - 1)].chars().any(|c| input_line.contains(c))
        } else {
            panic!("Unhandled pattern: {}", pattern);
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
        process::exit(0)
    } else {
        process::exit(1)
    }
}
