// Within destructuring of single variable, both `by-move` and `by-reference` pattern bindings can be used at same time
// Doing this results in partial move of variable, which means parts of variable are moved while other parts stay
// Parent vars cannot be used afterwards as a whole, parts that are only referenced (not moved) can still be used

fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    // println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("Person's age from persont struct is {}", person.age);
}