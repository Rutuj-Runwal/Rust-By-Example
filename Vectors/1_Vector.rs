fn main(){
    // Declare an i32 vector with `vec!` 
    let mut years: Vec<i32> = vec![2023,2024,2025];

    // Vec is dynamic - can add and remove elements in runtime 
    years.push(2026); // now vec "years" has 4 elements ending with 2026

    println!("Length: {}" , years.len());

    // Loop:
    for yr in years{
        print!("{} " ,yr);
    }  
}