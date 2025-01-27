//! Integer

fn main() {
    //* Signed
    // Signed integers can represent both positive and negative values, allowing for a wider range of numerical representation

    // The (i8) data type is a signed 8-bit integer, which can represent values from -128 to 127
    let eight_bit_signed: i8 = -100;

    // The (i16) data type is a signed 16-bit integer, which can represent values from -32,768 to 32,767
    let sixteen_bit_signed: i16 = -15000;

    // The (i32) data type is a signed 32-bit integer, which can represent values from -2,147,483,648 to 2,147,483,647
    let thirty_two_bit_signed: i32 = -2000000000;

    // The (i128) data type is a signed 128-bit integer, which can represent values from -2^127 to 2^126
    let one_hundred_twenty_eight_bit_signed: i128 = -3000000000000;
    
    //* Unsigned
    // Unsigned integers can only represent non-negative values, which allows for a larger maximum value for the same number of bits

    // The (u8) data type is an unsigned 8-bit integer, which can represent values from 0 to 255
    let eight_bit_unsigned: u8 = 200;

    // The (u16) data type is an unsigned 16-bit integer, which can represent values from 0 to 65,535
    let sixteen_bit_unsigned: u16 = 60000;

    // The (u32) data type is an unsigned 32-bit integer, which can represent values from 0 to 4,294,967,295
    let thirty_two_bit_unsigned: u32 = 3000000000;

    // The (u128) data type is an unsigned 128-bit integer, which can represent values from 0 to 2^127
    let one_hundred_twenty_eight_bit_unsigned: u128 = 3000000000000000000000000000000;

    //* Alternative Syntax
    // Here, we are using a different syntax to define signed and unsigned integers.

    let signed_bit = 100i8; 
    let unsigned_bit = 200u8;
}
