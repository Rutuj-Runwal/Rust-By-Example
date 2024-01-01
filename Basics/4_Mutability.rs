fn main(){
    let x = 2.1;
    println!("x = {} cannot be changed!" ,x);
    // x = 3.1;   Re-assinging leads to an error. Every variable is immutable in rust.

    // To make variable mutable use the `mut` keyword.
    let mut y = 3.1;
    println!("y = {} is mutable" ,y);
    y = 7.7; // Mutabnle variables can change values
    println!("Now, y = {}. Mutable variables can be changed" ,y);
    // y = "Some string" // Results in an error.Mutability is type restricted.

    let mut my_string = "Hi";
    print!("String: {}" ,my_string);
    my_string = "Hola";
    print!(" | Modified String: {}\n" ,my_string);

    let z = y;
    println!("z = {} is NOT mutable" ,z);
    // z = 77.7; --> ERROR


}