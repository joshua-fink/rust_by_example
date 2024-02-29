// indirectly accessing a variable makes it impossible to branch
// and use variable without rebinding. match provides @ sigil for
// binding values to names

fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("Haven't celebrated first bday yet"),
        // can match 1..=12 directly, but would not able to report age
        n @ 1 ..=12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound, return result
        n => println!("I'm an old person of age {:?}", n),
    }

    match some_number() {
        // Got Some variant, match if its value, bound to n
        // is equal to 42
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }
}