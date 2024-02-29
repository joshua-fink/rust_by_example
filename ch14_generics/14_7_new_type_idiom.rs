// `newtype` idiom gives compile time guarantees that right type of value is supplied to program

// age verification function that checks age in years, must be given value of type Years

struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    // truncates partial years
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(5);
    let age_days = age.to_days();

    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // Error! must be type Years
    // println!("Old enough {}", old_enough(&age_days));

    // To obtain `newtype`'s value as base type, use tuple or destructuring syntax like so
    let years = Years(42);
    let _years_as_primitive_1: i64 = years.0; // Tuple
    let Years(_years_as_primitive_2) = years; // Destructuring
}
