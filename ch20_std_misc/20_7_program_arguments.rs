// Program arguments

// Standard library
// Access command line args via `std::env::args`, which returns iterator that yields `String` for each argument

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // First argument is path that was used to call program
    println!("My path is {}.", args[0]);

    // Rest of args are passed command line parameters
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..])
}

// Crates
// Numerous packages that offer extra CLI functionality, such as `clap`