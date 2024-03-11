// `Iterator` trait used to implement iterators over collections like arrays
// Trait requires only method to be defined for next element -> automatically defined or done with impl block

// For some common situations, `for` construct turns some collections into iterators with .into_iter() method

struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement `Iterator` for `Fibonacci`
// `Iterator` trait only requires method to be defined for next elt
impl Iterator for Fibonacci {
    // Refer to type using Self::Item
    type Item = u32;

    // Here, define sequence with `.curr` and `.next`
    // Return type is `Option<T>`:
    // - Iterator finished -> return `None`
    // - Else, next value wrapped in `Some` and returned
    // Use Self::Item in return type, change type without having to update function sigs
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        // No endpoint to Fibonacci seq, `Iterator` never returns None
        Some(current)
    }
}

// Returns Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 } 
}

fn main() {
    // `0..3` is Iterator that generates: 0, 1, 2
    let mut sequence = 0..3;
    
    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // `for` works thru `Iterator` until it returns `None`
    // Each `Some` value is unwrapped and bound to variable (here, `i`)
    println!("Iterate thru 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // The `take(n)` method reduces an `Iterator` to its first `n` terms
    println!("First four terms of Fibonacci sequence are:");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }
    
    // `skip(n)` method shortens `Iterator` dropping first `n` terms 
    println!("Next four terms are:");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // `iter` method produces `Iterator` over array/slice
    println!("Iterate following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}
