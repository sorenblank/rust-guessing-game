// This is a simple number guessing game where the computer generates
// a random number and the user tries to guess it.

// Import required standard library modules
use std::io;          // For handling input/output operations
use std::cmp::Ordering;   // For comparing numbers
use rand::Rng;        // For generating random numbers

fn main() {
    // Print welcome message
    println!("Guess the number!");

    // Generate a random number between 1 and 100 (inclusive)
    // thread_rng() gives us a random number generator local to the current thread
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Main game loop - continues until the player guesses correctly
    loop {
        println!("Please input your guess.");

        // Create a new empty String to store the user's input
        // The 'mut' keyword makes the variable mutable, allowing us to modify it
        let mut guess = String::new();

        // Read a line of user input
        // read_line() appends the user's input to our string
        // expect() will crash the program with the given message if an error occurs
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the string input to a number (u32 = unsigned 32-bit integer)
        // trim() removes whitespace and newlines
        // parse() converts the string to another type
        // The match expression handles the Result returned by parse()
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,         // If parsing succeeds, use the number
            Err(_) => {            // If parsing fails (e.g., if input wasn't a number)
                println!("Please enter a valid number");
                continue;          // Skip the rest of this loop iteration
            },
        };

        // Show what the user guessed
        println!("You guessed: {}", guess);

        // Compare the guess with the secret number
        // cmp returns an Ordering (Less, Greater, or Equal)
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),    // Guess is smaller than secret number
            Ordering::Greater => println!("Too big!"),   // Guess is larger than secret number
            Ordering::Equal => {                         // Guess matches secret number
                println!("You win!");
                break;  // Exit the loop (and the game)
            }
        }
    }
}