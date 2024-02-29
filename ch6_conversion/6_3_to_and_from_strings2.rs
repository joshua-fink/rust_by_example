// Often strings need to be converted into number
// Idomatic approach is parse function, will work as long FromStr trait works for that type

fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}