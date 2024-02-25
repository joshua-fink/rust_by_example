// convert any type into a string by implementing the ToString trait for the type
// implement via fmt::Display trait

use std::fmt;

struct Circle {
    radius: i32,
}

// ah to_string provided by fmt::Display
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle{ radius: 6 };
    println!("{}", circle.to_string());
}