// can declare variable bindings before initializing them later

fn main() {
    // Declare a variable binding
    let a_binding;

    {
        let x = 2;

        // Initialize binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized binding
    // println!("another binding: {}", another_binding);

    another_binding = 1;
    println!("another binding: {}", another_binding);

}

// compiler forbids use of uninitalized variables to avoid undefined behavior