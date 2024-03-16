// HashMap

// Vectors store values by integer index => `HashMap`s store values by key
// `HashMap` keys can be booleans integers, strings, or any type that implements Eq and Hash traits

// Like vectors, `HashMap`s are growable, but can shrink themselves when there is excess space

// Create HashMap with certain starting capacity `HashMap::with_capacity(uint)` or use `HashMap::new()` for default initial capacity

use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "We're sorry, the call cannot be completed as dialed. Please hang up and try again.",
        "645-7689" => "Hello, this is Mr. Awesome Pizza. Me llamo Fred. What can I get for you today?",
        _ => "Hi who is this?",
    }
}

fn main() {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // Takes reference and returns Option<&V>
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number."),
    }

    // `HashMap::insert()` returns `None`
    // if inserted value is new, `Some(value)` otherwise
    contacts.insert("Daniel", "164-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number."),
    }

    contacts.remove(&"Ashley");

    // `HashMap::iter()` returns iterator that yields (&'a key, &'a value) pairs in arbitrary order
    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number));
    }
}