//! Math Operations

fn main() {
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

    //* Modulus Operation
    // The modulus operation returns the remainder of the division of two numbers
    let modulus = (a as i32) % (b as i32);
    println!("The modulus of {a} and {b} is {modulus}");
}
