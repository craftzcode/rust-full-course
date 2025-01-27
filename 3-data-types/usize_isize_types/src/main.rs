//! USIZE & ISIZE Types
// They are aliases for an existing integer type, they are not different or unique types
// They are nicknames for another existing Rust type
// It varies depending on what kind of computer is running the program

fn main() {
    /*
    For example, the (usize) unsigned integer is equivalent to a (u32) on a 32-bit system, 
    but it also equivalent to a (u64) on a 64-bit system, if your computer is 64-bit system

    If your computer is 64-bit, then it has more room for memory, so Rust will utilize a (u64)
    anytime we write a (usize)
    */
    let usize_type: usize = 55;
    let isize_type: isize = -15_000;
}
