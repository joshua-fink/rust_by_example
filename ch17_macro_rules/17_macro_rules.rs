// Rust macro system allows metaprogramming
// Macros look like functions, except name ends with bang !
// Instead of generating function call, macros expanded into source code that gets compiled with rest of program
// Unlike macros in C and other languages, Rust macros expanded into abstract syntax trees rather than string preprocessing
// Avoids unexpected precedence bugs

// Macros created using macro_rules! macro

// Simple macro named `say_hello`
macro_rules! say_hello {
    // `()` indicates that macro takes no argument
    () => {
        // Macro will expand into contents of this block
        println!("Hello!");
    };
}

fn main() {
    // Call will expand into `println!("Hello")`
    say_hello!()
}

// Why macros useful?
// 1) DON'T REPEAT YOURSELF -> Many cases where you need similar functionality in multiple places but with different types
// Helpful way to avoid repeating code

// 2) DOMAIN-SPECIFIC LANGUAGES -> Macros allow you to define special syntax for specific purpose

// 3) VARIADIC INTERFACES -> Sometimes you want to define interface with variable number of arguments
// For example, println!, any number depending on format string