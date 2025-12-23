# Rust Quick Start Guide

Get started learning Rust in 30 minutes.

## Step 1: Install Rust (5 min)

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

## Step 2: Hello World (5 min)

```bash
# Create new project
cargo new hello_rust
cd hello_rust

# Look at the generated files
cat src/main.rs

# Run it
cargo run
```

## Step 3: Understand Ownership (10 min)

Read and run the ownership demo:
```bash
rustc examples/ownership_demo.rs
./ownership_demo
```

Key concepts:
- Every value has ONE owner
- Ownership can be moved or borrowed
- No garbage collector needed!
- Memory safety guaranteed at compile time

## Step 4: Read The Book (Ongoing)

Start here: https://doc.rust-lang.org/book/

**Essential chapters for week 1:**
- Chapter 3: Common Programming Concepts
- Chapter 4: Understanding Ownership (CRITICAL!)
- Chapter 5: Structs
- Chapter 6: Enums and Pattern Matching

## Step 5: Practice (10 min)

Install Rustlings for interactive exercises:
```bash
cargo install rustlings
rustlings init
rustlings watch
```

## Your First Week Plan

### Day 1-2: Basics
- Install Rust
- Complete chapters 1-3 of The Book
- Write basic programs (calculator, FizzBuzz)

### Day 3-4: Ownership (CRITICAL)
- Chapter 4 of The Book (read twice!)
- Run ownership_demo.rs
- Do Rustlings ownership exercises
- This is the hardest part - take your time!

### Day 5: Structs and Enums
- Chapters 5-6 of The Book
- Build a simple struct-based program

### Day 6-7: Practice
- Solve 5 easy LeetCode problems in Rust
- Compare solutions with your C++ versions
- Note the differences

## Common Gotchas for C++ Developers

1. **No implicit copies** - Use `.clone()` explicitly
2. **Borrowing rules** - Only one mutable reference OR many immutable references
3. **No null** - Use `Option<T>` instead
4. **No exceptions** - Use `Result<T, E>` instead
5. **Macros are different** - `println!` is a macro, not a function
6. **String vs &str** - Two string types, understand the difference

## Running the Examples

```bash
cd work/rust/examples

# Compile and run ownership demo
rustc ownership_demo.rs && ./ownership_demo

# Compile and run traits demo
rustc traits_and_generics.rs && ./traits_and_generics

# Compile and run error handling
rustc error_handling.rs && ./error_handling

# Compile and run concurrency
rustc concurrency.rs && ./concurrency
```

## Next Steps

After your first week:
1. Review ROADMAP.md for structured learning path
2. Check RESOURCES.md for books and courses
3. Start solving LeetCode problems in Rust
4. Build a small CLI tool project
5. Join the Rust community (Discord, forums)

## Getting Help

- Rust Book: https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- r/rust subreddit
- Rust Users Forum: users.rust-lang.org
- Official Discord

## Remember

**Rust has a steep learning curve, but it's worth it!**

The compiler is your friend - error messages are helpful. Fighting the borrow checker is normal at first. Once ownership "clicks", everything else becomes easier.

Don't get discouraged - even experienced developers take time to learn Rust properly.
