// Into is basically reciprocal of the from trait.

use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}

fn main() {
    let int = 5;

    // Type annotation necessary so compiler knows type needed
    let num: Number = int.into();
    println!("My number is {:?}", num);
}