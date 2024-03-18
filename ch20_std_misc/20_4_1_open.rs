// open

// open function can be used to open file in read-only mode
// `File` owns resource, file descriptor, takes care of closing file when `drop`ed

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create path to desired file
    let path = Path::new("hello.txt");
    let display = path.display();

    // Open path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read file contents into string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` goes out of scope, "hello.txt" is closed
}