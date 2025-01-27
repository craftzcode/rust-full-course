//! Strings & Raw Strings

fn main() {
    //* String Literals
    // They are enclosed in double quotes
    // They can contain any valid UTF-8 characters

    // For example:
    println!("Hello World!");

    //* Special Character
    // They are non-alphanumeric characters that have a specific meaning in programming

    // In this case, the newline character (\n) is used to move the cursor to the next line
    println!("Dear Ivan,\nHow have you been?");

    // Another special character we have is (\t) which is short for tab, this is a bit of indentation
    println!("\tOnce upon a time");

    // This (\"\") allows us to include double quotes in the output without ending the string.
    println!("Juliet said \"I love you Romeo\"");

    
    // The (\\) is a special character used as an escape character in strings
    // It allows us to include characters that would otherwise be interpreted differently
    // Such as the (\) itself. In this case, it is used to specify the file path
    let filepath = "C:\\Users\\craftzcode\\Desktop\\Dev\\Rust\\rust-full-course";

    println!("{filepath}");

    //? PS: (\\) this syntax can be a little bit annoying due to just the confusing aspect of it
    //? So another option that Rust offers us in this situation is using something called a (Raw String)

    //* Raw String
    // This will automatically ignore all special characters

    println!(r"Dear Ivan, \nHow have you been?");

    println!(r"\tOnce upon a time");

    println!(r#"Juliet said "I love you Romeo""#);

    println!(r"C:\Users\craftzcode\Desktop\Dev\Rust\rust-full-course");
}
