// closures are functions that capture the enclosing environment
// very convenient for on the fly usage

// calling a closure is like calling a function
// input and return types can be inferred
// input variable names must be specified

// Characteristics
// Use || instead of () around input vars
// Optional body delimitation {} for single line expression
// Ability to capture outer env variables

fn main() {
    let outer_var = 42;

    // A regular function cannot refer to variables in enclosing environment
    // Error! regular function cannot refer to var outside function scope
    // fn function(i: i32) -> i32 { i + outer_var }

    // Closures are anonymous, here we are binding them references
    // Annotation is identical to functio annotation but is optional
    // as are the `{}` wrapping the body. these nameless functions
    // are assigned to appropriately named variables
    let closure_annotated = |i: i32| -> i32 {i + outer_var };
    let closure_inferred = |i| i + outer_var;

    // Call the closures
    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));

    // Once closure's type has been inferred, it cannot be inferred again with another type
    // println!("cannot reuse closure_inferred with another type: {}", closure_inferred(42i64));
    // above line causes error

    // A closure taking no arguments returns i32
    // return type is inferred
    let one = || 1;
    println!("closure returning one: {}", one());
}

