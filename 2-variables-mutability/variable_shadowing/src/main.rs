//! Variable Shadowing
// Variable shadowing allows us to reuse the same variable name
// Each new declaration "shadows" the previous one, changing both the value and potentially the type

fn main() {
    // First declaration - string type
    let grams_of_protein = "100.345";
    println!("First (String): {grams_of_protein}");

    // Second declaration - shadows with float type
    let grams_of_protein = 100.345;
    println!("Second (f64): {grams_of_protein}");

    // Third declaration - shadows with integer type
    let grams_of_protein = 100;
    println!("Third (i32): {grams_of_protein}");

    //? PS: If we had used (let mut grams_of_protein) instead of shadowing
    //? We couldn't change the data type
    //? Mutation only allows changing values of the same type

    // For example, this wouldn't work:
    // let mut grams_of_protein = "100.345";    // String type
    // grams_of_protein = 100.345;              // Error! Can't change from String to f64
    // grams_of_protein = 100;                  // Error! Can't change from String to i32

    // The final value of grams_of_protein is 100 (integer)
    println!("{grams_of_protein}");
}
