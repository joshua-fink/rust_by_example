// read_lines

// Naive approach
/*
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
*/

// Since method lines() returns an iterator over the lines in the file, we can perform map inline and collect results, yielding more concise and fluent expression
/*
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split string into iterator of string slices
        .map(String::from) // make each slice into string
        .collect() // gather them together into vector
}
*/
// In both above examples, must convert &str reference returned from lines() to owned type `String` using `.to_string()` and `String::from` respectively

// MORE EFFICIENT APPROACH
// Pass ownership of open `File` to `BufReader` struct, `BufReader` uses internal buffer to reduce intermediate allocations
// Can update `read_lines` to return iterator instead of allocating new String objects in memory for each line

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./hosts.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}