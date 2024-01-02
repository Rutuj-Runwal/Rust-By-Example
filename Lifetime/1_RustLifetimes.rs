// A lifetime is the time between when a value is allocated and when it's deallocated.
// Lifetime annotations are a way to track dependencies between lifetimes.
// Note that 'a has appeared in several new places. This is a lifetime parameter, and it works like a type parameter for lifetimes. (Like type parameters, you can pick any name you want instead of a, such as 'b, 'foo, 'blah, etc.)
struct Releases<'a> {
    years: &'a [i64],
    eighties: &'a [i64],
    nineties: &'a [i64],
}

fn track<'a>(years: &'a [i64]) -> Releases<'a> {
    let eighties: &'a [i64] = &years[0..2];
    let nineties: &'a [i64] = &years[2..4];

    Releases {
        years,
        eighties,
        nineties,
    }
}

fn main() {
    let releases = {
        let all_years: Vec<i64> = 
            // alloc
            vec![
              1980, 1985, 1990, 1995, 2000, 2000
            ];

        track(&all_years)
    }; // dealloc

    for year in releases.eighties.iter() {
        println!("Eighties year: {}", year);
    }
}