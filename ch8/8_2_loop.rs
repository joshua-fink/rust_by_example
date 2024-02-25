// Rust provides loop keyword for infinite loop
// break statement -> exit loop anytime
// continue statement -> skip rest of iteration, start new one

fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            continue; // skip rest of iteration
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }
}