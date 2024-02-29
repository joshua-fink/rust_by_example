// Bounds allow you to use traits as bounds, even if it doesn't have any functionality
// Examples: `Eq` and `Copy` are traits in `std` library

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// These functions only valid for types that implement these traits
// Fact that traits empty is irrelevant
fn red<T: Red>(_: &T) -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;
    
    // Bounds restrict where functions work
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));

    // This causes error because Turkey does not implement Red trait
    // println!("A turkey is {}", red(&_turkey));
}