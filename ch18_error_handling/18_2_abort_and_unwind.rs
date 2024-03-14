// Previous section illustrates error handling mechanism panic
// Different code paths are conditionally compiled based on panic setting
// Current values available are unwind and abort

// Builds on 18_1
/*
fn drink(beverage: &str) {
    // You shouldn't drink too much sugar
    if beverage == "lemonade" {
        if cfg!(panic = "abort") {
            println!("This is not your party. Run!!!");
        } else {
            println!("Spit it out!!!");
        }
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}
*/

// Version 2 focusing on rewriting `drink()` and explicitly using `unwind` keyword

#[cfg(panic = "unwind")]
fn ah() {
    println!("Spit it out!!!")
}

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("This is not your party. Run!!");
}

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        ah();
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}


fn main() {
    drink("water");
    drink("lemonade");
}

