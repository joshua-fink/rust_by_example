// Some conditionals like `target_os` implicitly provided by rustc
// Custom conditionals must be passed to `rustc` via `--cfg` flag

#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
}

// Must using this compilation command for it to work
// rustc --cfg some_condition 13_4_custom.rs && ./13_4_custom