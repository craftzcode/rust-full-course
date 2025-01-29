//! Array Type
// Is a collection of `homogenous data` elements of the same type
// It has a fixed size, which cannot be changed
// The elements of an array are stored in contiguous memory locations
// The elements can be accessed using an index, which starts at 0

fn main() {
    // The array is initialized with the values 4, 8, 12, 16, and 20
    // This data type ([i32; 5]) declares an (array) of 5 elements of type (i32), which is a 32-bit signed integer
    let array_numbers: [i8; 5] = [4, 8, 12, 16, 20];

    let array_strings: [&str; 5] = ["Apple", "Banana", "Grapes", "Strawberry", "Orange"];
    println!("Length of the array_strings: {}", array_strings.len());

    // This will result in an error
    // Because Rust requires the type and size of an array to be known at compile time to ensure memory safety and prevent runtime errors
    // This is important because Rust is a statically typed language, which means it checks the types of variables at compile time
    // By knowing the type and size of an array at compile time, Rust can allocate the necessary memory and prevent common errors like buffer overflows
    // let empty_array = [];
}
