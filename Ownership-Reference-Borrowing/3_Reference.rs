fn print_years(years: &Vec<i64>) {
    for year in years.iter() {
        println!("Year: {}", year);
    }
}

fn main() {
    let years = vec![1995, 2000, 2005, 2010, 2015]; // alloc `years`

    print_years(&years); // temporarily give `print_years` access to `years`
    print_years(&years); // temporarily give `print_years` access to `years`

    // dealloc `years`
}