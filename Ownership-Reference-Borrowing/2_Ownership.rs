fn get_years() -> Vec<i32> {
    let years = vec![1995, 2000, 2005, 2010, 2015]; // alloc `years`

    return years; // transfer ownership of `years` to caller's scope ("move")
}

fn main() {
    let all_years = get_years(); // take ownership of returned value (`years`)
    // dealloc `all_years`
}