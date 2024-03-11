// Macros can be overloaded to accept different combos of arguments
// Works similarly to match block

// `test!` will compare `$left` and `$right`
// in different ways depending on invocation
macro_rules! test {
    // Args don't need comma separation, any template can used!
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
                stringify!($left),
                stringify!($right),
                $left && $right)
    };
    // Each arm must end with semicolon
    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}",
                stringify!($left),
                stringify!($right),
                $left || $right)
    };
}

fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false)
}