//! Boolean
// Is a type whose values can only be (true) or (false)

fn main() {
    let is_handsome: bool = true;
    let is_lazy: bool = false;
    println!("Ivan is handsome?: {is_handsome}\nIvan is lazy?: {is_lazy}");

    let legal_age: i32 = 18;
    let current_age: i32 = 25;
    let is_legal_age: current_age > legal_age;
    println!("Ivan is already in the legal age?: {is_legal_age}");
}
