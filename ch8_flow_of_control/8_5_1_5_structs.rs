// You can destructure a struct...

fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {} ", b, y),

        // you can destructure structs and rename the variables
        // order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // you can ignore vars
        Foo { y, .. } => println!("y = {}, we don't care about x", y),

        // error because pattern makes no mention of field x
        // Foo { y } => println!("y = {}", y),
    }

    let faa = Foo { x: (1, 2), y: 3 };

    // do not need match block to destructure structs
    let Foo { x: x0, y: y0 } = faa;
    println!("Outisde: x0 = {x0:?}, y0 = {y0}");

    // destructuring works with nested structs as well
    struct Bar {
        foo: Foo,
    }

    let bar = Bar { foo: faa };
    let Bar { foo: Foo { x: nested_x, y: nested_y } } = bar;
    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
}