// from trait allows for type to define how to create itself from another type

/*
let my_str = "hello";
let my_string = String::from(my_str)
*/

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}
