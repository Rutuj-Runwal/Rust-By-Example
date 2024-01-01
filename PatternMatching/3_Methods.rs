fn main(){
    enum Color{
        Red,
        Green,
        Yellow,
        Custom {red:u8,green:u8,blue:u8},
    }

    // `impl` => implementation on Color enum,
    // Define functions inside the `impl` to create methods.
    impl Color{
        fn new(r:u8,g:u8,b:u8) -> Color{
            let clr =  Color::Custom {red:r,green:g,blue:b};
            return clr;
        }
    }

    // Associated function syntax - `new` function tied to Color enum namespace
    let purple = Color::new(100,10,208);

    // println!("Purlpe RGB: {} {} {}" ,r,g,b);
}