// Compiler is capable of providing basic implementations for some traits via `#[derive]` attribute
// Can manually implement if more complex behavior is required

// List of derivable traits
// - comparsion traits
// - `Clone`, to create `T` from `&T` via copy
// - `Copy` to give type 'copy semantics' instead of 'move semantics'
// - `Hash` to compute hash from &T
// - `Default` to create empty instance of data type
// - `Debug` to format value using `{:?}` formatter

// `Centimeters`, tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds` tuple struct with no additional attributes
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // Error! Seconds cannot be printed, fails to implement `Debug` trait
    // println!("One second looks like: {:?}", _one_second);

    // Error! `Seconds` cannot be compared, does not implement the PartialEq trait
    // let _this_is_true = (_one_second == _one_second);

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);
    
    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}