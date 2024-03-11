// Trait is collection of methods defined for unknown type: `Self`
// Can access other methods declared in same trait

// Can be implemented for any data type

// Defined Animal, group of methods
// Animal trait implemented for Sheep data type, allowing use of methods from Animal with Sheep

struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // Associated function signature; `Self` refers to implementor type
    fn new(name: &'static str) -> Self;

    // Method signatures; these will return String
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use implementor's trait methods
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// Implememt `Animal` trait for `Sheep`
impl Animal for Sheep {
    // `Self` is implementor type: `Sheep`
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaaah?"
        } else {
            "baaaaaah!"
        }
    }

    // Default trait methods can be overridden
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}