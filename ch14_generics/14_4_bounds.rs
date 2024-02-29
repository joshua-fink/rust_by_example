// When working with generics, type params often must use traits as bounds
// to stipulate what functionaliy type implements

// Define function `printer` that takes generic type `T` which
// must implement trait `Display`
// In other words, requires T to be bound by Display, T must implement Display
/*
fn printer<T: Display>(t: T) {
    println!("{}", t);
}
*/

// Bounding restricts generic to types that conform to bounds
/*
struct S<T: Display>(T);

// Error! `Vec<T>` does not implement `Display`
// Specialization will fail
let s = S(vec![1]);
*/

// Another effect of bounding -> generic instances allowed to access methods
// of traits specified in bounds

// Trait which implements print marker `{:?}`
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }

#[allow(dead_code)]
struct Triangle { length: f64, height: f64 }

// Generic `T` must implement `Debug`
// Regardless of type, this works properly
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` must implement `HasArea`. Any type which meets
// the bound can access `HasArea`'s function `area`
fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    // print_debug(&_triangle);
    // println!("Area: {}", area(&_triangle));
    // ^ These two lines cause error because Triangle does not implement `Debug` or `HasArea`
}
