//! Floating Point Types
// There are two primary floating point types: (f32) and (f64)
// (f32) is a 32-bit floating point type, which can represent numbers with up to 7 decimal digits of precision
// (f64) is a 64-bit floating point type, which can represent numbers with up to 15 decimal digits of precision

//? Note: Using (f32) we use less memory, but we lose precision, we lose accuracy
//? Because the result may be rounded to fit the precision limits of the floating point type 
//? For example, the (f32) type can represent numbers with up to 7 decimal digits of precision
//? If a number has more than 7 decimal digits, it will be rounded to fit this limit

fn main() {
    let pi_f64: f64 = 3.141592653589793; 
    println!("The current value of pi (f64) is {pi_f64}");

    let pi_f32: f32 = 3.1415927; 
    println!("The current value of pi (f32) is {pi_f32}");
    
}
