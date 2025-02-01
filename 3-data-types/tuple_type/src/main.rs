//! The Tuple Type
// In Rust, tuples are a way to group together a number of values with different types into a single compound type
// Tuples are useful when you need to return multiple values from a function, or when you need to group together
// A few values in a way that's more concise than using a struct

fn main() {
    let person = ("John Doe", 30, "Software Engineer");

    // This is how to access the tuple values
    // let name = person.0;
    // let age = person.1;
    // let occupation = person.2;

    //* Tuple Destructuring
    // An alternative to access the tuple values
    let (name, age, occupation) = person;

    println!("Name: {:?}", name);
    println!("Age: {:?}", age);
    println!("Occupation: {:?}", occupation);
}
