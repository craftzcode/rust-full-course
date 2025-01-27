//! Methods
// Is a function that lives on a value
// It's an action we can ask the value to execute
// When we have a value like integer or string, we can invoke a method on it
// What we do is we write (value.method())
// You can think of the method as a command that we give to the value

fn main() {
    //* ABS Method
    let integer_value_with_abs_method: i32 = -15;

    // The (abs()) method it returns the absolute value of the integer, which is the non-negative value of the number without regard to its sign. 
    // In this case, it will return 15
    println!("{}", integer_value_with_abs_method.abs());

    //* Trim Method
    let string_value_with_trim_method = "           remove empty space           ";

    // The (trim()) method removes whitespace from both ends of a string.
    // In this case, it will return "remove empty space" without the leading and trailing spaces
    println!("{}", string_value_with_trim_method.trim());

    //* Pow Method
    let integer_value_with_pow_method: i32 = -15;

    // The (pow()) method raises the integer to the power of the specified exponent
    // In this case, it will return 225, which is -15 raised to the power of 2
    println!("{}", integer_value_with_pow_method.pow(2));
}
