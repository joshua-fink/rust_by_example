// Closures succintly capture variables from enclosing scopes
// Using closure as a function parameter requires generics
// This needed because how they are defined

// `F` must be generic
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

// when closure defined, compiler implicitly creates new anonymous structure
// to store captured vars inside, implementing the functionality via one of the
// traits: Fn, FnMut, or FnOnce for this unknown type
// Type is assigned to the variable which is stored until calling

// Since new type is of unknown type, any usage in function will require genetics
// An unbounded type parameter <T> would still be ambiguous and not be allowed
// Bounding by one of the traits: Fn, FnMut, FnOnce is suffiient to specify its type

fn main() {
    let x = 7;

    // Capture `x` into anonymous type and implement
    // `Fn` for it. Store it in `print`
    let print = || println!("{}", x);

    apply(print);
}