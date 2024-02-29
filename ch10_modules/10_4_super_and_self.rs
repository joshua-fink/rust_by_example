// `super` and `self` keywords can be used in path to remove
// ambiguity when accessing items and prevent unnecessary hardcoding of paths

fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }
    
    pub fn indirect_call() {
        // Let's access all the functions named `function` from this scope!
        print!("called `my::indirect_call()`, that\n> ");

        // The `self` keyword refers to the current module scope - in this case `my`
        // Calling `self::function()` and calling `function()` directly both give
        // the same result, because they refer to the same function
        self::function();
        function();

        // `self` can be used to access another module inside `my`:
        self::cool::function();

        // The `super` keyword refers to parent scope (outside `my` module)
        super::function();

        // This will bind to `cool::function` in the *crate* scope
        // Crate scope is outermost scope
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}