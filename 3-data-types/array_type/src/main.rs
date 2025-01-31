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

    //? Reading & Writing Array Elements
    // In Rust, array indexing starts at 0. This means that the first element of the array is at index 0,the second element is at index 1, and so on

    // For example, in the 'seasons' array below
    // 'Spring' is at index 0, 'Summer' is at index 1, 'Fall' is at index 2, and 'Winter' is at index 3.
    let seasons = ["Spring", "Summer", "Fall", "Winter"];
    // The following lines of code access elements from the 'seasons' array using their indices
    // 'first_index' is assigned the value of the first element in the array (index 0), which is "Spring"
    // 'second_index' is assigned the value of the second element in the array (index 1), which is "Summer"
    let first_index = seasons[0];
    let second_index = seasons[1];
    println!("The first season is {first_index} and the second season is {second_index}");
    println!("{}", seasons[3]);

    // Arrays have a fixed size, meaning we cannot add or remove elements from them 
    // However, we can create a mutable array that allows us to replace elements at specific indices
    // Here is an example of a mutable array:
    let mut numbers = [1, 2, 3, 4];
    println!("Original array: {:?}", numbers);

    // We can replace the element at index 2 (which is currently 3) with a new value, say 10
    numbers[2] = 10;
    println!("Updated array: {:?}", numbers);
}
