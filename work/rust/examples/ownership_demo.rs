// Ownership and Borrowing Demonstration
// This is the MOST important concept in Rust

fn main() {
    println!("=== Ownership Demo ===\n");

    // 1. Ownership basics
    ownership_basics();

    // 2. References and borrowing
    borrowing_demo();

    // 3. Mutable references
    mutable_references();

    // 4. Slices
    slice_demo();
}

fn ownership_basics() {
    println!("1. Ownership Basics:");

    // String is heap-allocated (similar to std::string in C++)
    let s1 = String::from("hello");

    // This MOVES s1 to s2 (s1 is no longer valid)
    let s2 = s1;

    // println!("{}", s1); // ERROR! s1 was moved
    println!("s2 = {}", s2);

    // To copy, use clone (like deep copy in C++)
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);

    // Primitive types implement Copy trait (stack-only)
    let x = 5;
    let y = x; // x is still valid!
    println!("x = {}, y = {}\n", x, y);
}

fn borrowing_demo() {
    println!("2. Borrowing (Immutable References):");

    let s = String::from("hello");

    // Pass reference (borrow) instead of moving
    let len = calculate_length(&s);

    println!("Length of '{}' is {}\n", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // s goes out of scope but doesn't own the data, so nothing happens
}

fn mutable_references() {
    println!("3. Mutable References:");

    let mut s = String::from("hello");

    // Mutable borrow
    change(&mut s);

    println!("Changed: {}", s);

    // Key rule: Only ONE mutable reference at a time!
    // This prevents data races at compile time

    let r1 = &mut s;
    // let r2 = &mut s; // ERROR! Can't have two mutable references
    r1.push_str("!");
    println!("After r1: {}\n", r1);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn slice_demo() {
    println!("4. Slices:");

    let s = String::from("hello world");

    // String slices
    let hello = &s[0..5];  // or &s[..5]
    let world = &s[6..11]; // or &s[6..]

    println!("First word: {}", hello);
    println!("Second word: {}", world);

    // Array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    println!("Array slice: {:?}", slice);
}

// Key Takeaways:
// 1. Every value has ONE owner
// 2. When owner goes out of scope, value is dropped
// 3. Either many immutable refs OR one mutable ref (never both)
// 4. References must always be valid (no dangling pointers!)
//
// This prevents:
// - Use after free
// - Double free
// - Dangling pointers
// - Data races
//
// All at COMPILE TIME with zero runtime cost!
