// Like how generic types can be bounded, lifetimes (themselves generic) use bounds as well
// The : character has a slightly different meaning here, + is the same
// 1. T: 'a -> All references in T must outlive lifetime 'a
// 2. T: Trait + 'a: Type T must implement trait Trait and all references in T must outlive 'a

use std::fmt::Debug; // Trait to bound with

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` contains reference to generic type `T` that has
// unknown lifetime `'a`. `T` bounded such that any *references* in `T` must outlive `'a`
// Additionally, lifetime of `Ref` may not exceed `'a`.

// Generic function which prints using `Debug` trait
fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t is {:?}", t);
}

// Here reference to `T` taken where `T` implements `Debug`
// and all *references* in `T` outlive `'a` 
// `'a` must outlive function
fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}