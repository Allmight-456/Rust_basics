# Rust Learning Journey

This repository documents my journey learning Rust programming language. It contains separate files exploring various Rust concepts and features.

## Contents

1. **Structs**: Demonstrating custom data types.
2. **Enums**: Exploring enumerated types and pattern matching.
3. **Result Enum**: Handling operations that can fail.
4. **Option Enum**: Working with optional values.
5. **Error Handling**: Techniques for robust error management.

## Key Concepts Covered

- Ownership and borrowing
- Lifetimes
- Pattern matching
- Generic types
- Traits and implementations
- Iterators and closures
- Error propagation

## Code Snippets

Each file contains examples and explanations of Rust's unique features. For instance:

```rust
// Example of a struct with implementation
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Example of error handling with Result
fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(x / y)
    }
}

```
## Running the code 
- cd path_to_directory_of_file
- rustc path_to_file.rs
- ./filename

- OR

- Use cargo run --bin <filename> to run a specific file.

## Learning Resources
- The Rust Programming Language Book
- Rust By Example
- docs.rust-lang.org

