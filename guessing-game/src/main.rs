use std::io; //We're calling the input/output library from the std(standard) library. This helps us in getting an input from the user
use rand::Rng; //Used for generating random numbers
use std::cmp::Ordering; //The Ordering type is an enum with Less, Greater, and Equal variants.

fn main () {
    
    println!("Guess the number!"); // Using println! (macro) to print output on terminal

    let secret_number = rand::thread_rng().gen_range(1..=100); //In this line of code we generated a random number between the range of 1-100


    loop{
        println!("Please input your guess:");

        let mut guess = String::new(); //In this line we basically created a variables named String which is currently empty.

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!"); //Shadowing the previous guess variable by declaring it as a u32 number in which the value of the previous guess is stored which is pasrsed and trimmed off any whitespaces or newlines.
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {  //Quits the program as soon as the user guesses the correct number
                 println!("You win!");
                 break;
            }
        }
    }
    

}
