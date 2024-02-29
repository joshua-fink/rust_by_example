// Crate -> compilation unit in Rust
// rustc FILENAME, FILENAME treated as crate file
// If FILENAME has mod declarations -> contents of module files would be inserted BEFORE running compiler
// Modules do not get compiled individually, only crates do

// Crates can be compiled into binaries or libraries
// rustc can produce a binary or library -> default is binary, override via --crate-type flag

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

/* COMMANDS
rustc --crate-type=lib 11_1_creating_a_library.rs
*/