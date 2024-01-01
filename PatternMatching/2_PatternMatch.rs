fn main(){
    // Name of the enum: Color, Variation/Possible Values: Red,Green....
    enum Color{
        Red,
        Green,
        Yellow,
        Custom {red:u8,green:u8,blue:u8},
    }
    // Type of variables is Color.
    let current_color = Color::Red;
    let diff_color = Color::Custom {red:100,green:20,blue:75};

    // The `match` keyword is used for pattern matching:
    // match VariableToMatch{ //patterns go here... }
    //  It is similar to a `switch-case` in other languages
    // Does not requre `break` keyword.
    // Unlike switch-case in other languages, `match` can be used in patternmatching of custom variable types not just strings or numbers
    match current_color{
        Color::Green => {
            println!("It was green!");
        }
        Color::Yellow => {
            println!("It was Yellow!");
        }
        Color::Red => {
            println!("It was red");
        }
        Color::Custom {red,green,blue} => {
            println!("Color was: {} {} {}" ,red,green,blue);
        }
    }

    match diff_color{
        Color::Green => {
            println!("It was green!");
        }
        Color::Yellow => {
            println!("It was Yellow!");
        }
        Color::Red => {
            println!("It was red");
        }
        Color::Custom {red,green,blue} => {
            println!("Color was: {} {} {}" ,red,green,blue);
        }
        _ => {
            println!("underscore(_) is a catch all pattern - default case")
        }
    }
}