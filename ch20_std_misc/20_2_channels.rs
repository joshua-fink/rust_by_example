// Channels

// Rust provides async channels for communication between threads. Channels allow unidirectional flow of information between two end-points: `Sender` and `Receiver`

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

fn main() {
    // Channels have two endpoints: `Sender<T>` and `Receiver<T>`
    // `T` is type of msg to be transferred
    // (type annotation is superfluous)

    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();
    
    for id in 0..NTHREADS {
        // Sender endpoint can be copied
        let thread_tx = tx.clone();

        // Each thread will send its id via the channel
        let child = thread::spawn(move || {
            // Thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            thread_tx.send(id).unwrap();

            // Sending is non-blocking operation, thread will continue immediately after sending its message
            println!("thread {} finished", id);
        });

        children.push(child);
    }

    // All messages are collected
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // `recv` method picks message from channel
        // `recv` will block current thread if there are no messages available
        ids.push(rx.recv());
    }

    // Wait for threads to complete any remaining work
    for child in children {
        child.join().expect("oops! child thread panicked");
    }

    // Show order in which messages were sent
    println!("{:?}", ids);
}