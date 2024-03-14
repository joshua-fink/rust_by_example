// Iterating over `Result`s

// If `Iter::map` operation might fail, for example

/*
This fails...
fn main() {
    let string = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}
*/

// Ignore failed items with filter_map()
// filter_map -> calls function, filters out results that are `None`

/*
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok()) // different from above
        .collect();
    println!("Results: {:?}", numbers);
}
*/

// Collect failed items with `map_err()` and `filter_map()`
// `map_err` calls a function with error, by adding that to previous `filter_map` can save them off to side while iterating
/*
fn main() {
    let strings = vec!["42", "tofu", "93", "999", "18"];
    let mut errors = vec![];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<u8>())
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
*/

// Fail entire operation with `collect()`
// `Result` implements `FromIterator` so that vector of results (Vec<Result<T, E>>) can be turned into result
// With vector (`Result<Vec<T>, E>`)
// When `Result::Err` is found, iteration terminates
// Same technique can be used with `Option`

/*
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}
*/

// Collect all valid values and failures with `partition()`
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    // All the results are still wrapped in Result, add more boilerplate
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
