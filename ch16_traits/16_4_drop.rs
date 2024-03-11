// `Drop` trait has only one method: drop, called automatically when object goes out of scope
// `Drop` -> used to free resources implementor instance owns

// `Box`, `Vec`, `String`, `File`, `Process` are examples of types that implement Drop trait to free resources
// `Drop` trait can also be manually implemented for any custom data type

struct Droppable {
    name: &'static str,
}

// Trivial implementation of `drop` adds prints to console
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // Block A
    {
        let _b = Droppable {name: "b"};
        
        // Block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just excited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // Variable can be manually dropped using `drop` function
    drop(_a);

    println!("end of main function");
    // Already manually dropped _a, so _a won't be `drop`ped again here
}