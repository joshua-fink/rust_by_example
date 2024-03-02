// When doing pattern matching or destructuring via let binding
// ref keyword can be used to take references to fields of struct/tuple

#[derive(Clone, Copy)]
struct Point { x: i32, y: i32 }

fn main() {
    let c = 'Q';
    
    // `ref` borrow on left side of assignment equivalent 
    // to `&` borrow on right side
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // `ref` also valid when destructuring struct
    let _copy_of_x = {
        // `ref_to_x` is reference to `x` field of `point`
        let Point { x: ref ref_to_x, y: _ } = point;

        // Return copy of `x` field of `point`
        *ref_to_x
    };

    // Mutable copy of `point`
    let mut mutable_point = point;
    
    {
        // `ref` can be paired with `mut` to take mutable reference
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

        // Mutate `y` field of `mutable_point` via mutable reference
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    // Mutable tuple that includes pointer
    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        // Destructure `mutable_tuple` to change value of `last`
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }

    println!("tuple is {:?}", mutable_tuple);
}