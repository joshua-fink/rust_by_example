// Like how generic types can be bounded, lifetimes (themselves generic) use bounds as well
// The : character has a slightly different meaning here, + is the same
// 1. T: 'a -> All references in T must outlive lifetime 'a
// 2. T: Trait + 'a: Type T must implement trait Trait and all references in T must outlive 'a

use std::fmt::Debug; // Trait to bound with

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
