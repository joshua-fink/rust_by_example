// Arguments of macro are prefixed with `$` and type annotated with designator

macro_rules! create_function {
    // Macro takes arg of designator `ident` and creates function named `$func_name`
    // `ident` designator used for variable/function names
    ($func_name:ident) => {
        fn $func_name() {
            // `stringify!` macro converts an `ident` into string
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

// Create functions `foo` and `bar` with macro
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // Macro takes expression of type `expr` and prints
    // it as string along with result
    // `expr` designator used for expressions
    ($expression:expr) => {
        // `stringify!` converts expression *as is* into string
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    // Blocks are expressions too!
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
}

/*
Some available designators
- block
- expr -> for expressions
- ident -> for var/function names
- item
- literal -> for literal constants
- pat -> pattern
- stmt -> stmt
- tt -> token tree
- ty -> type
- vis -> visibility qualifier
*/