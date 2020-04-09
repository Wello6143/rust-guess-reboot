use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number game. The game will reset continiously until you win or press Ctrl+C");

    let secret_number = rand::thread_rng().gen_range(1, 101);


    loop{
        println!("Please input your guessing number:");

        let mut number = String::new();

        io::stdin().read_line(&mut number)
            .expect("Error while reading your number");

        let number: u32 = match number.trim().parse(){
            Ok(number) => number,
            Err(_) => {
                println!("It wasn't a number, try it again!\n"); 
                continue
            },
        };

        println!("So you guessed number: {}", number);

        match number.cmp(&secret_number){
            Ordering::Greater => println!("Too big! Try it again.\n"),
            Ordering::Less => println!("Too small! Try it again.\n"),
            Ordering::Equal => {
                println!("You win! Now the game will quit");
                break
            }
        }
    }
}
