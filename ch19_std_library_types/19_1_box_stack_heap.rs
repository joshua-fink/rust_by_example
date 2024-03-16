// Box, stack, and heap

// All values are stack allocated by default
// Values can be boxed (allocated on the heap) by using Box<T>

// Box is a smart pointer to heap allocated value of type T
// When box goes out of scope, its destructor is called, inner object is destroyed, memory on heap is freed

// Boxed values can be dereferenced using * operator, removing one layer of indirection

use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]

struct Point {
    x: f64,
    y: f64,
}

// Rectangle can be specified by where top left and bottom right corners are in place
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // Allocate this point on heap, return pointer to it
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    // (all type annotations are superfluous)
    // Stack allocated variables
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 }
    };

    // Heap allocated rectangle
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // Output of functions can be boxed
    let boxed_point: Box<Point> = Box::new(origin());

    // Double indirection
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!("Point occupies {} bytes on stack", mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes on stack", mem::size_of_val(&rectangle));

    // box_size == pointer_size
    println!("Boxed point occupies {} bytes on stack", mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes on stack", mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes on stack", mem::size_of_val(&box_in_a_box));

    // Copy data contained in `boxed_point` into `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes on stack", mem::size_of_val(&unboxed_point));
}