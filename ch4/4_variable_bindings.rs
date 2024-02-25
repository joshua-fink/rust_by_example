// Rust provides type safety via static typings
// Variable bindings can be type annotated when declared
// In most cases, compiler will be able to infer type of variable from context, reducing annotation burden

// Values like literals 

fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy an_integer into copied integer
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // Compiler warns about unused variable binding, underscore silences them
    let _unused_variable = 3u32;

    // NOISY one
    let noisy_unused_variable = 2u32;
}