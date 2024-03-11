// impl Trait can be used in two locations
// 1) as arg type
// 2) as return type 

// Argument type -> if function is generic over trait but don't mind specific type, simplify the function declaration with impl trait as type of argument

/*
fn parse_csv_document<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // for each line in source
            line.map(|line| {
                // If line read successfully, process it, if not, return error
                line.split(',')
                    .map(|entry| String::from(entry.trim())) // Remove leading/trailing whitespaces
                    .collect() // Collect all strings in a row into Vec<String>
            })
        })
        .collect() // Collect all lines into Vec<Vec<String>>
}
*/

// parse_csv_document is generic, allowing it to take any type which implements BufRead, like BufReader<File> or [u8]
// Not as important what type R is, R is only used to declare type of src, so can rewrite function like so:

/*
fn parse_csv_document(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // for each line in src
            line.map(|line| {
                // if line read successfully, process it, if not return error
                line.split(',') // split line separated from commas
                    .map(|entry| String::from(entry.trim())) // remove leading/trailing whitespace
                    .collect() // Collect all strings in row into Vec<String>
            })
        })
        .collect() // Collect all lines into Vec<Vec<String>>>
}
*/

// Using impl Trait as an argument type means you can't explicitly state what form of function you use
// i.e. parse_csv_document::<std::io::Empty>(std::io::empty()) won't work with second example

// As return type

// if function returns type that implements MyTrait, you can write return type as -> impl MyTrait
// this can help simplify type signatures

use std::iter;
use std::vec::IntoIter;

// Function combines two `Vec<i32>` and returns iterator over it
// Very complicated :(
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// Same function as above, except return type uses `impl Trait`
fn combine_vecs(
    v: Vec<i32>,
    u: Vec<i32>,
) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// Some Rust types cannot be written out. Every closure has own unnamed concrete type
// Before `impl Trait` syntax, had to allocate on heap in order to return a closure
// Now you can do it statically, like so:

// Returns function that adds `y` to input
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y };
    closure
}

// Can use impl Trait to return iterator that uses map or filter closures, making map and filter easier
// Closure types do not have names, cannot use explicit return type if your function returns iterators with closures
// impl Trait you can do this easily
fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2)
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("all done");

    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);

    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positives(&singles);
    assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);
}
