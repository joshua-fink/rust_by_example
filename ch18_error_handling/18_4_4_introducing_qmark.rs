// Introducing ?

// Sometimes we want simplicity of unwrap without possibility of panic
// ? avoids deep nesting behavior with unwrap

// Upon finding an `Err`, two valid actions to take
// - `panic!` -> which we already decided to try to avoid if possible
// - return because Err means cna't be handled

// ? is almost equal to unwrap which returns instead of panic king on Errs.

use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    // try! macro had same functionality as ? in old code
    // Ex. let first_number = try!(first_number_str.parse::<i32>());
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}