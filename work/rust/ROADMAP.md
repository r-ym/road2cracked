# Rust Learning Roadmap

## Phase 1: Fundamentals (Week 1-2)

### Basic Syntax
- [ ] Variables and mutability
- [ ] Data types (scalar and compound)
- [ ] Functions
- [ ] Comments
- [ ] Control flow (if, loop, while, for)

### Ownership (CRITICAL)
- [ ] Ownership rules
- [ ] References and borrowing
- [ ] The slice type
- [ ] Understanding the stack vs heap

**Practice**: Implement basic data structures (vector, string manipulation)

## Phase 2: Core Concepts (Week 3-4)

### Structs and Enums
- [ ] Defining and instantiating structs
- [ ] Method syntax
- [ ] Enums and pattern matching
- [ ] Option and Result types

### Collections
- [ ] Vector
- [ ] String
- [ ] HashMap

### Error Handling
- [ ] Recoverable errors with Result
- [ ] Unrecoverable errors with panic!
- [ ] The ? operator

**Practice**: Build a simple CLI tool or data processor

## Phase 3: Intermediate (Week 5-6)

### Generics and Traits
- [ ] Generic data types
- [ ] Trait definitions
- [ ] Trait bounds
- [ ] Implementing traits
- [ ] Trait objects vs generics

### Lifetimes
- [ ] Lifetime syntax
- [ ] Lifetime annotations in functions
- [ ] Lifetime annotations in structs
- [ ] Lifetime elision rules

### Testing
- [ ] Writing unit tests
- [ ] Integration tests
- [ ] Test organization

**Practice**: Re-implement C++ data structures in Rust (linked list, binary tree)

## Phase 4: Advanced (Week 7-8)

### Closures and Iterators
- [ ] Closure syntax and capturing
- [ ] Iterator trait
- [ ] Consuming and iterator adaptors
- [ ] Performance: iterators vs loops

### Smart Pointers
- [ ] Box<T>
- [ ] Rc<T> and Arc<T>
- [ ] RefCell<T> and interior mutability
- [ ] Preventing reference cycles

### Concurrency
- [ ] Threads
- [ ] Message passing with channels
- [ ] Shared state with Mutex and Arc
- [ ] Send and Sync traits

**Practice**: Implement concurrent algorithms, solve LeetCode concurrency problems

## Phase 5: Expert Topics (Week 9+)

### Unsafe Rust
- [ ] Raw pointers
- [ ] Unsafe functions and methods
- [ ] FFI (Foreign Function Interface)
- [ ] When and why to use unsafe

### Macros
- [ ] Declarative macros with macro_rules!
- [ ] Procedural macros
- [ ] Attribute-like and function-like macros

### Async Programming
- [ ] async/await syntax
- [ ] Futures
- [ ] Tokio runtime
- [ ] Async I/O

**Practice**: Build a web server or async data processor

## Ongoing Practice

### Compare with C++
- Implement same algorithms in both languages
- Compare memory management approaches
- Understand performance characteristics
- Learn when to use each language

### Real Projects
- CLI tools (ripgrep, fd style)
- Network applications
- Systems programming
- WebAssembly modules

### Interview Preparation
- Solve LeetCode problems in Rust
- Implement common data structures
- Practice explaining ownership and borrowing
- Understand Rust's zero-cost abstractions
