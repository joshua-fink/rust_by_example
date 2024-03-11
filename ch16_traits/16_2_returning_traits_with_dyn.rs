// Rust compiler needs to know amount of space each function's return type requires
// All functions must return concrete type
// Unlike other languages, you have trait like `Animal`, can't write function that returns Animal because diff implementations have diff amounts of memory

// Easy workaround -> instead of returning trait object directly, functions return Box which contains Animal
// Box is reference to some memory in the heap
// Reference has statically-known size, compiler guarantees it points to heap-allocated Animal, making it possible to return

// Use `Box<dyn Animal>` for example with keyword dyn to indicate this

struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement `Animal` trait for `Sheep`
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaah!"
    }
}

// Implement `Animal` trait for `Cow
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    } 
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("Randomly chose animal, says {}", animal.noise());
}