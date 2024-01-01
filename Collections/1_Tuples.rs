fn main(){
    // let TUPLE_NAME:(TYPES - ONE PER INDEX) = (comma,separated,values,...);
    let point: (i64,i64,i64) = (77,135,500);

    // Zero-indexed
    let x = point.0;
    let y = point.1;
    let z = point.2;

    // Destructured 
    let (a,b,c) = point;


    println!("Point: {} {} {}" ,x,y,z);
    println!("Point: {} {} {}" ,a,b,c);

    
    // Mutable tuple - values can be edited based on index.
    let mut origin:(i32,i32) = (0,-5);

    origin.1 = 0; // Update index-1 value to "0"

    println!("Origin - updated: ({} , {})" ,origin.0,origin.1);

    // Tuples cannot change sizes during runtime: So, `origin` is always a 2-element tuple

}