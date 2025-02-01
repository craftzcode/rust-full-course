//! Traits
// A trait is a contract that requires that a type support one or more methods
// Traits establish consistency between types; methods that represent the same behavior have the same name
// When a type opts in to honoring a trait's requirement, we say the type implements the trait
// Types can vary in their implementation but still implement the same trait
// A type can choose to opting in to implementing a trait
// A type can implement multiple traits. There are hundreds of traits available in Rust
// A trait is called an interface or protocol in other programming languages

fn main() {
    //? The Display Trait
    // The Display Trait allows a type to be shown as a simple, easy-to-read string
    // It requires a method called format that creates this string
    // When we use the ({}) syntax in Rust, it calls the format method to display the value
    // Basic types like integers, floats, and booleans can use the Display Trait, so we can print them with curly braces
    // However, for more complex types, it can be tricky to decide how to show them as text
    // Not every type supports the Display Trait; for example, arrays do not

    //* The {} Format Specifier
    // The {} format specifier is used to format a value as a string
    // It is a placeholder that is replaced by the value of the variable or expression inside the curly braces
    // This specifier is commonly used to display simple values like integers, floats, and booleans in a human-readable format
    println!("{}", 5);
    println!("{}", 3.14);
    println!("{}", true);

    // The following line will result in an error because the array type does not implement the Display Trait
    // Therefore, we cannot use the ({}) format specifier to print it directly
    let array = ["Ivan", "Christine", "Aiori"];
    // This line will result in an error because the array type does not implement the Display Trait by default
    // The Display Trait is required for a type to be formatted as a string using the ({}) placeholder in (println!)
    // Since arrays do not implement Display, we cannot use the ({}) placeholder to print them directly
    // println!("{}", array);

    //? The Debug Trait
    // The Debug Trait allows a type to be printed for debugging purposes
    // It requires a method called fmt that produces a formatted, debug-friendly string representation
    // When we use the {:?} or {:#?} syntax in Rust, it calls the fmt method to display the value
    // Basic types like integers, floats, and booleans implement Debug by default, so we can print them with {:?}
    // For more complex types like structs or enums, we often derive or manually implement the Debug trait to make them printable
    // The Debug trait is typically used with print statements to inspect the internal state of variables
    // It helps with debugging and understanding how values are stored in memory, especially for custom types

    //* {:?} Format Specifier
    // The ({:?}) format specifier is used to format a value using the Debug trait
    // It is typically used for debugging purposes to display the internal state of a value
    // This specifier is particularly useful for complex types like structs or enums that do not implement the Display trait
    // When used with (println!), it calls the fmt method of the Debug trait to display the value in a debug-friendly format
    let array = ["Ivan", "Christine", "Aiori"];
    // println!("{:?}", array);
    println!("{array:?}");

    //* {:#?} Format Specifier
    // The ({:#?}) format specifier is used to format a value using the Debug trait with pretty-printing
    // It is similar to the ({:?}) format specifier, but it adds extra formatting to make the output more readable
    // This specifier is particularly useful for complex types like structs or enums that do not implement the Display trait
    // When used with (println!), it calls the fmt method of the Debug trait to display the value in a debug-friendly format
    // The difference is that the output is prettified, making it easier to read and understand
    let array = ["Ivan", "Christine", "Aiori"];
    println!("{array:#?}");

    //* {:.2} Format Specifier
    // The ({:.2}) format specifier is used to format a floating-point number to a specific number of decimal places
    // In this case, the specifier (.2) indicates that we want to format the number to two decimal places
    // This is particularly useful for displaying numerical values in a human-readable format
    let pi: f64 = 3.1415926535897932384;
    println!("The current value of pi is {pi:.2}");
}
