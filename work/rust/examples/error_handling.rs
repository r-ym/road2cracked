// Error Handling in Rust
// Compare with C++ exceptions and error codes

use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("=== Error Handling Demo ===\n");

    option_type();
    result_type();
    question_mark_operator();
}

// Option<T> - for values that might not exist
fn option_type() {
    println!("1. Option Type:");

    let numbers = vec![1, 2, 3, 4, 5];

    // Safe indexing (returns Option)
    match numbers.get(2) {
        Some(value) => println!("Found: {}", value),
        None => println!("Not found"),
    }

    // Unwrap or default
    let value = numbers.get(10).unwrap_or(&0);
    println!("Value: {}", value);

    // Map and other combinators
    let doubled = numbers.get(2).map(|x| x * 2);
    println!("Doubled: {:?}\n", doubled);
}

// Result<T, E> - for operations that can fail
fn result_type() {
    println!("2. Result Type:");

    match read_file("nonexistent.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error: {}", e),
    }
    println!();
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?; // ? propagates errors
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// The ? operator (elegant error propagation)
fn question_mark_operator() {
    println!("3. Question Mark Operator:");

    match process_data() {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Processing error: {}", e),
    }
}

fn process_data() -> Result<i32, String> {
    let data = get_data()?;
    let processed = transform_data(data)?;
    Ok(processed)
}

fn get_data() -> Result<i32, String> {
    Ok(42)
}

fn transform_data(x: i32) -> Result<i32, String> {
    if x > 0 {
        Ok(x * 2)
    } else {
        Err(String::from("Invalid data"))
    }
}

// Custom error types
#[derive(Debug)]
enum DataError {
    ParseError(String),
    ValidationError(String),
    IoError(io::Error),
}

impl std::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DataError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            DataError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            DataError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for DataError {}

// Comparison with C++:
//
// 1. No Exceptions:
//    - Rust: Explicit Result types
//    - C++: try/catch blocks
//
// 2. Forced Error Handling:
//    - Rust: Must handle Result explicitly
//    - C++: Exceptions can be ignored
//
// 3. Zero Cost:
//    - Rust: No stack unwinding overhead
//    - C++: Exception overhead even if not used
//
// 4. Type Safety:
//    - Rust: Error types are known at compile time
//    - C++: Can throw anything
//
// 5. Panic vs Exception:
//    - panic! - for unrecoverable errors (like std::abort)
//    - Result - for recoverable errors (preferred)
//
// Best Practices:
// - Use Option for values that may not exist
// - Use Result for operations that can fail
// - Use ? operator for clean error propagation
// - Define custom error types for libraries
// - Reserve panic! for truly exceptional cases
