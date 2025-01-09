// Variable Types
// Scalar Types
fn scalar_types_demo(){
    // Integers
    // 8-bit signed integer
    let eight_bit_signed_max: i8 = 127; // i8::MAX
    let eight_bit_signed_min: i8 = -128; // i8::MIN

    // 8-bit unsigned integer
    let eight_bit_unsigned_max: u8 = 255; // u8::MAX
    let eight_bit_unsigned_min: u8 = 0; // u8::MIN

    // 16-bit signed integer
    let sixteen_bit_signed_max: i16 = 32_767; // i16::MAX
    let sixteen_bit_signed_min: i16 = -32_768; // i16::MIN

    // 16-bit unsigned integer
    let sixteen_bit_unsigned_max: u16 = 65_535; // u16::MAX
    let sixteen_bit_unsigned_min: u16 = 0; // u16::MIN

    // 32-bit signed integer
    let thirty_two_bit_signed_max: i32 = 2_147_483_647; // i32::MAX
    let thirty_two_bit_signed_min: i32 = -2_147_483_648; // i32::MIN

    // 32-bit unsigned integer
    let thirty_two_bit_unsigned_max: u32 = 4_294_967_295; // u32::MAX
    let thirty_two_bit_unsigned_min: u32 = 0; // u32::MIN

    // 64-bit signed integer
    let sixty_four_bit_signed_max: i64 = 9_223_372_036_854_775_807; // i64::MAX
    let sixty_four_bit_signed_min: i64 = -9_223_372_036_854_775_808; // i64::MIN

    // 64-bit unsigned integer
    let sixty_four_bit_unsigned_max: u64 = 18_446_744_073_709_551_615; // u64::MAX
    let sixty_four_bit_unsigned_min: u64 = 0; // u64::MIN

    // 128-bit signed integer
    let one_twenty_eight_bit_signed_max: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727; // i128::MAX
    let one_twenty_eight_bit_signed_min: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728; // i128::MIN

    // 128-bit unsigned integer
    let one_twenty_eight_bit_unsigned_max: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455; // u128::MAX
    let one_twenty_eight_bit_unsigned_min: u128 = 0; // u128::MIN
    
    // Integer Literals

    let integer_with_literal_suffix: u8 = 1u8;
    // Decimal literal
    let decimal_literal: i32 = 98_222; // Regular decimal value with underscores for readability

    // Hexadecimal literal
    let hexadecimal_literal: i32 = 0xff; // Hexadecimal value (255 in decimal)

    // Octal literal
    let octal_literal: i32 = 0o77; // Octal value (63 in decimal)

    // Binary literal
    let binary_literal: i32 = 0b1111_0000; // Binary value (240 in decimal)

    // Byte literal (u8 only)
    let byte_literal: u8 = b'A'; // Byte value representing the ASCII code for 'A' (65 in decimal)

    // Floating point
    let x = 2.0; // f64 by default

    let y: f32 = 3.0; // f32
    let y: f64 = 2.999; // f64

    // Boolean 
    let true_value: bool = true;
    let false_value: bool = false;

    // Characters
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    let japanese_symbol = '会'; // Chinese, Japanese, Korean supported as 4 bytes, c++ 1 byte
}

fn compound_types_demo(){
    // Tuples
    // Type annotation is optional
    let tuple = (500, 6.4, 1);

    // Dot notation
    let position_one = tuple.0;

    // Destructuring
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // Arrays

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [1, 2, 3];
    let c = [3; 5];

    let element = a[0];
}

// Mutability
fn mutability_demo() {
    let x = 5;
    println!("The value of x is: {x}");
    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");
}

// Constants
const TEST_CONSTANT: u32 = 111;
const TEST_CONSTANT_EXPRESSION: u32 = 1 * 1;

// Shadowing
fn shadowing_demo(){
    let x = 5;
    println!("The value of x is: {x}");
    let x = x + 1;
    println!("The value of x is: {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

}