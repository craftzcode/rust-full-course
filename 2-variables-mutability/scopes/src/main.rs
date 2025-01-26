//! Scopes
// This is a regions in code where variables, functions, and other items are valid and accessible.
// A scope is created by a block of code enclosed in curly braces {}.
// Variables declared in an inner scope cannot be accessed from an outer scope, but the outer scope can access the inner scope.
// When a scope ends, any variables declared within it are dropped and their memory is freed.

fn main() {
    // For example:
    // This variable is in the outer scope
    let outer_value = 100;
    println!("Outer value: {outer_value}");

    // This block of code creates a new scope, which is a region of the program where items like variables are defined.
    // The {} brackets define the scope, and any variables declared within them are only accessible within that scope.
    //? PS: These {} brackets are not necessary for the program to function and have no technical reason or benefit, but they help us understand the concept of scopes better.
    {
        // Start of inner scope
        // This variable is only valid in this inner scope
        let inner_value = 200;
        println!("Inner value: {inner_value}");
        println!("Can access outer value from inner scope: {outer_value}");
    } // End of inner scope - inner_value is dropped here

    // This would cause an error, inner_value no longer exists:
    // println!("Inner value: {}", inner_value);

    println!("Outer value still accessible: {outer_value}");

    // Another example:
    // This is not an example of Variable Shadowing
    // Variable Shadowing only applies when I declare a variable with the same name in the same scope
    let variable_shadowing = 10; // Variable Shadowing Outer Scope

    {
        // This creates a separate independent variable within this new scope, within this area or region of code
        let variable_shadowing = 20; // Variable Shadowing Inner Scope
        println!("Variable Shadowing Inner Scope: {variable_shadowing}") // Variable Shadowing Inner Scope
    }

    println!("Variable Shadowing Outer Scope: {variable_shadowing}") // Variable Shadowing Outer Scope
} // End of outer scope - outer_value is dropped here
