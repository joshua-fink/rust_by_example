// Most of the time, we'd like to access without ownership transfer
// Rust uses *borrowing* mechanism to accomplish this
// Instead of passing objects by value (T), objects can be passed by reference (&T)

// Compiler statically guarantees (via borrow checker) that references always point to valid objects
// While references to object exist, cannot be destroyed

// This function takes ownership of box and destroys it
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// Borrows an i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    // Create boxed i32 in heap and i32 on stack
    // Numbers can have underscores added for readability
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;


    // Borrow contents of box, ownership not taken, so contents can be borrowed again
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Take reference to data contained inside box
        let _ref_to_i32: &i32 = &boxed_i32;

        // Error! Cannot destroy `boxed_i32` while inner value borrowed later in scope
        // eat_box_i32(boxed_i32);
        
        // Attempt to borrow `_ref_to_i32` after inner value destroyed
        borrow_i32(_ref_to_i32);

        // `ref_to_i32` goes out of scope, no longer borrowed
    }

    // `boxed_i32` can now give up ownership to `eat_box` and be destroyed
    eat_box_i32(boxed_i32);
}