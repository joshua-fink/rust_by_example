// For pointers, difference between destructuring and dereferencing
// Dereferencing uses *
// Destructuring uses &, ref, ref mut

fn main() {
    // Assign reference of type `i32`, `&` signifies reference being assigned
    let reference = &4;

    match reference {
        // If reference is pattern matched against val, it results
        // In comparison like: `&i32` and `&val`
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // Avoid the `&`, dereference before matching
    match *reference {
        val => println!("Got value by dereferencing: {:?}", val),
    }

    // Not a reference because right side is not one
    let _not_a_reference = 3;

    // Use ref to modify assignment so reference is created for given element
    let ref _is_a_reference = 3;

    // You can define 2 values without references
    // References can be retrieved via `ref` and `ref mut`
    let value = 5;
    let mut mut_value = 6;

    // ref keyword creates a reference
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // ref mut creates a reference
    match mut_value {
        ref mut m => {
            // Have reference, gotta dereference before any addition
            *m += 10;
            println!("We added 10. mut_value: {:?}", m);
        },
    }
}   