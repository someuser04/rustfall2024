use std::io;

// Function to check if guess matches secret
fn check_guess(guess: i32, secret: i32) -> i32
{   
    // If guess == secret return 0
    if guess == secret
    {
        0
    }
    // If guess too high return 1
    else if guess > secret
    {
        1
    }
    // If guess is too low return -1
    else 
    {
        -1
    }
}

fn main() {
    // Declare mutable variable 'secret' hard code the value '44'
    let secret: i32 = 44;
    let mut guess_count = 0;
    // println!("{}", secret);
    // Setting up infinite loop
    loop
    {   
        // Mutable variable to store user input
        let mut guess = String::new();
        
        // Read user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read input");
        // Convert string to integer
        let guess: i32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) =>
            {
                println!("Please enter a valid number");
                continue;
            }
        };
        guess_count += 1;
        let result = check_guess(guess, secret);
        //  Can Use match to check_guess 
        // If guess correct break loop
    //     match check_guess(guess, secret){
    //     0 => {
    //         println!("You guessed the number :)!!");
    //         break; 
    //     }
    //     1 => println!("Too high, try again!"),
    //     -1 => println!("Too low, try again!"),
    //     _ => println!("Error"),
    // }
    // Check result and print messages 
        if result == 0
        {
            println!("You guessed the number :)!!");
            break; // Break out loop if guess is correct
        }
        else if result == 1 
        {
            println!("Too high, try again!");
        }
        else if result == -1
        {
            println!("Too low, try again!");
        }
    }
    // Print out message using the guess_count value to show how many attempts it took to guess
    println!("Took {} guesses for you to guess the number congratulations !!", guess_count);
}
