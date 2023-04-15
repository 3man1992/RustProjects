// A simple program to generate random numbers
use rand::Rng; // Import the Rng trait from the rand crate

fn main() {
    let number = rand::thread_rng().gen_range(1..=100); // Generate a random number between 1 and 100
    println!("Your random number is: {}", number);
}
