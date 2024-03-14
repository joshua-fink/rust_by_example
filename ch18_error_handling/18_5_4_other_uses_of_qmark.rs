// Other uses of ?

// `?` means `unwrap` or `return Err(From::from(err))`
// `From::from` is conversion utility between different types
// Therefore, if you `?` where error is convertible to return type, will return automatically

use std::error;
use std::fmt;

// Change alias to use `Box<dyn error::Error>`
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

// Same structure as before but rather than chain all `Results` and `Options` along, we `?` to get inner value out immediately
fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result{
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

// Compared to `panic` much cleaner -> very similar to replacing `unwrap` calls with `?` except return types are `Result`
// Must be destructured at top level