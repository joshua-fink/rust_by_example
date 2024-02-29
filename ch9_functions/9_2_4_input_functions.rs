// Since closures can be used as args, you might wonder if same
// can be said about functions. If you declare function that takes as a parameter
// then any function that satisfies the trait bound that closure can be passed as a parameter

// Define a function which takes a generic `F` argument
// bounded by `Fn` and calls it
fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("I'm a function!");
}

fn main() {
    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}
