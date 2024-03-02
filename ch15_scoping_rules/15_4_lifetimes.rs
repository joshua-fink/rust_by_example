// Lifetime is construct compiler (its borrow checker) uses to ensure all borrows valid
// Var's lifetime begins at creation, ends at destruction

// Lifetime is different concept than scoping
// Borrow has a lifetime determined by where it is declared
// Scope of borrow is determined by where reference is actually used

// In example, `i` has longest lifetime -> scope entirely encloses borrow1 and borrow2
// Duration of `borrow1` and `borrow2` irrelevant due to disjoint nature

fn main() {
    let i = 3; // Lifetime for `i` starts
    {
        let borrow1 = &i; // `borrow1` lifetime starts
        println!("borrow1: {}", borrow1);
    } // `borrow1` ends

    {
        let borrow2 = &i; // `borrow2` lifetime starts
        println!("borrow2: {}", borrow2);
    } // `borrow2` ends

}   // Lifetime for i ends