// More than one way to unpack `Option` and fall back on default if `None`

// When making selection, consider:
// - Do we need eager or lazy evaluation?
// - Do we need to keep original empty value intact

/*
// or() is chainable, evaluates eagerly, keeps empty value intact
// Because or's arguments are evaluated eagerly, the variable passed to or is moved

#[derive(Debug)]
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;

    let first_available_fruit = no_fruit.or(orange).or(apple);
    println!("first_available_fruit: {:?}", first_available_fruit);

    // first_available_fruit: Some(Orange)

    // `or` moves its arg

    // In example above, `or(orange)` returned `Some` so `or(apple)` not invoked
    // But variable named apple has been moved regardless and cannot be used anymore

    // println!("Variable apple was moved, so this line won't compile: {:?}", apple);
    // This line fails to compile ^
}
*/

/*
// or_else() is chainable, evaluates lazily, keeps empty value intact

#[derive(Debug)]
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let no_fruit: Option<Fruit> = None;

    let get_kiwi_as_fallback = || {
        println!("Providing kiwi as fallback");
        Some(Fruit::Kiwi)
    };

    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Some(Fruit::Lemon)
    };

    let first_available_fruit = no_fruit
        .or_else(get_kiwi_as_fallback)
        .or_else(get_lemon_as_fallback);

    println!("first_available_fruit: {:?}", first_available_fruit);

    // Providing kiwi as fallback
    // Prints first_available_fruit: Some(Kiwi)
}
*/

/*
// get_or_insert() evaluates eagerly, modifies empty value in place
// Make sure option gets value, use `get_or_insert` to modify it in place with fallback value
// `get_or_insert()` eagerly evaluates its parameter, so variable `apple` is moved

#[derive(Debug)]
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let mut my_fruit: Option<Fruit> = None;
    let apple = Fruit::Apple;
    let first_available_fruit = my_fruit.get_or_insert(apple);
    println!("first_available_fruit is: {:?}", first_available_fruit);
    println!("my_fruit_is: {:?}", my_fruit);
    // Prints:
    // first_available_fruit is: Apple
    // my_fruit is: Some(Apple)
    // println!("Compile error, doesn't work because var `apple` is moved: {:?}", apple);
}
*/

// `get_or_insert_with()` evaluates lazily, modifies empty value in place
// Instead of explicitly providing value to fall back on, can pass a closure to `get_or_insert_with()`

#[derive(Debug)]
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let mut my_fruit: Option<Fruit> = None;
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Fruit::Lemon
    };

    let first_available_fruit = my_fruit
        .get_or_insert_with(get_lemon_as_fallback);
    println!("my first available fruit is: {:?}", first_available_fruit);
    println!("my_fruit is: {:?}", my_fruit);

    // providing lemon as fallback
    // first_available_fruit is: Lemon
    // my_fruit is: Some(Lemon)

    // If Option has value, it is left unchanged and closure not invoked
    let mut my_apple = Some(Fruit::Apple);
    let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback);
    println!("should_be_apple is: {:?}", should_be_apple);
    println!("my_apple is unchanged: {:?}", my_apple);
    // Output is as follows. Note closure `get_lemon_as_fallback` not invoked
    // should_be_apple is Apple
    // my_apple is unchanged: Some(Apple)
}










