fn main(){
    // 3 i32 elements.
    let years:[i32;3] = [2023,2024,2025];
    println!("{}" ,years[0]);
    // Desturcture
    let [_,secondYr,thirdYr] = years; 
    println!("{} {}" ,secondYr,thirdYr);

    // Array iteration:
    // All array elements must have same time to be iterated: i32 here
    for yr in years.iter(){
        println!("Iter: {}" ,yr);
    }
}