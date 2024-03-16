// HashSet

// Consider HashSet as a HashMap where we just care about keys ( HashSet<T> is in actuality, just wrapper around HashMap<T, ()>)

// HashSet unique feature is that it is guaranteed to not have duplicate elements -> all set collections fulfill this
// If you insert value in HashSet, new value will replace old if same

// 4 primary operations: union (get all unique in both sets)
// difference: all elts in first but not second
// intersection: get all elements that are only in both sets
// symmetric_difference: get all elements that are in one set or other but not both

use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // `HashSet::insert()` returns false if there was already value present
    // assert!(b.insert(4), "Value 4 already in set B!");
    // This line fails assert ^

    b.insert(5);

    // If collection's element type implements `Debug`, collection implements `Debug`
    // Usually prints its elements in format `[elem1, elem2, elem3, ... ]`
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // Print [1, 2, 3, 4, 5] in arbitrary order
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    // This prints [1]
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    // Print [2, 3, 4] in arbitrary order
    println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());

    // Print [1, 5]
    println!("Symmetric difference: {:?}", a.symmetric_difference(&b).collect::<Vec<&i32>>());
}