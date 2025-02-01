//! The dbg! Macro
// The (dbg!) macro is a debugging tool that allows you to easily print the value of an expression to the console
// It is particularly useful for inspecting the values of variables or expressions during runtime
// The (dbg!) macro returns the value of the expression it is given, allowing it to be used in a variety of contexts
// Once the desired output is observed, the (dbg!) macro can be removed to avoid unnecessary output in the final code

fn main() {
    let example_value = 42;
    // This will print "example_value: 42" to the console and return the value 42
    let debug_value = dbg!(example_value);
    // This will print "The value of example_value is: 42"
    println!("The value of example_value is: {:?}", debug_value);
}
