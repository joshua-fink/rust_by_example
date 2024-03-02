// Mutability of data can be changed upon ownership transfer
fn main() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // Mutability error
    // *immutable box = 4;

    // *Move* box, changing ownership and mutability
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // Modify box contents
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}