// `use` declaration can be used to bind full path to new name
/*
use crate::deeply::nested::{
    my_first_function,
    my_second_function,
    AndATraitType,
};

fn main() {
    my_first_function();
}
*/

// Use `as` keyword to bind imports to different name

// Bind `deeply::nested::function` path to `other_function`
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    // Easier access to `deeply::nested::function`
    other_function();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function` as function
        // This function() shadows outer one

        // `use` bindings have local scope. Shadowing of function only in block
        function();

        println!("Leaving block");
    }

    function();

}