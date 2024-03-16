// Arc (Atomically Reference Counted)

// When shared ownership between threads is needed, Arc can be used
// This struct, via Clone implementation can create reference pointer for location of value in memory heap while increasing reference counter
// As it shares ownership between threads, when last reference pointer is out of scope, variable is dropped

use std::time::Duration;
use std::sync::Arc;
use std::thread;

fn main() {
    // This variable declaration is where value is specified
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        // Here no value specification as it is pointer to a reference in memory heap
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            // As Arc was used, threads can be spawned using value allocated in Arc variable pointer's location
            println!("{:?}", apple);
        });
    }

    // Make sure all Arc instances are printed from spawned threads
    thread::sleep(Duration::from_secs(1));
}