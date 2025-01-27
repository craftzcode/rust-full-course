//! Interpolation with Curly Braces
// Allows embedding values into strings by placing variable names inside ({}) in the format string

fn main() {
    let apples = 50;
    let oranges = 14 + 6;

    // Old Rust Syntax
    // println!(
    //     "This year, my garden has {} apples and {} oranges.",
    //     apples, oranges
    // );

    // New Rust Syntax
    println!("This year, my garden has {apples} apples and {oranges} oranges.");
}
