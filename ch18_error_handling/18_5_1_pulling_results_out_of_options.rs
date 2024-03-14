// Pulling Results out of Options

// Most basic way of handling error types is just embed them in each other

use std::num::ParseIntError;

/*
fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}
*/

// For time when stop processing on errors but keep going when option is None, do this:
fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });

    opt.map_or(Ok(None), |r| r.map(Some))
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings  = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));
    println!("The first doubled is {:?}", double_first(empty)); // Err 1: empty input vec
    println!("The first doubled is {:?}", double_first(strings)); // Err 2: element does not parse to number
}