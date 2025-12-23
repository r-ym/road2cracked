// Traits and Generics in Rust
// Compare with C++ templates and interfaces

use std::fmt::Display;

fn main() {
    println!("=== Traits and Generics Demo ===\n");

    generic_functions();
    trait_bounds();
    trait_objects();
}

// Generic function (similar to C++ templates)
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn generic_functions() {
    println!("1. Generic Functions:");

    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("Largest number: {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("Largest char: {}\n", result);
}

// Trait definition (like interface in C++)
trait Summary {
    fn summarize(&self) -> String;

    // Default implementation
    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

struct Article {
    headline: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.content)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

// Trait bound syntax
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
fn display_and_summarize<T: Summary + Display>(item: &T) {
    println!("{}", item);
}

// Where clause (cleaner for complex bounds)
fn complex_bounds<T, U>(t: &T, u: &U) -> String
where
    T: Summary + Display,
    U: Clone + Summary,
{
    format!("{}", t.summarize())
}

fn trait_bounds() {
    println!("2. Trait Bounds:");

    let article = Article {
        headline: String::from("Rust wins again"),
        content: String::from("Memory safety without GC!"),
    };

    notify(&article);
    println!();
}

// Trait objects (dynamic dispatch, like virtual functions in C++)
fn trait_objects() {
    println!("3. Trait Objects (Dynamic Dispatch):");

    let items: Vec<Box<dyn Summary>> = vec![
        Box::new(Article {
            headline: String::from("Tech News"),
            content: String::from("Rust 2.0 released"),
        }),
        Box::new(Tweet {
            username: String::from("rustlang"),
            content: String::from("Fearless concurrency!"),
        }),
    ];

    for item in items {
        println!("- {}", item.summarize());
    }
    println!();
}

// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// Implementation for specific type
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Key Differences from C++:
//
// 1. Traits vs Templates:
//    - Rust: Traits define behavior, must be explicit
//    - C++: Duck typing with templates
//
// 2. Monomorphization:
//    - Both Rust and C++ generate code for each type
//    - Zero runtime cost for static dispatch
//
// 3. Trait Objects:
//    - Rust: Box<dyn Trait> for dynamic dispatch
//    - C++: Virtual functions and vtables
//
// 4. Coherence:
//    - Rust: Orphan rule prevents conflicts
//    - C++: Can cause ODR violations
