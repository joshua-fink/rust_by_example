// Rust program is made up of series of statements
// Most common two statements are declaring a variable binding, using ; with an expression

fn main() {
    // variable binding
    let x = 5;

    // expression;
    x;
    x + 1;
    15;

    let x = 5u32;

    // blocks are expressions too, can be used as values in assignments
    // last expression in block will be assigned to place expression such as local var
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}