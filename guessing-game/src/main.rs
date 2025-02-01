use std::io; //We're calling the input/output library from the std(standard) library. This helps us in getting an input from the user

fn main () {
    
    println!("Guess the number!"); // Using println! (macro) to print output on terminal
    println!("Please input your guess:");

    let mut guess = String::new(); //let is used to create a variable.

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {}", guess);

    

}
