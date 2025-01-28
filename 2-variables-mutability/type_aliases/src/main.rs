//! Type Alias
// Is a way to give an existing type a new name. It does not create a new type, but rather provides a more descriptive or convenient name for an existing type

// For example, in the following code, (Meters) is a type alias for (i32). This means that (Meters) can be used anywhere (i32) can be used, and vice versa
type Meters = i32;

fn main() {
    // Here, we use the (Meters) type alias to declare two variables, (mile_race_length) and (two_mile_race_length), which are both of type (i32) but with a more descriptive name
    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = 3200;
    println!("A one mile race is {mile_race_length} meters long and a two mile race is {two_mile_race_length} meters long.")
}
