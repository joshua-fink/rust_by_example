// Some functions connect to type
// Two forms: associated functions and methods

// Associated functions -> defined on a type generally
// Methods -> associated functions called on particular instance of type

struct Point {
    x: f64,
    y: f64,
}


// Implementation block, all `Point` associated functions & methods go in here
impl Point {
    // This is "associated function" because function associated with type Point
    // Associated functions don't need to be called with instance
    // Functions used more like constructors
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Associated function with two args
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // &self is sugar for `self: &Self` where `Self` is type of caller object
    // in this case, `Self` = `Rectangle`
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via dot operator
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        
        // `abs` is a f64 method that returns abs val of caller
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    fn destroy(self) {
        // Destructure `self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
    }
}

fn main() {
    let rectangle = Rectangle {
        // Associated functions are called with double colons
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note first argument `&self` is implicitly passed
    // Ex. rectangle.perimeter() === Rectangle::perimeter(&rectangle)
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires mutable object
    // rectangle.translate(1.0, 1.0);

    // Mutable objects can call mutable methods
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous destroy call "consumed" pair
    // pair.destroy();
}

