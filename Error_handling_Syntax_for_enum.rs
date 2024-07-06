use std::fs;                   // importing the fs library used to read files like "example.txt"

// ways to write the struct and enums for different data structures
#[allow(dead_code)]
// Struct with named fields
struct Point<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
// Enum with tuple variants
enum Option<T> {
    Some(T),
    None,
}

#[allow(dead_code)]
// Enum with struct-like variants
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

#[allow(dead_code)]
// Tuple struct (uses parentheses)
struct Color(u8, u8, u8);

// an alternative to try catch error of JS , we have predefined Result enum library
//this is how it would look , but since predifned in library we dont need to define this enum
#[allow(dead_code)]
enum Result<A ,B>{
    Ok(A),
    Err(B),
}

fn main(){
    let res = fs::read_to_string("eaxample.txt");
    
    match res{
        Ok(content) => {
            println!("Content inside is : {}",content);

        }Err(err) => {
            println!("Error in loading the file : {}",err);
        }
    }
}