// Longer lifetime can be coerced into shorter so it works inside scope normally wouldn't work in
// Rust compiler performs inferred coercion
// Also comes in form of declaring lifetime difference

// Rust infers lifetime that is as short as possible
// Two references then coerced into that lifetime
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b> reads as lifetime `'a` at least as long `'b`
// Take in `&'a i32` returns `&'b i32` as result of coercion
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2; // longer lifetime

    {
        let second = 3; // shorter lifetime

        println!("Product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    };
}