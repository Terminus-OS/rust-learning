use std::io;

fn main() {
    println!("Welcome to number guesser");
    println!("Enter the guess number");
    let mut guesser = String::new();
    io::stdin()
        .read_line(&mut guesser)
        .expect("Failed  to read line");
    println!("You guessed: {guesser}");
}
