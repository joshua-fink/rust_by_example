// Structs have extra level of visibility with their field
// Visibility defaults to private, overridden with pub modifier
// Visibility only matters when a struct is accessed from outside module where it is defined, w/ encapsulation goals

mod my {
    // A public struct with a public field of generic type `T`
    pub struct OpenBox<T> {
        pub contents: T, // key difference here is pub keyword
    }

    // Public struct with private field of generic type `T`
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // Public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    // Public structs with public fields can be constructed as usual
    let open_box = my::OpenBox { contents: "public information" };

    // fields can be normally accessed
    println!("The open box contains: {}", open_box.contents);

    // Public structs with private fields cannot be constructed using field names
    // Error! `ClosedBox` has private fields
    // let closed_Box  = my::ClosedBox { contents: "classified info"};

    // However, structs with private fields can be created w/ public constructors
    let _closed_box = my::ClosedBox::new("classified info");

    // Private fields of a public struct cannot be accessed
    // Error! `contents` is private
    // println!("The closed box contains: {}", _closed_box.contents);
}