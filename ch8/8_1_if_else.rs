fn main() {
    let n = 11;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is postiive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n
        } else {
            println!(", and is a big number, halve the number");
            n / 2 // if you add semicolon... ERROR!!
        }; // semicolon is needed for let bindings

    println!("{} -> {}", n, big_n);
}