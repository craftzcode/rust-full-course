//! Methods
// Is a function that lives on a value
// It's an action we can ask the value to execute
// When we have a value like integer or string, we can invoke a method on it
// What we do is we write (value.method())
// You can think of the method as a command that we give to the value

fn main() {
    //* STRING METHODS

    //* Trim
    // The (trim()) method removes whitespace from both ends of a string.
    // In this case, it will return "remove empty space" without the leading and trailing spaces
    let string_value_with_trim_method: &str = "           remove empty space           ";
    println!("{}", string_value_with_trim_method.trim());

    //* Is Alphabetic
    // The (is_alphabetic()) method checks if all characters in a string are alphabetic
    // In this case, it will return (true)
    let string_value_with_is_alphabetic_method: char = 'A';
    println!(
        "Is this string alphabetic?: {}",
        string_value_with_is_alphabetic_method.is_alphabetic()
    );

    //* Is Uppercase
    // The (is_uppercase()) method checks if all characters in a string are uppercase
    // In this case, it will return (true)
    let string_value_with_is_uppercase_method: &str = "HELLOWORLD";
    println!(
        "Is this string uppercase?: {}",
        string_value_with_is_uppercase_method.is_uppercase()
    );

    //* Is Lowercase
    // The (is_lowercase()) method checks if all characters in a string are lowercase
    // In this case, it will return (true)
    let string_value_with_is_lowercase_method: &str = "helloworld";
    println!(
        "Is this string lowercase?: {}",
        string_value_with_is_lowercase_method.is_lowercase()
    );

    //* INTEGER METHODS

    //* ABS
    // The (abs()) method it returns the absolute value of the integer, which is the non-negative value of the number without regard to its sign.
    // In this case, it will return 15
    let integer_value_with_abs_method: i32 = -15;
    println!("{}", integer_value_with_abs_method.abs());

    //* Pow
    // The (pow()) method raises the integer to the power of the specified exponent
    // In this case, it will return 225, which is -15 raised to the power of 2
    let integer_value_with_pow_method: i32 = -15;
    println!("{}", integer_value_with_pow_method.pow(2));

    //* FLOATING POINT METHODS

    //* Floor
    // The (floor()) method rounds a floating-point number down and returns the largest integer value that is less than or equal to the given floating-point number
    // In this case, it will return 3.0, which is the largest integer less than or equal to 3.1415926535897932384
    let float_value_with_floor_method: f64 = 3.1415926535897932384;
    println!("{}", float_value_with_floor_method.floor());

    //* Ceil
    // The (ceil()) method rounds a floating-point number up to the nearest integer value, returning the smallest integer that is greater than or equal to the number
    // In this case, it will return 4.0, which is the smallest integer greater than or equal to 3.1415926535897932384
    let float_value_with_ceil_method: f64 = 3.1415926535897932384;
    println!("{}", float_value_with_ceil_method.ceil());

    //* Round
    // The (round()) method rounds a floating-point number to the nearest integer value
    // If the fractional part is 0.5 or greater, it rounds up to the next integer; otherwise, it rounds down
    // In this case, it will return 3.0, which is the nearest integer to 3.1415926535897932384
    let float_value_with_round_method: f64 = 3.1415926535897932384;
    println!("{}", float_value_with_round_method.round());

    //* BOOLEAN METHODS

    //* Is Positive
    // The (is_positive()) method checks if the integer is greater than zero
    // It returns (true) if the integer is positive, and (false) otherwise
    let integer_value_with_is_positive_method: i32 = 25;
    println!(
        "Is this integer is a positive?: {}",
        integer_value_with_is_positive_method.is_positive()
    );

    //* Is Negative
    // The (is_negative()) method checks if the integer is less than zero
    // It returns (true) if the integer is negative, and (false) otherwise
    let integer_value_with_is_negative_method: i32 = -25;
    println!(
        "Is this integer is a negative?: {}",
        integer_value_with_is_negative_method.is_negative()
    );

    //* ARRAY METHODS

    //* Len
    // The (len()) method returns the number of elements in an array or a slice
    // In this case, it will return 5, which is the number of elements in the array [1, 2, 3, 4, 5]
    let array_value_with_len_method = [1, 2, 3, 4, 5];
    println!("Length of the array: {}", array_value_with_len_method.len());
}
