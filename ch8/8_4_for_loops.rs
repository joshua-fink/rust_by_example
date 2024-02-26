// for loops
fn main() {

    // `for in` construct can be used to iterate through an `Iterator`
    
    // One of the easiest ways to create `Iterator` is to use the range notation `a..b`
    // Yields values from `a` (inclusive) to `b` (exclusive)
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // `a..=b` means `a` to `b` (inclusive, including `b`)
    // same as previous for loop
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // Here is how you create an iterator out of a vector


    let names = vec!["Bob", "Frank", "Ferris"];
    
    // This BORROWS each element of the collection through each iteration
    // Leaves the collection untouched and available for reuse after loop
    println!("`.iter()`");

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);
    println!("");

    println!("`.into_iter()`");
    let names = vec!["Bob", "Frank", "Ferris"];
    // This CONSUMES the collection so that on each iteration the exact data is provided
    // Once consumed it is NO LONGER AVAILABLE FOR REUSE as it has been moved within the loop
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // println!("names: {:?}", names);
    println!("");

    println!("`.iter_mut()`");
    // This MUTABLY BORROWS each element of the collection
    // Allows for collection to be modified in place
    
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}