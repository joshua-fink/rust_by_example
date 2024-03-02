// Scopes play important role in ownership, borrowing, lifetimes
// Indicate to compiler when borrows are valid, when resources can be freed, when variables are created or destroyed

// Variables in Rust do more than hold data in the stack, also own resources
// Ex: Box<T> owns heap memory

// Rust enforces RESOURCE ACQUISITION IS INITIALIZATION
// whenever object goes out of scope, destructor called and owned resources freed

// Behavior shields against resource leak bugs, never have to manually free memory or worry about memory leaks

fn create_box() {
    // Allocate integer on the heap
    let _box1 = Box::new(3i32);

    // `_box1` destroyed here, memory freed
}

// Notion of destructor in Rust is provided via `Drop` trait
// Destructor called when resource goes out of scope
// Trait not required, only implement for your type when necessary to have destructor logic
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}


fn main() {
    // Allocate integer on heap
    let _box2 = Box::new(5i32);

    // A nested scope:
    {
        // Allocate integer on heap
        let _box3 = Box::new(4i32);

        // `_box3` destroyed here and memory freed
    }

    // Creating lots of boxes because why not?
    // No need to manually free memory
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` destroyed here, memory freed

    let x = ToDrop;
    println!("Made a ToDrop!");
}

// Can check for memory leaks with valgrind
// $ rustc 15_1_raii.rs && valgrind ./15_1_raii
