/*
fn main() {
    // --- Ownership rules ---
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world!", s1);
}
*/

fn main() {
    // ---- The Rules of References ---
    // 1. At any given time, you can have either one mutable reference 
    // or any number of immutable references.
    // 2. References must always be valid.
}