// Function signatures with lifetimes have few constraints
// - any reference must have annotated lifetime
// - any reference returned must have same lifetime as input or be static

// Note that returning references without input is banned would result in returning references to invalid data

// One input reference with lifetime `'a` which live at least as long as the function
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// Mutable references possible with lifetimes as well
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Multiple elements with different lifetimes. This case, it would
// be fine for both to have same lifetime `'a`, but
// in more complex cases, different lifetimes may be required
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// Returning references that have been passed in is acceptable
// However, correct lifetime must be returned
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

// fn invalid_output<'a>() -> &'a String { &String::from("foo") }
// Above invalid because '`a`' must live longer than function
// Here, `&String::from("foo")` would create a `String`, followed by reference
// Data dropped upon exiting scope, leaving reference to invalid data to be returned

fn main() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}