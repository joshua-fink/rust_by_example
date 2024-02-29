// Bound can also be expressed using `where` clause immediately before opening `{`
// rather than at type's first mention

// Where clauses can apply bounds to arbitrary types, rather than type params

// Cases where clause is useful
// (1) When specifying generic types and bounds separately is clear
/* 
impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// Expressing bounds with `where` clause
impl <A, D> MyTrait<A, D> for YourType where
    A: TraitB + TraitC,
    D: TraitE + TraitF {}
*/

// (2) When using `where` clause is more expressive than normal syntax
// `impl` in following example cannot be directly expressed without `where` clause
use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// Avoid expressing this as `T:Debug` or other method of indirect approach...
// Use `where` clause instead
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // Want `Option<T>: Debug` as bound because that's what's being printed
    // Otherwise would be using wrong bound
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
