// Rc

// Multiple ownership is needed, Rc (Reference Counting) can be used
// Rc -> keeps track of the number of references which means number of owners of value wrapped inside Rc

// Rc increases by 1 whenever Rc is clone, decreases by 1 whenever cloned Rc drops out of scope
// When Rc's count goes to zero, value and Rc are dropped

// Cloning Rc never performs deep copy, just another pointer to wrapped value plus increments count

use std::rc::Rc;

fn main() {
    let rc_examples = "Rc examples".to_string();
    {
        println!("--- rc_a is created ---");

        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
    
        {
            println!("--- rc_a is cloned to rc_b ---");

            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

            // Two Rc's are equal if their inner values are equal
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

            // Can use methods of value directly
            println!("Length of value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);

            println!("--- rc_b dropped out of scope ---");
        }

        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        println!("--- rc_a dropped out of scope ---");
    }

    // Error! `rc_examples` already moved into `rc_a`
    // When rc_a is dropped, rc_examples dropped together
    // println! ("rc_examples: {}", rc_examples);
}