// Can unpack `Option`s by using match statements, often easier to use `?` operator
// If `x` is `Option`, evaluating x? will return value if x is Some, else terminate whatever function is executed and return None

fn next_birthday(current_age: Option<u8>) -> Option<String> {
    // If `current_age` is `None`, this returns `None`
    // If `current_age` is `Some`, inner `u8` value + 1 assigned to `next_age`
    let next_age: u8 = current_age? + 1;
    Some(format!("Next year I will be {}", next_age))
}

// Can chain many `?`s together to make code more readable
struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {

    // Gets area code of phone number of person's job if exists
    fn work_phone_area_code(&self) -> Option<u8> {
        // This would take lots of nested `match` statements without `?` operator
        self.job?.phone_number?.area_code
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}