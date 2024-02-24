// fmt::Debug isn't most compact and clean
// Solution is to customize the input appearance
use std::fmt;

// Define a structure for which `fmt::Display` will be implemented
// Tuple struct named `Structure` that contains an `i32`
struct Structure(i32);

// To use the `{}` marker, the trait `fmt::Display` must be
// implemented manually for type
// Any new container type not generic must use this for display purposes
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. returns `fmt::Result` which indicted whether
        // op succeded or failed. write! uses syntax like `println!`
        write!(f, "{}", self.0)
    }
}

fn main() {}