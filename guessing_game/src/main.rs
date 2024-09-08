use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// fn main() {
//     println!("Guess the number!");

//     // let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
//     // println!("Generated secret Num {secret_number}");

//     let mut guess: String = String::new();
//     // io::stdin().read_line(&mut guess).expect("Failed to read user input");

// loop {
//     println!("Please input your guess:");
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read user input");

//     let guess: u32 = guess.trim().parse().expect("Failed to read user input");

//     match guess.cmp(&secret_number) {
//         Ordering::Equal => println!("You won!"),
//         Ordering::Greater => println!("Too Big"),
//         Ordering::Less => println!("Too Small"),
//     }
// }

// }
