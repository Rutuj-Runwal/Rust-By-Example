// This is the main function.
fn main() {
    // Statements here are executed when the compiled binary is called.
    // Binary compilation: rustc HelloWorld.rs -> ./HelloWorld

    // Print text to the console: in a new line
    println!("Hello World!");

    print!("Hi!");
    print!(" ,this will be printed in the same line"); // Without the /n character

    // Output: 

    // Hello World!
    // Hi! ,this will be printed in the same line
}