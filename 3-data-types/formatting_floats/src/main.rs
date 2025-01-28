//! Formatting Floats with Format Specifier
// A format specifier allows you to control how floating-point numbers are displayed.


fn main() {
    //* ({:.2}) Specifier
    // In this case, the specifier (.2) indicates that we want to format the number to two decimal places
    let pi: f64 = 3.1415926535897932384; 
    println!("The current value of pi is {pi:.2}"); 
}