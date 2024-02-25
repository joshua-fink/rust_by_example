// RUST HAS NO IMPLICIT TYPE CONVERSION (COERCION) BETWEEN PRIMITIVE TYPES!
// But explicit type conversion can be performed

// Suppress all warnings from casts which overflow
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;
    
    // Error! No implicit conversion
    // let integer: u8 = decimal;

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // Error! Limitations in conversion rules, decimal cannot be converted into char
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to unsigned type, T
    // T::MAX + 1 is added or subtracted until value fits into new type
    println!("1000 as a u16: {}", 1000 as u16);

    // u8 max = 255, u8 max + 1 = 256
    // 1000 - 256 - 256 - 256 = 232
    // under the hood, first 8 least significant bits (LSB) are kept
    // rest toward most significant bit (MSB) get truncated
    println!("1000 as a u8 is : {}", 1000 as u8);

    // -1 + 256 = 255
    println!("  -1 as a u8 is: {}", (-1i8) as u8);

    // For positive nums, same as modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant bit is 1, the value is negative

    // Unless it already fits
    println!("128 as a i16 is: {}", 128 as i16);
    
    // In boundary case 128 value in 8-bit two's complement representation is -128
    println!("128 as a i8 is: {}", 128 as i8);

    println!("1000 as a u8 is: {}", 1000 as u8);
    // value of 232 in 8-bit two's complement representation is -24
    println!("232 as a i8 is: {}", 232 as i8);

    // Rust >=1.45, as keyword performs a saturating cast from float to int
    // If floating point value exceeds upper bound or lower bound, int equal to returned value crossed

    // 300.0 as u8 is 255
    println!("300.0 as u8 is: {}", 300.0_f32 as u8);

    // -100.0 as u8 is 0
    println!("-100.0 as u8 is: {}", -100.0_f32 as u8);

    // nan as u8 is 0
    println!(" nan as u8 is: {}", f32::NAN as u8);

    // This behavior incurs small runtime, avoid with unsafe methods
    // results might overflow and return unsound values
    unsafe {
        // 300.0 as u8 is 44
        println!("300.0 as u8 is: {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is: {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }
}