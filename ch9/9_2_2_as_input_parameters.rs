// Rust chooses how to capture vars on the fly without type annotation
// Ambiguity not allowed when writing functions
// When taking closure as input parameter, closure's complete type needs to be annotated
// using one of a few `traits` -> determined by what closure does with captured value
// `trait` annotated in order of decreasing restriction
// Fn -> Closure uses captured value by reference (&T)
// FnMut -> Closure uses captured value by mutable reference (&mut T)
// FnOnce -> Closure uses captured value by value (T)

// On var-by-var basis, compiler captures vars in least restrictive manner possible

// Consider parameter annotated as FnOnce. Specifies closure may capture by &T, &mut T, or T
// Compiler will ultimately choose based on how captured variables are used in the closure
// Note, reverse not possible, if param is annotated as Fn, cannot capture &mut T or T

// A function which takes closure as an argument and calls it
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F) where
    // Closure takes no input, returns nothing
    F: FnOnce() {
    f();
}

fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`
    F: Fn(i32) -> i32 {
    f(3)
}

fn main() {
    use std::mem;
    
    let greeting = "hello";

    // A non-copy type
    //`to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value
    let diary = || {
        // `greeting` is by reference: requires `Fn`
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`
        mem::drop(farewell);
    };

    // Call the function which applies the closure
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}