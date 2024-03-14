// `Result` is richer version of `Option` type that describes possible error instead of possible absence

// `Result<T, E> could have one of two outcomes...
// - `Ok(T)` -> elt T was found
// - `Err(E)` -> error found with element E

// Convention: `Ok` is expected, `Err` is not

// Like `Option`, `Result` has many methods
// `unwrap()` eithers yields element T or panics, many aspects of case handling also overlap
// `parse()` returns a `Result` type. 

/*
fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt); // this generates error for unwrap to panic on
}
*/

// Using `Result` in `main()`
// `Result` type can also be return type of main function if specified explicitly

// If error occurs within `main` function it will return error code and print debug representation of error (using Debug trait)

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
