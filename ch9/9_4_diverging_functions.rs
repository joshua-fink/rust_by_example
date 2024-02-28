// Diverging functions never return
// Marked with `!`, which is empty type

fn foo() -> ! {
    panic!("This call never returns.");
}

// Opposed to all other types, this one cannot be instantiated
// Set of all possible values this type can have is empty
// Note: different from () type -> has one val

fn some_fn() {
    ()
}

fn main() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Return type of match expression must be u32
            // because of type of "addition" variable
            let addition: u32 = match i%2 == 1 {
                // i var is type u32, OK
                true => i,
                // Other hand, continue expression does not return
                // u32, it is still fine, because it never returns and
                // therefore does not violate type reqs of match expression
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));

    // let x: ! = panic!("This call never returns.");
    // println!("You will never this line!");
}