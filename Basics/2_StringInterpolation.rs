fn main(){
    // String formatting:

    // Numeric Argument based: "James" = 0, "Bond" = 1
    println!("My name is {1}, {0} {1}.","James","Bond"); 
    // Output: My name is Bond, James Bond!

    // Named Argument based
    println!("{alice}, this is {bob}. {bob}, this is {alice}",alice="Alice",bob="Bob");

    let subject = "World!";
    let greeting = format!("Hello, {}!",subject); // `format!() - Inplace comparison without pritning to std console 
    print!("{}",greeting);
}