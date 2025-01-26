//! Compiler Directive
// Is an annotation that tells the compiler how to parse the source code

// The following line is a global compiler directive that allows unused variables throughout the entire file. This means that the compiler will not generate warnings for unused variables.
// #![allow(unused_variables)]

fn main() {
    // The following lines demonstrate the use of a local compiler directive to allow unused variables within the (main) function. This directive is applied only to the scope of the (main) function.
    #[allow(unused_variables)]
    let first_unused_variable = "THIS WILL NOT HAVE A WARNING IN THE COMPILER.";

    #[allow(unused_variables)]
    let second_unused_variable = "THIS WILL NOT HAVE A WARNING IN THE COMPILER.";
}

// The following lines demonstrate the use of a local compiler directive to allow unused variables within the (main) function. This directive is applied only to the scope of the (main) function, differing from the global compiler directive that affects the entire file.
// #[allow(unused_variables)]
// fn main() {
//     let first_unused_variable = "THIS WILL NOT HAVE A WARNING IN THE COMPILER.";
//     let second_unused_variable = "THIS WILL NOT HAVE A WARNING IN THE COMPILER.";
// }
