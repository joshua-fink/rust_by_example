// Two types of consts...
// const -> unchangable val (most common)
// static -> possibly mutable value with static lifetime (inferred not have to be specified)
// Accessing or modifying mutable static variable is unsafe

// Globals declared outside all other scopes
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify const
    // THRESHOLD = 5;
}