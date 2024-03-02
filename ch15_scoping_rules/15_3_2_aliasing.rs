// Data can be immutably borrowed any number of times
// But while immutably borrowed, cannot be mutably borrow
// Only one mutable borrow allowed at a time
// Original data can be borrowed again only after mutable reference been used at last time

struct Point { x: i32, y: i32, z: i32 }

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // Data can be accessed via refs and original owner
    println!("Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z);

    // Error! Can't borrow point as mutable because currently borrowed as immutable
    // let mutable_borrow = &mut point;

    // Borrowed values are used again here
    println!("Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z);

    // Now immutable refs are no longer used for rest of code
    // Possible to reborrow with a mutable ref
    let mutable_borrow = &mut point;

    // Change data via mutable reference
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // Error! Cannot borrow as immutable because currently borrowed as mutable
    // let y = &point.y;

    // Error! Can't print because `println!` takes immutable reference
    // println!("Point Z coordinate is {}", point.z);

    // Ok! Mutable references can be passed as immutable to `println!`
    println!("Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);
    
    // Mutable reference no longer used for rest of code so possible to reborrow
    let new_borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);
}