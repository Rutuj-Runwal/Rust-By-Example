fn main() {
    let mut city_names = vec!["Pythonia", "Javasburg", "C by the Sea", "Rustville"];

    // The pop() method returns a Option<T> enum parameter.
    // That can either be Some(T) - "some value"
    // OR can be "None" - if the vector is empty
    let last_city = match city_names.pop(){
        Some(inner_value) => {
            inner_value
        }
        None => {""}
    };

    if last_city.starts_with("R") {
        println!("“{}” starts with an R!", last_city);
    } else {
        println!("“{}” doesn't start with R", last_city);
    }

    city_names.push(last_city);

    println!("Here is the full list of cities:");
    for city in city_names.iter(){
        print!("{} " ,city);
    }
}