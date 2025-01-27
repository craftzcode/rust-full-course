//! Underscore with Variables

fn main() {
    let apples = 50;
    let oranges = 14 + 6;

    // Variables with leading underscore (_) are used to indicate that they are intentionally unused
    // Rust will not warn about unused variables if they start with an underscore
    // In this case, _fruits is created but never used elsewhere in the code
    let _fruits = apples + oranges;
}
