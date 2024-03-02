// Borrow checker uses explicit lifetime annotations to determine how long references should be valid
// Where lifetimes are not ELIDED?, Rust requires explicit annotations to determine what lifetime of reference should be
// Syntax for explicitly annotating lifetime is '

// Example: foo<'a> -> foo has lifetime parameter `'a`

// Like closures, using lifetimes requires generics
// Lifetime syntax indicates lifetime of foo may not exceed that of 'a
// Explicit annotation of type has form &'a T where 'a already introduced

// Can use multiple lifetimes
// foo<'a, 'b> -> foo has lifetime params `'a` and `'b'` -> foo cannot exceed either 'a or 'b

// `print_refs` takes two references to `i32` which have different lifetimes `a` and `b`
// These two lifetimes must both be at least as long as function `print_refs`
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// A function takes no args, but has lifetime parameter 'a
fn failed_borrow<'a>() {
    let _x = 12;

    // Error! `_x` does not live long enough
    // let _y: &'a i32 = &_x;
    // Attempting to use lifetime `'a` as explicit type annotation
    // inside function will fail because lifetime of `&x` is shorter
    // than that of `_y`. Short lifetime can't be coerced into longer one
}

fn main() {
    // Create variables to be borrowed below
    let (four, nine) = (4, 9);

    // Borrows (`&`) of both variables passed into function
    print_refs(&four, &nine);
    // Any input which is borrowed must outlive borrower
    // Other words, lifetime of `four` and `nine` must be longer than that of `print_refs`

    failed_borrow();
    // `failed_borrow` contains no refs to force `'a` to be
    // longer than lifetime of function, but `'a` is longer
    // lifetime never constrained, defaults to `'static`
}