// Use of "associated types" improves overall readability of the code moving inner types
// locally into trait as output types

// Syntax for `trait` defintion is as follows

// `A` and `B` are defined in trait via `type` keyword
// Not the same as aliases
/*
trait Contains {
    type A;
    type B;

    // Updated syntax to refer to new types generically
    fn contains(&self, _, &Self::A, _: &Self::B) -> bool;
}
*/

// functions that use `Contains` now no longer need to express `A` or `B`
/* 
// Without...
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A,B> { ... }

// Using associated types
fn difference<C: Contains>(container: &C) -> i32 }{ ... }
*/

// Rewritten from 14_8_1
struct Container(i32, i32);

// Trait which checks if 2 items are stored inside container
// Retrieves first/last value
trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // Specify what types A and B are
    // If input is Container(i32, i32) -> output types are determined as i32, i32
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) -> i32 { self.0 }
    fn last(&self) -> i32 { self.1 }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}