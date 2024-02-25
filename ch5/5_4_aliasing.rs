// `type` statement can be used to give new name to existing type
// Types must be UpperCamelCase, else compiler throws warning
// Exception to rule are primitive types
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    // Type aliases do not provide extra type safety
    // Because... aliases are not new types
    println!("{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches);
}