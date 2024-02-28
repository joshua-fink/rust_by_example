// Closures are flexible and will do what functionality requires
// to make closure work

// Capturing flexibly adapts to use case, moving or borrowing
// Closures capture variables by reference, but can go lower when required
// Reference, mutable reference, value
fn main() {
    use std::mem;

    let color = String::from("green");

    // Closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in `print` var
    // remain borrowed until `print` used last time

    // `println` only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive
    let print = || println!("`color`: {}", color);

    // Call closure using the borrow
    print();

    // `color` can be borrowed immutably again, because closure only holds
    // immutable reference to color.
    let _reborrow = &color;
    print();

    // A move or reborrow is allowed after final use of `print`
    let _color_moved = color;

    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that
    // Immediately borrows `count`


    // A `mut` is required on `inc` because a `&mut` is stored inside
    // Calling the closure mutates `count` which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure using a mutable borrow
    inc();

    // The closure still mutably borrows `count` because it is called later
    // Attempt to reborrow will lead to error
    // let _reborrow = &count; THIS LINE CAUSES ERROR!
    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, possible
    // to reborrow without error
    let _count_reborrowed = &mut count;


    // A non-copy type
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value
    // Copy type would copy into the closure leaving original untouched
    // A non-copy must move and so `movable` immediately moves into the closure
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can be only called once
    consume();
    // consume(); THIS LINE CAUSES ERROR

    let haystack = vec![1, 2, 3];
    
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There are {} elements in vec", haystack.len());
    // LINE ABOVE CAUSES COMPILE-TIME ERROR
    // borrow checker doesn't allow reusing variable after it has been moved

    // Removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting above line will not cause error
}