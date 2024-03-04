// Rust has few reserved lifetime names
// One of those is 'static

/*
// Reference with 'static lifetime
let s: &'static str = "hello world";

// 'static as part of trait bound:
fn generic<T>(x: T) where T: 'static {}
*/
// Subtle differences, common source of confusion

// 1) Reference lifetime
// Reference lifetime 'static incidates data pointed to by reference lives for remaining lifetime of program
// Can be coerced into shorter lifetime 

// Two common ways to make var with 'static lifetime -> both are stored in read-only memory of binary
// Make constant with static declaration
// Make string literal with type `&'static str`

/*
// Make constant with `'static` lifetime
static NUM: i32 = 18;

// Returns reference to `NUM` where `'static` lifetime coerced to that of input arg
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // Make `string` literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);
        
        // When `static_string` goes out of scope, reference
        // can be no longer used, but data remains in binary
    }

    {
        // Make integer to use for `coerce_static`:
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", &NUM);
}
*/

// 'static references only need to be valid for reminder of program's life
// Can be created while program is executed
// To demonstrate, below example uses Box::leak to dynamically create 'static references
// Doesn't live for entire duration, only for leaking point onward

/* Can't compile :(
extern crate rand;
use rand::Fill;

fn random_vec() -> &'static [usize; 100] {
    let mut rng = rand::thread_rng();
    let mut boxed = Box::new([0; 100]);
    boxed.try_fill(&mut rng).unwrap();
    Box::leak(boxed)
}

fn main() {
    let first: &'static [usize; 100] = random_vec();
    let second: &'static [usize; 100] = random_vec();
    assert_ne!(first, second);
}
*/

// 2) Trait bound
// As trait bound, type does not contain any non-static references
// Receiver can hold on to type as long as they want, will never become invalid until they drop it

// Important to understand any owned data always passes 'static lifetime bound
// Reference to owned data generally does not

/* Will not compile, i does not live long enough
use std::fmt::Debug;

fn print_it( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input);
}

fn main() {
    // i is owned and contains no references, thus it's 'static
    let i = 5;
    print_it(i);

    // &i only has lifetime defined by scope of main(), so not 'static:
    print_it(&i);
}
*/

