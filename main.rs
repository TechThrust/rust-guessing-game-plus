extern crate rand;

use std::io;
use rand::prelude::*;

fn main() {
    println!("Welcome to Habeeb's number guessing game, Do you want to activate endless mode? (Yes = 1, No = 0)");
    let mut endless_mode = String::new();
    io::stdin().read_line(&mut endless_mode).expect("There was an error");
    let endless_mode_num = endless_mode.trim().parse::<i32>().unwrap();
    println!("Choose a number from 1 to 100");
    let mut up = 101;
    let mut guesses = 0;
    let won: bool = false;
    let mut rng = rand::thread_rng();
    let mut random_number: i32 = rng.gen_range(1, up);
    while won == false {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("There was an error");
        let guess = guess.trim().parse::<i32>().unwrap();
        let checknum = random_number - guess;
        guesses = guesses+1;
        if checknum == 0 {
            println!("Congractulations, you got the correct number with {} guesses", guesses);
            if endless_mode_num != 1 {
                println!("Do you want to keep playing: Yes = 1, No: 0", );
                let mut playagain = String::new();
                io::stdin().read_line(&mut playagain).expect("There was an error");
                let playagain_num = playagain.trim().parse::<i32>().unwrap();
                if playagain_num != 1 {
                    break;
                }
            }
            up = up+50;
            println!("Try and guess the new number: Choose a number from 1 to {}", up-1);
            rng = rand::thread_rng();
            random_number = rng.gen_range(1, up);
            guesses = 0;
        } else if checknum > 0 {
            println!("Too low");
        } else if checknum < 0 {
            println!("Too high")
        }
}
    
}
