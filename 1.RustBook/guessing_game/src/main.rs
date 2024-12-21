/* --------------------Guessing Game-----------------------
The program will generate a random integer between 1 and 100.
It will then prompt the player to enter a guess. After a guess
is entered, the program will indicate whether the guess is
too low or too high. If the guess is correct, the game will
print a congratulatory message and exit.
-----------------------------------------------------------
*/

use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("---------Guess The Number!!!--------------");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
