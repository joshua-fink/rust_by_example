// tuples can be destructured in a match like this...
fn main() {
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);

    match triple {
        // Destructure second and third elements
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1`, rest doesn't matter"),
        (.., 2) => println!("Last is `2` and the rest doesn't matter"),
        (3, .., 4) => println!("First is `3`, last is `4`, rest does not matter"),
        // `_` don't bind value to variable
        _ => println!("It does not matter what they are"),
    }
}