// `trait` that is generic over its container type has type specification requirements
// Users of the trait must specify all of its generic types

// In example below, `Contains` `trait` allows use of generics `A` and `B`
// Trait then implemented for `Container` type, specifying `i32` for `A` and `B` for use with `fn difference()`

// Because `Contains` is generic -> must explicitly state all generic types for `fn difference()`
struct Container(i32, i32);

// A trait which checks if 2 items are stored inside container
// Also retrieves first or last value
trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool; // Explicitly requires `A` and `B`
    fn first(&self) -> i32; // Doesn't explicitly require `A` or `B`
    fn last(&self) -> i32; // Doesn't explicitly require `A` or `B`
}

impl Contains<i32, i32> for Container {
    // True if stored nums are equal
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // Grab first number
    fn first(&self) -> i32 { self.0 }

    // Grab last number
    fn last(&self) -> i32 { self.1 }
}

// `C` contains `A` and `B`. Given that, having to express that again is annoying
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2),    
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}