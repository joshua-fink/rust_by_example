// Variadic inferface takes in arbitrary number of args
// Example println!

// Extension of calculate! macro from 17_3 to be variadic
macro_rules! calculate {
    // Pattern for single `eval`
    (eval $e:expr) => {
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    };

    (eval $e:expr, $(eval $es:expr), +) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};
}

fn main() {
    calculate! {
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}