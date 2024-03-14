// Multiple error types

// Previous examples always been convenient, Results interact with other Results and Options interact with other Options

// Sometimes Option needs to interact with a Result, or Result<T, Error1> needs to interact with Result<T, Error2>

fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Generate error 1
    2 * first.parse::<i32>().unwrap() // Generate error 2
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));
    println!("The first doubled is {}", double_first(empty)); // Err 1: empty input vec
    println!("The first doubled is {}", double_first(strings)); // Err 2: element does not parse to number
}