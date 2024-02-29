// THIS DOES NOT COMPILE, EVEN COPIED FROM RUST BY EXAMPLE

// Iterator::any is function which when passed iterator
// Return `true` if any element satisfies the predicate, else `false`

pub trait Iterator {
    // Type being iterated over
    type Item;

    // `any` takes `&mut self` meaning caller may be borrowed
    // and modified, but not consumed
    fn any<F>(&mut self, f: F) -> bool where
        // `FnMut` meaning any captured variable may be modified, not consumed
        // `Self::Item` states it takes arguments to closure by value
        F: FnMut(Self::Item) -> bool;
}


fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    // `iter()` only borrows `vec1` and elts, they can be used again
    println!("vec1 len: {}", vec1.len());
    println!("First element of vec1 is: {}", vec1[0]);
    // `into_iter()` does move `vec2` and its elts, so they can't be used again
    // println!("First element of vec2 is: {}", vec2[0]);
    // println!("vec2 len: {}", vec2.len());
    // TODO: uncomment two lines above and see compiler errors

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`
    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    // `into_iter()` for arrays yields `i32`.
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));
}
