// Like tuples, arrays and slices can be destructed as well
fn main() {
    let array = [1, -2, 6];

    match array {
        // Binds second and third elements to respective vars
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        // Single vals can be ignored with _
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),
        // You can bind some, ignore rest
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {}, ignore rest",
            second
        ),
        // This won't compile
        // [-1, second] = > ...

        // Store them in another array or slice
        // Type depends on value being matched against
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {}, other elts were {:?}",
            second, tail
        ),

        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}