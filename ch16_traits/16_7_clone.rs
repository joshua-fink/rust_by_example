// Clone trait
// When dealing with resources, default behavior is to transfer them during assignments or function calls
// But when copies are needed, .clone() method helps with this

// Unit struct no resources
#[derive(Debug, Clone, Copy)]
struct Unit;

// Tuple struct with resources that implements `Clone` trait
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    // Instantiate `Unit`
    let unit = Unit;

    // Copy `Unit`, there are no resources to move
    let copied_unit = unit;

    // Both `Unit's` used independently
    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);

    // Instantiate `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);
    
    // Move `pair` into `moved_pair`, moves resources
    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    // Error! `pair` has lost its resources
    // println!("original: {:?}", pair); -> this causes error

    // Clone `moved_pair` into `cloned_pair` (resources included)
    let cloned_pair = moved_pair.clone();
    // Drop the moved original pair using std::mem::drop
    drop(moved_pair);

    // Error! `moved_pair` has been dropped
    // println!("moved and dropped: {:?}", moved_pair);

    // Result from .clone() can still be used!
    println!("clone: {:?}", cloned_pair);
}