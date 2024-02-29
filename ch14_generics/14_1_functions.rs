// Funcitons take same set of rules for generics -> type T becomes generic when preceded by <T>

// Generic functions sometimes need type parameters to be explicitly specified
// Example situtaions ^: if function called where return type is generic, or compiler lacks the necessary information

// Example function call with explicitly specified type parameters
// fun::<A, B, ...> ()

struct A; // Concrete type `A`
struct S(A); // Concrete type `S`
struct SGen<T>(T); // Generic type `SGen`

// Following functions all take ownership of var passed into them and immediately go out of scope, freeing var

// Takes in arg `_s` of type `S`
// No `<T>` so not generic function
fn reg_fn(_s: S) {}

// Takes in arg `_s` of type `SGen<T>`
// Explicitly given type param `A`, but `A` has not been specified as generic type parameter
// for `gen_spec_t()` -> not generic
fn gen_spec_t(_s: SGen<A>) {}

// Takes arg `_s` of type `SGen<i32>`
// Explicitly given type parameter `i32` -> specific type
// `i32` not generic type, function not generic type
fn gen_spec_i32(_s: SGen<i32>) {}

// Define fucnction `generic` that takes arg `_s`of type `SGen<T>`
// Because `SGen<T>` preceded by `<T>`, this function generic over `T`
fn generic<T>(_s: SGen<T>) {}

fn main() {
    // Using non-generic functions
    reg_fn(S(A)); // Concrete type
    gen_spec_t(SGen(A)); // Implicitly specified type param `A`
    gen_spec_i32(SGen(6));

    // Explicitly specified type parameter `char` to generic()
    generic::<char>(SGen('a'));

    // Implicitly specified type parameter `char` to `generic()`
    generic(SGen('c'));
}