// Type can implement many different traits
// If two traits both require same name for function, need Fully Qualified Syntax to figure out difference

trait UsernameWidget {
    // Get selected username out of widget
    fn get(&self) -> String;
}

trait AgeWidget {
    // Get selected age out of widget
    fn get(&self) -> u8;
}

// Form with both UsernameWidget and AgeWidget
struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };

    // Error! Multiple `get` found due to implementations
    // println!("{}", form.get());

    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}