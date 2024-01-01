fn main(){
    // Name of the enum: Color, Variation/Possible Values: Red,Green....
    enum Color{
        Red,
        Green,
        Yellow,
        CustomA {red:u8,green:u8,blue:u8},
        CustomB (u8,u8,u8)
    }
    // Type of variables is Color.
    let go = Color::Green;
    let stop = Color::Red;
    let slow_down = Color::Yellow;
    let purple = Color::CustomA { red:100,green:0,blue:250 };
}