fn main() {
    let number = 20;

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        // Match an inclusive range
        14..=19 => println!("A teen"),
        // Rest of cases
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // Match is an expression
    let binary = match boolean {
        // Arms of match must cover all possible values
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}