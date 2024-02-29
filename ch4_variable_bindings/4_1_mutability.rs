// Variable bindings immutable by default, but can be overridden with mut modifier

fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error: can't assign new value to immutable variable
    // _immutable_binding += 1;
}