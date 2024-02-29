// Common way to implement linked-list via enums...

use crate::List::*;

enum List {
    // Cons: Tuple struct that wraps element and pointer to next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies end of linked list
    Nil,
}

// Methods that can be attached to enum
impl List {
    // Create empty list
    fn new() -> List {
        Nil // Nil has type list
    }

    // Consume a list, return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        //Cons also has type List
        Cons(elem, Box::new(self))
    }

    // return length of list
    fn len(&self) -> u32 {
        // self has to be matched, because behavior of method
        // depends on the variant of self
        // self has type &List and *self has type List
        // Matching on concrete type T is preferred over match on reference &T
        match *self {
            // Can't take ownership of tail, because self is borrowed
            // Instead take reference to tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: Empty list has zero length
            Nil => 0
        }
    }

    // return representation of list as (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // format! is similar to print! but returns heap
                // allocated string instead of printing to console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // Create empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show final state of list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}