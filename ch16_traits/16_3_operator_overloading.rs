// Many operators can be overloaded via traits
// Some operators can be used to accomplish diff tasks based on input arguments
// Possible because operators are syntactic sugar for method callss
// + operator in a + b calls add method a.add(b). add method part of `Add` trait
// Thus, ANY implement of Add trait can use `+`

// core::ops contains list of traits where overload ops can be found

use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// The `std::ops::Add` trait used to specify functionality of `+`
// `Add<Bar>` -> trait for addition with RHS of type `Bar`
// Following block implements operation: Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;
    
    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

// Opposite of previous one
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}