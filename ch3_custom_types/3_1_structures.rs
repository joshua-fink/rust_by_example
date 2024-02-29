// Hide warnings for dead code
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}


// THREE TYPES OF STRUCTS -> UNIT, TUPLE, C-STYLE
// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(r: Rectangle) -> f32 {
    let h = r.bottom_right.x - r.top_left.x;
    let v = r.top_left.y - r.bottom_right.y;
    h * v
}

fn square(top_left: Point, side_length: f32) -> Rectangle {
    Rectangle {
        bottom_right: Point { 
            x: top_left.x + side_length,
            y: top_left.y - side_length, 
        },
        top_left: top_left,
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a point
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use fields of other one
    let bottom_right = Point { x: 5.2, ..point };

    // bottom_right.y is the same as point.y because we used field from point
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // destructure the point using a let binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // instantiate a unit struct
    let _unit = Unit;

    // instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access fields of tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // Activity 1:
    println!("rect area: {}", rect_area(Rectangle { top_left: Point { x: 1.0, y: 5.0 }, bottom_right: Point {x: 7.0, y: 2.0}}));

    println!("This is a square: {:?}", square(Point{ x: 4.0, y: 3.0 }, 5.0));

}