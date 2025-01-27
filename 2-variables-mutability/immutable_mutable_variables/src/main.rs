//! Immutable & Mutable Variables

fn main() {
    //* Immutable
    // In Rust, variables are immutable by default
    // This means once a value is bound to a name, you cannot change that value

    // For example:
    let x = 5; // x is immutable
    println!("Old value of x is {x}");
    // x = 6; // This would cause a compilation error

    //* Mutable
    // To make a variable mutable, we must add the `mut` keyword

    // For example:
    let mut y = 5; // y is mutable
    println!("Old value of y is {y}");

    y = 6; // This is allowed because y is mutable
    println!("New value of y is {y}");
}
