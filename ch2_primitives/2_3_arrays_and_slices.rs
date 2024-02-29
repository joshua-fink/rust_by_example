use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // Length
    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to section of array.
    // Of form [start_index .. end_index]
    // start_index -> first pos
    // end_index -> last pos + 1
    println!("Borrow whole array as slice");
    analyze_slice(&ys[1 .. 4]);

    // Example of empty slice `&[]`
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // same, more verbose

    // arrays can be safely accessed using .get which returns 'Option'
    // Option can be matched or used with expect() if you want program to exit instead of continue
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(x_val) => println!("{}: {}", i, x_val),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Out of bound indexing causes compile time error
    // println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error
    // println!("{}", xs[..][5]);
}