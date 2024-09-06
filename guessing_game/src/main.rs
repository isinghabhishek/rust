use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");

    // let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    // println!("Generated secret Num {secret_number}");

    let mut guess: String = String::new();
    // io::stdin().read_line(&mut guess).expect("Failed to read user input");

loop {
    println!("Please input your guess:");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read user input");

    let guess: u32 = guess.trim().parse().expect("Failed to read user input");

    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You won!"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Less => println!("Too Small"),
    }
}

}
