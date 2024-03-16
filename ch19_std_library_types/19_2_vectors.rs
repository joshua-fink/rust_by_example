// Vectors

// Vectors are resizable arrays
// Size not known at compile time, can grow or shrink at any time

// Vector represented using 3 params: pointer to data, length, capacity

// Capacity indicates amount of memory reserved for vector. Vector can grow as long as length smaller than capacity
// Reallocated with larger capacity when threshold must be surpassed

fn main() {
    // Iterators can be collected into vectors
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // `vec!` macro can be used to initialize vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // Insert new element at the end of vector
    println!("Push 4 into vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // collected_iterator.push(0); // Error! Immutable vectors cannot grow

    // `Len` method yields number of elements currently stored in a vector
    println!("Vector length: {}", xs.len());

    // Indexing done using square brackets (indexing starts at 0)
    println!("Second element: {}", xs[1]);

    // `pop` removes last element from vector and returns it
    println!("Pop last element: {:?}", xs.pop());

    // Out of bounds indexing yields panic
    // println!("Fourth element: {}", xs[3]);

    // `Vector`s can be easily iterated over
    println!("Contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }

    // `Vector` can also be iterated over while iteration count is enumerated in separate variable (`i`)
    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i , x);
    }

    // Thanks to `iter_mut`, mutable `Vector`s can also be iterated over in way that allows modifying each value
    for x in xs.iter_mut() {
        *x *= 3
    }
    println!("Updated vector: {:?}", xs);
}