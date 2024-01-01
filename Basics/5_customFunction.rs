fn main(){
    let a = 7.7;
    let b = 1;
    let result = multiply_both_converted_types(a,b);
    println!("Result of {} * {}: {}" ,a ,b ,result);

} 

// Custom function declaration
// Read as:
// `multiply_both` takes two f64[64-bit float - 8bytes storage] arguments and returns an f64
fn multiply_both(num1:f64,num2:f64) -> f64{
    return num1 * num2;
}

fn multiply_both_converted_types(num1:f64,num2:u8) -> f64{
    return num1 * num2 as f64;
}