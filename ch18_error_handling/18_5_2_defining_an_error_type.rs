// Defining an error type

// Sometimes simplier code-wise to mask all different error with single type of error -> create custom error

// "Good" error type
// - Represents different error with same type
// - Presents nice error messages to user
// - Easy to compare with other types
// -- Good -> Err(EmptyVec)
// -- Bad -> Err("Pleases use vec with at least one element".to_owned())

// - Can hold info about error
// -- Good: Err(BadChar(c, position))
// -- Bad: Err("+ cannot be used here".to_owned())

// - Composes well with other errors

use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

// Define error types, can be customized for error handling cases
// Now able to write own errors, defer to underlying error, or do something in between
#[derive(Debug, Clone)]
struct DoubleError;

// Generation of error is completely separate from how it is displayed
// No need to be concerned about cluttering complex logic with display style
// We don't store any extra info about error. Cannot state which string failed to parse without futher modifying our types

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // Change error to new type
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                // Update to new error type here also
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}