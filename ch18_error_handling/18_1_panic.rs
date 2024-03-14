// Simplest error handling mechanism is `panic`
// Prints error message, starts unwinding stack, usually exits program

// Explicit call to panic here
fn drink(beverage: &str) {
    if beverage == "lemonade" { panic!("AAAAaaaaa!!!!"); }

    println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
    drink("water");
    drink("lemonade");
    drink("still water");
}

// First call to drink works, second panics so third not called