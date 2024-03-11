// Macros can use + in argument list to indicate that arg may repeat at least once, or *
// to indicate argument may repeat zero or more times

// Following example, surrounding matcher with `$(...),+` will match one or more
// expression, separated by commas.
// Note semicolon is optional on last case

// `find_min!` will calculate the minimum of any number of args
macro_rules! find_min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y`
    ($x:expr, $($y:expr), +) => (
        // Call `find_min!` on tail `y`
        std::cmp::min($x, find_min!($($y), +))
    )
}

fn main() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1+2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}