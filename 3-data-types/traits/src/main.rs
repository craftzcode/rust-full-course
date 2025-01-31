//! Traits
// A trait is a contract that requires that a type support one or more methods
// Traits establish consistency between types; methods that represent the same behavior have the same name
// When a type opts in to honoring a trait's requirement, we say the type implements the trait
// Types can vary in their implementation but still implement the same trait
// A type can choose to opting in to implementing a trait
// A type can implement multiple traits. There are hundreds of traits available in Rust
// A trait is called an interface or protocol in other programming languages

fn main() {
    //* The Display Trait
    // The Display Trait allows a type to be shown as a simple, easy-to-read string
    // It requires a method called format that creates this string
    // When we use the ({}) syntax in Rust, it calls the format method to display the value
    // Basic types like integers, floats, and booleans can use the Display Trait, so we can print them with curly braces
    // However, for more complex types, it can be tricky to decide how to show them as text
    // Not every type supports the Display Trait; for example, arrays do not
    println!("{}", 5);
    println!("{}", 3.14);
    println!("{}", true);

    // The following line will result in an error because the array type does not implement the Display Trait
    // Therefore, we cannot use the ({}) format specifier to print it directly
    let array = ["Ivan", "Christine", "Aiori"];
    println!("{:?}", array);
}
