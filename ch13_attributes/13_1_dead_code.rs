// Compiler provides a `dead_code` lint that will warn about unused functions
// Attribute can be used to disable the lint

fn used_function() {}

// Disables `dead_code`lint
#[allow(dead_code)]
fn unused_function() {}

#[allow(dead_code)]
fn noisy_unused_function() {}

fn main() {
    used_function();
}