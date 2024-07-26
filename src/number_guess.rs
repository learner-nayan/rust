use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn number_guess(){
    let secret_number = rand::thread_rng().gen_range(1..=101);
    println!("The secret number is {}",secret_number);

    println!("Guess the number");

    loop {
        let mut guess = String::new();
        println!("Enter the number:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}",guess);
        // println!("The secret number is {}",secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed low!"),
            Ordering::Greater => println!("You guessed high!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }

}
