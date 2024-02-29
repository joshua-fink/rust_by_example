// Match guard can be added filter the arm

#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    let temperature = Temperature::Celsius(35);

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        // if condition is guard
        Temperature::Celsius(t) => println!("{}C is equal or below 30 Celsius", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("{}F above 86", t),
        Temperature::Fahrenheit(t) => println!("{}F is equal to below 86", t),
    }

    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        // Compiler does not take guard conditions into account when checking if all patterns covered
        _ => unreachable!("Should never happen."),
    }
}


