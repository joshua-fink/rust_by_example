// Option

// Sometimes it's desirable to catch failure of some parts of program instead of calling `panic!`
// Solution is Option enum

// `Option<T>` has two variants -> `None` to indicate failure or lack of value, `Some(value)` tuple that wraps value of type T

// Integer division that doesn't panic
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // Failure represented as `None` variant
        None
    } else {
        // Result is wrapped in `Some` variant
        Some(dividend / divisor)
    }
}

// Function handles division that may not succeed
fn try_division(dividend: i32, divisor: i32) {
    // `Option` values can be pattern matched, just like other enums
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    // Binding `None` to variable needs to be type annotated
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    // Unwrapping a `Some` variant will extract value wrapped
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

    // Unwrapping a `None` variant will `panic!`
    println!("{:?} unwraps to {:?}", none, none.unwrap());
}