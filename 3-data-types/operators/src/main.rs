//! Operators

fn main() {
    //? MATH OPERATORS

    let a: f64 = 10.5;
    let b: f64 = 2.5;

    //* Addition
    let sum = a + b;
    println!("The sum of {a} and {b} is {sum}");

    //* Subtraction
    let difference = a - b;
    println!("The difference between {a} and {b} is {difference}");

    //* Multiplication
    let product = a * b;
    println!("The product of {a} and {b} is {product}");

    //* Division
    // Floor Division
    // If we divide two integers, it will return the largest integer less than or equal to the division result
    let floor_division = (a as i32) / (b as i32);
    println!("The floor division of {a} divided by {b} is {floor_division}");

    // Decimal Division
    // If we divide two floating-point numbers, it will return the exact result of the division, including any decimal points
    let decimal_division = a / b;
    println!("The decimal division of {a} divided by {b} is {decimal_division}");

    //* Modulus
    // The modulus operation returns the remainder of the division of two numbers
    let modulus = (a as i32) % (b as i32);
    println!("The modulus of {a} and {b} is {modulus}");

    //? Augmented Assignment Operator
    // This operator allows you to perform an operation on a variable and update its value in a more concise way
    // Instead of writing the variable name twice, you can use a shorthand that combines the operation and assignment into one step
    // For example, instead of writing `year = year + 1`, you can simply write `year += 1`, which makes the code cleaner and easier to read

    //* +=
    // Using the augmented assignment operator to increment the year
    let mut year = 2025;
    year += 1;
    println!("The new year is {year}");

    //? EQUALITY OPERATOR

    //* Integer
    let integer_a: i8 = 5;
    let integer_b: i8 = 10;
    let is_equal: bool = integer_a == integer_b;
    println!("Is a equal to b? {is_equal}");

    //* Floating Point
    let float_a: f32 = 5.0;
    let float_b: f32 = 10.0;
    let is_equal_float: bool = float_a == float_b;
    println!("Is float_a equal to float_b? {is_equal_float}");

    //* String
    let string_a: &str = "Hello";
    let string_b: &str = "World";
    let is_equal_string: bool = string_a == string_b;
    println!("Is string_a equal to string_b? {is_equal_string}");

    //* Boolean
    let bool_a: bool = true;
    let bool_b: bool = false;
    let is_equal_bool: bool = bool_a == bool_b;
    println!("Is bool_a equal to bool_b? {is_equal_bool}");

    //* Character
    let char_a: char = 'a';
    let char_b: char = 'b';
    let is_equal_char: bool = char_a == char_b;
    println!("Is char_a equal to char_b? {is_equal_char}");

    //* With Casting Type
    let integer_a_with_casting_type: i8 = 10;
    let float_b_with_casting_type: f32 = 10.0;
    let is_equal_with_casting_type: bool =
        integer_a_with_casting_type == float_b_with_casting_type as i8;
    println!("Is integer_a_with_casting_type equal to float_b_with_casting_type? {is_equal_with_casting_type}");

    //? INEQUALITY OPERATOR

    //* Integer
    let integer_c: i8 = 15;
    let integer_d: i8 = 20;
    let is_not_equal: bool = integer_c != integer_d;
    println!("Is c not equal to d? {is_not_equal}");

    //* Floating Point
    let float_c: f32 = 15.0;
    let float_d: f32 = 20.0;
    let is_not_equal_float: bool = float_c != float_d;
    println!("Is float_c not equal to float_d? {is_not_equal_float}");

    //* String
    let string_c: &str = "Hello";
    let string_d: &str = "World";
    let is_not_equal_string: bool = string_c != string_d;
    println!("Is string_c not equal to string_d? {is_not_equal_string}");

    //* Boolean
    let bool_c: bool = true;
    let bool_d: bool = false;
    let is_not_equal_bool: bool = bool_c != bool_d;
    println!("Is bool_c not equal to bool_d? {is_not_equal_bool}");

    //* Character
    let char_c: char = 'a';
    let char_d: char = 'b';
    let is_not_equal_char: bool = char_c != char_d;
    println!("Is char_c not equal to char_d? {is_not_equal_char}");

    //* With Casting Type
    let integer_c_with_casting_type: i8 = 10;
    let float_d_with_casting_type: f32 = 10.0;
    let is_not_equal_with_casting_type: bool =
        integer_c_with_casting_type != float_d_with_casting_type as i8;
    println!("Is integer_c_with_casting_type not equal to float_d_with_casting_type? {is_not_equal_with_casting_type}");

    //? AND OPERATOR
    // The AND logical operator (&&) returns (true) if both conditions are (true)

    //* &&
    // In this example, both `is_admin` and `is_active` are (true), so `can_access` is also (true)
    // If either `is_admin` or `is_active` were (false), can_access would be (false)
    let is_admin: bool = true;
    let is_active: bool = true;
    let can_access: bool = is_admin && is_active;
    println!("Can access? {can_access}");

    //? OR OPERATOR
    // The OR logical operator (||) returns (true) if at least one of the conditions is (true)
    
    //* ||
    // In this example, `is_admin` is (true) and `is_guest` is (false), so `can_access` is (true)
    // If both `is_admin` and `is_guest` were (false), can_access would be (false)
    let is_admin: bool = true;
    let is_guest: bool = false;
    let can_access: bool = is_admin || is_guest;
    println!("Can access? {can_access}");
}