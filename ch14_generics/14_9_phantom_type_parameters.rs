// Phantom type parameter is one that doesn't show up at runtime
// Checked statically (and only) at compile time

// Data types can use extra generic type parameters to act as markers
// or perform type checking at compile time. Extra params hold no storage values, and have no runtime behavior

// Combine std::marker::PhantomData with phantom type parameter concept
// to create tuples containing different data types

use std::marker::PhantomData;

// Phantom tuple struct which is generic over `A` with hidden param `B`
#[derive(PartialEq)] // Allow equality test for this type
struct PhantomTuple<A, B>(A, PhantomData<B>);

// Phantom type struct generic over `A` with hidden param `B`
#[derive(PartialEq)]
struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> }

// Note: Storage allocated for generic type `A` but not for `B`
// B cannot be used in computations

fn main() {
    // Here, `f32` and `f64` are hidden params
    // PhantomTuple type specified as `<char, f32>`
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // PhantomTuple type specified as `<char, f64>`
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // Type specified as `<char, f32>`
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // Type specified as `<char, f64>`
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // The following lines cause compile-time errors due to type mismatches
    // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);
    // println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2);
}