// aliases for Result

// when we want to reuse specific `Result` type many times, use aliases

// Aliases are helpful especially at module level
// Errors found in specific module often have same `Err` type, so single alias can succintly define all associated `Results`
// std library even has one

use std::num::ParseIntError;

// Define generic alias for `Result` with error type `ParseIntError`
type AliasedResult<T> = Result<T, ParseIntError>;

// Use above alias to refer to specific `Result` type
fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number|{
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

// Alias again saves space
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
