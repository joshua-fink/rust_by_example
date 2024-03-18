// threads

// Rust provides mechanism for spawning native OS threads via spawn function, arg of function is moving closure

use std::thread;

const NTHREADS: u32 = 10;

// Main thread
fn main() {
    // Make vector to hold children which are spawned
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Spin another thread
        children.push (thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        // Wait for thread to finish. Returns result
        let _ = child.join();
    }
}

// Threads will be scheduled by OS
