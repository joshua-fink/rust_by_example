// panic! Macro

// Macro is used to start unwinding stack upon generation of panic
// While unwinding, runtime will take care of freeing all resources owned by the thread by calling the destructor of all of its objects

// Since we are dealing with programs with only one thread, panic! will cause the program to report the panic message and exit

// Re-implementation of integer division
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // Division by zero triggers a panic
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

// Main task
fn main() {
    // Heap allocated integer

    let _x = Box::new(0i32);

    // This operation will trigger task failure
    division(3,0);

    println!("This point will not be reached!");

    // _x will get destroyed here
}