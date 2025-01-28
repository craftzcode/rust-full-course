//! Augmented Assignment Operator
// This operator allows you to perform an operation on a variable and update its value in a more concise way
// Instead of writing the variable name twice, you can use a shorthand that combines the operation and assignment into one step
// For example, instead of writing `year = year + 1`, you can simply write `year += 1`, which makes the code cleaner and easier to read

fn main() {
    // Using the augmented assignment operator to increment the year
    let mut year = 2025;
    year += 1;
    println!("The new year is {year}");
}
