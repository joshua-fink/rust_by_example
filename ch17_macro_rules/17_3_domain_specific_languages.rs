// DSL is mini language embedded in Rust macro
// Completely valid Rust because macro expands into normal Rust constructs, but looks like small language
// Allows you to define concise or intuitive syntax for special functionality (within bounds)

// Suppose I want to define calculator API. Would like to supply expression and have output printed to console
macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e; // Force types to be unsigned integers
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

fn main() {
    calculate! {
        eval 1 + 2
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}