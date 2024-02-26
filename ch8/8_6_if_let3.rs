enum Foo {Bar}

fn main() {
    let a = Foo::Bar;

    // Variable a matches Foo::Bar
    // if let allows us to account for instances not in the enum
    if let Foo::Bar = a {
        println!("a is foobar");
    }
}