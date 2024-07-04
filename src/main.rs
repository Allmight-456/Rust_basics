// Some rules of ownership and borrow in RUST 
//Ownership Rules:
// a. Each value in Rust has a variable that's called its owner.
// b. There can only be one owner at a time.
// c. When the owner goes out of scope, the value will be dropped.
// Borrowing Rules:
// a. At any given time, you can have either one mutable reference or any number of immutable references.
// b. References must always be valid.

fn main() {
    let s1 = String::from("hello"); // s1 owns the String
    let s2 = s1; // ownership is moved from s1 to s2
    // println!("{}", s1); // This would cause an error - s1 no longer owns the String
    println!("{}", s2); // This is fine
} // s2 goes out of scope and the String is dropped

#[allow(dead_code)]            // to remind the compiler that function is not used
fn ownership() {
    let mut s = String::from("hello");

    // Immutable borrowing
    let r1 = &s; // OK
    let r2 = &s; // OK - multiple immutable borrows are allowed
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    // Mutable borrowing
    let r3 = &mut s; // OK - mutable borrow
    // let r4 = &s; // This would be an ERROR - can't borrow as immutable while mutably borrowed
    println!("{}", r3);
}

#[allow(dead_code)]
fn borrowing() {
    let mut s = String::from("hello");

    // Immutable borrowing
    let r1 = &s; // OK
    let r2 = &s; // OK - multiple immutable borrows are allowed
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    // Mutable borrowing
    let r3 = &mut s; // OK - mutable borrow
    // let r4 = &s; // This would be an ERROR - can't borrow as immutable while mutably borrowed
    println!("{}", r3);
}