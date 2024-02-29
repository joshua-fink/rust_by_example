// `crate_type` attribute can be used to tell compiler whether crate is a binary or library
// `crate_name` attribute can be used to set the name of the crate

// These attributes have NO EFFECT WHEN USING CARGO

// This crate is a library
#![crate_type = "lib"]
// Library named "rary"
#![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's `public function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

// No longer need to pass --crate-type flag to rustc
// rustc 13_2_crates.rs