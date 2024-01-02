fn main() {
    let numbers = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24];

    let sum_of_nums = sum(numbers.as_slice());
    let product_of_nums = product(numbers.as_slice());
    let average_of_nums = average(numbers.as_slice());

    println!("Sum of these numbers: {}", sum_of_nums);
    println!("Product of these numbers: {}", product_of_nums);
    println!("Average of these numbers: {}", average_of_nums);

    let other_numbers = vec![1, 2, 3, 4, 5, 6];
    let (slice1, slice2) = first_three(&numbers, &other_numbers);

    println!("The first three elements in `slice1` are:");

    for num in slice1 {
        println!("• {}", num);
    }

    println!("The first three elements in `slice2` are:");

    for num in slice2 {
        println!("• {}", num);
    }
}

fn sum(numbers: &[i64]) -> i64 {
    let mut total = 0;

    for num in numbers.iter() {
        total += num;
    }

    total
}

fn product(numbers: &[i64]) -> i64 {
    let mut total = 1;

    for num in numbers.iter() {
        total *= num;
    }

    total
}

fn average(numbers: &[i64]) -> i64 {
    let length = numbers.len() as i64;

    sum(numbers) / length
}

fn first_three<'lft_a,'lft_b>(numbers1: &'lft_a Vec<i64>, numbers2: &'lft_b Vec<i64>) -> (&'lft_a [i64], &'lft_b [i64]) {
    let num1 = &numbers1[0..3];
    let num2 = &numbers2[0..3];
    // 👉 TODO Return two slices, one containing the first 3 elements in
    //    numbers1, and the other containing the first 3 elements in numbers2.
    //
    // 💡 TIP 1: You can get slices like this using this syntax:
    //
    //        &numbers[0..3]
    //
    // 💡 TIP 2: A slice type with a lifetime annotation looks like this:
    //
    //        &'a [i64]
    //
    // 💡 TIP 3: To introduce lifetime annotations to this function, change its
    //    declaration to something like the following, depending on how many
    //    different lifetimes you want to use:
    //
    //        fn first_three<'a>
    //
    //        fn first_three<'a, 'b>
    //
    //        fn first_three<'a, 'b, 'c>
    (num1, num2)
}