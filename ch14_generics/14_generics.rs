// Generics -> topic of generalizing types and functionalities to broader cases
// Helpful for reducing code duplication
// However syntax can be involved -> need to specify which types generic type is valid
// Often used for type parameters

// Type parameter specified as generic with angle brackets and UpperCamelCase
// "Generic type parameters" often represented as <T>

// In Rust, "generic" is anything that accepts one of more generic type parameters <T>
// Concrete is non-generic

// fn foo<T>(arg: T) { ... } sample generic function

// Concrete type `A`
struct A;

// Defining type `Single`, first use of `A` not preceded by `<A>`
// Thus, `Single` is concrete type
struct Single(A);
//            ^ Single's first use of type `A`

// Here `<T>` precedes first use of `T` so `SingleGen` is a generic type
// `T` type parameter is generic -> could be anything, including A
struct SingleGen<T>(T);

fn main() {
    // `Single` is concrete and explicitly takes `A`
    let _s = Single(A);

    // Create variable `_char` of type SingleGen<char> and give it value SingleGen('a')l
    // Here `SingleGen` has explicitly specified type parameter
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen` can also have type parameter implicitly specified
    let _t = SingleGen(A); // Uses 'a' defined at top
    let _i32 = SingleGen(6); // Uses `i32`
    let _char = SingleGen('a'); // Uses `char`
}
