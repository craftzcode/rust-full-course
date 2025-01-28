//! Casting Types with the as Keyword
// Casting is the process of converting a value from one type to another
// The (as) keyword is used for this purpose. It's important to note that casting 

fn main() {
    // However, it's important to ensure that the value is compatible with the target type to avoid data loss or overflow
    // The value of 50 is within the range of both (i8) (-128 to 127) and (u8) (0 to 255), 
    // which means it can be safely cast from (i32) to (i8) and (u8) without any data loss.
    let miles_away: i32 = 50; 
    let miles_away_i8 = miles_away as i8; 
    let miles_away_u8 = miles_away as u8; 

    // We can convert (f64) to (i32), but this will result in the loss of the decimal points
    let miles_away_float: f64 = 100.329032; 
    let miles_away_int = miles_away_float as i32; 
}
