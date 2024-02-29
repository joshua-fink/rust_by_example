// Configuration conditional checks use two operators:
// - `cfg` attribute -> #[cfg(...)] -> enables conditional compilation
// - `cfg!` macro -> allows checks at run-time -> does not remove code, only evals to T/F

// TWO FUNCTIONS WITH SAME NAME, but only ONE gets compiled to cfg attribute
// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running Linux!");
}

// Function only compiled if target OS not Linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running Linux!");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    // Macro simply returns T/F
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely Linux!");
    } else {
        println!("Yes. It's definitely NOT Linux!")
    }
}