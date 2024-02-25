// Type inference engine looks at type of value expression during initialization
// Looks at variable afterwards to infer its type

fn main() {
    // because of annotation, the compiler knows elem has type u8
    let elem = 5u8;

    // create empty vector (growable array)
    let mut vec = Vec::new();
    // At this point, compiler doesn't know exact type of vec,
    // it knows it is a vector of somethign ("Vec<_>").

    // Insert elem into vector, now compiler knows vec is vec of u8s (Vec<u8>)
    vec.push(elem); // if this line not here compiler throws error
    println!("{:?}", vec);
}