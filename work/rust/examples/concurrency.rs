// Concurrency in Rust
// Fearless concurrency - preventing data races at compile time!

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Concurrency Demo ===\n");

    basic_threads();
    message_passing();
    shared_state();
}

fn basic_threads() {
    println!("1. Basic Threads:");

    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("  Thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..3 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for thread to finish
    handle.join().unwrap();
    println!();
}

fn message_passing() {
    println!("2. Message Passing (Channels):");

    use std::sync::mpsc; // Multiple producer, single consumer

    let (tx, rx) = mpsc::channel();

    // Clone transmitter for multiple producers
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Receive messages
    for received in rx {
        println!("  Got: {}", received);
    }
    println!();
}

fn shared_state() {
    println!("3. Shared State (Arc + Mutex):");

    // Arc = Atomic Reference Counted (thread-safe Rc)
    // Mutex = Mutual exclusion lock
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;

            println!("  Thread {} incremented counter", i);
        });

        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    println!("  Final count: {}\n", *counter.lock().unwrap());
}

// Advanced: Send and Sync traits
//
// Send: Type can be transferred between threads
// Sync: Type can be referenced from multiple threads
//
// Compiler automatically implements these traits!
// If a type is not Send/Sync, you can't use it across threads.
// This prevents data races at COMPILE TIME!

struct NotSendOrSync {
    // Example: types with raw pointers are not Send/Sync
    _ptr: *const u8,
}

// This won't compile:
// fn try_to_send() {
//     let x = NotSendOrSync { _ptr: std::ptr::null() };
//     thread::spawn(move || {
//         println!("{:?}", x._ptr); // ERROR: NotSendOrSync is not Send
//     });
// }

// Comparison with C++:
//
// 1. Data Race Prevention:
//    - Rust: Compile-time enforcement via Send/Sync
//    - C++: Runtime errors, undefined behavior
//
// 2. Shared State:
//    - Rust: Arc<Mutex<T>> explicit locking
//    - C++: std::shared_ptr<T> + std::mutex
//
// 3. Message Passing:
//    - Rust: Built-in channels (mpsc)
//    - C++: Third-party or manual implementation
//
// 4. Thread Safety:
//    - Rust: Type system prevents data races
//    - C++: ThreadSanitizer for runtime detection
//
// Key Points for HFT:
// - Lock-free structures need unsafe (see crossbeam crate)
// - Channels have overhead, shared state can be faster
// - Use atomics for simple counters/flags
// - Rayon crate for data parallelism
// - Consider thread pinning for consistent latency

// Example: Lock-free counter with atomics
use std::sync::atomic::{AtomicUsize, Ordering};

fn atomic_example() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // No mutex needed!
            counter.fetch_add(1, Ordering::SeqCst);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Atomic counter: {}", counter.load(Ordering::SeqCst));
}
