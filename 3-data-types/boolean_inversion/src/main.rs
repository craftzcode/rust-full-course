//! Boolean Inversion with !
// The exclamation mark (!) is used to invert a boolean value
// If a boolean variable is (true), applying (!) will make it false, and vice versa
// This is useful in conditional statements where you want to check the opposite of a condition

fn main() {
    let age: i32 = 13;
    
    // We then check if the age is greater than or equal to 17 to determine if the person can see a rated R movie
    // The result is stored in 'can_see_rated_r_movie', which will be false since 13 is less than 17
    let can_see_rated_r_movie: bool = age >= 17;
    
    // We use the exclamation mark (!) to invert the boolean value of 'can_see_rated_r_movie'
    // This means 'cannot_see_rated_r_movie' will be true, indicating that the person cannot see a rated R movie
    let cannot_see_rated_r_movie: bool = !can_see_rated_r_movie;

    println!("I am {age} years old.\nCan I not see this scary movie? {cannot_see_rated_r_movie}")
}
