fn main() {
    // A counter variable
    let mut n = 1;

    // Loop while n < 101
    while n < 101 { 
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }
}