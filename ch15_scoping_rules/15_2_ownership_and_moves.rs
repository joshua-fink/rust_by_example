// Because variables are in charge of freeing own resources...
// Resources can only have ONE owner
// Prevents resources from being freed more than once
// Not all variables own resources -> references

// Doing assignments (let x = y) or passing function args by value (foo(x))
// Ownership of resources in those cases is transferred -> "Rust" speak, "move"

// After "move", prev owner no longer can be used, avoiding dangling pointers

// This function takes ownership of heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("Destroying box that contains {}", c);
}

fn main() {
    // _Stack_ allocated integer
    let x = 5u32;

    // *Copy* `x` into `y` -> no resources moved
    let y = x;
    
    // Both values independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    let b = a;

    // Pointer address of `a` is copied (not data) into `b`
    // Both pointers to same heap allocated data, but `b` now owns it

    // Error! `a` can no longer access data, no longer owns heap memory
    // println!("a contains: {}", a);

    // this function takes ownership of heap allocated memory from `b`
    destroy_box(b);

    // Heap memory freed at this point, action would result
    // in dereferencing freed memory, but is forbidden my compiler
    // println!("b contains: {}", b); <- Causes same error
}