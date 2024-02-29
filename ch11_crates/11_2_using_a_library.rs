// Link crate use rustc's `--extern` flag
// All of its items will then be imported under module named the same as library
// This module generally behaves same as any other module

// 2015 or earlier -> extern crate 11_1_creating_a_library;

fn main() {
    lib11_1_creating_a_library::public_function();

    // 11_1_creating_a_library::private_function(); ERROR! because private function private 

    lib11_1_creating_a_library::indirect_access();
}

// COMMAND
// rustc 11_2_using_a_library.rs --extern lib11_1_creating_a_library=lib11_1_creating_a_library.rlib && ./11_2_using_a_library