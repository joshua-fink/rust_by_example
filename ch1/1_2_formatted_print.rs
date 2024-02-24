// Printing handled by macros in std::fmt

/*
format! -> write formatted text to String
print! -> same as format! but text is printed to console (io::stdout)
println! -> same as print! + appended newline
eprint! -> same as print! but text printed to standard error (io::stderr)
eprintln! -> same as eprint! but a newline is appended

All parse text the same way, Rust checks for formatting correctness at compile time
*/

fn main() {
    // Generally '{}' is automatically replaced with any args in stringified manner
    println!("{} days", 31);

    // Positional args -> num inside '{}' determines which elt goes inside
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Args can be named
    println!(
        "{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over"
    );

    // Different formatting can used by doing : + format char
    println!("Base 10:               {}", 69420);
    println!("Base 2 (binary):       {:b}", 69420);
    println!("Base 8 (octal):        {:o}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);
    println!("Base 16 (HEXADECIMAL): {:X}", 69420);

    // You can right-justify text with specified width.
    println!("{number:>5}", number=1);

    // Pad nums with zeroes
    println!("{number:0>5}", number=1);
    
    // Left adjust by flipping sign 
    println!("{number:0<5}", number=1);

    // Named arguments in the format specifier by appending a '$'.
    println!("{number:0>width$}", number=1, width=5);

    // Rust checks to ensure proper amount of args used
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Only types that implement fmt::Display can be formatted with `{}`.
    // User-defined types do not implement fmt::Display by default.

    //#[allow(dead_code)] // disable `dead_code` which warn against unused module
    // struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}