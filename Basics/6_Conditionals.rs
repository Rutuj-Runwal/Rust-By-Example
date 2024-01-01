fn main(){
    let dogs_better_than_cats = true;
    let no_of_cats = 2;
    let no_of_dogs = 1;

    if no_of_cats>1 && no_of_dogs>1{
        println!("Multiple Cats and dogs!");
    }else if no_of_cats>1 && no_of_dogs==1{
        println!("Multiple cats and a dog!");
    }else{
        println!("Else block");
    }

    // Stores the evaluated value in `message` variable
    let message = if no_of_cats>1 && no_of_dogs>1{
        "Multiple Cats and dogs!"
    }else if no_of_cats>1 && no_of_dogs==1{
        "Multiple cats and a dog!"
    }else{
        "Else block"
    };
    println!("{}",message);

    println!("> Are dogs better than cats?");
    if dogs_better_than_cats{
        println!("> YES!");
    }
}