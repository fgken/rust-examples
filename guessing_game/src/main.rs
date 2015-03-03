use std::old_io;
use std::rand;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = (rand::random::<u32>() % 100) + 1;

    //println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");
    
        let input = old_io::stdin()
                    .read_line()
                    .ok()
                    .expect("Failed to read line");
        let input_num: Result<u32, _> = input.trim().parse();
    
        let num = match input_num {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            },
        };
    
        match cmp(num, secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!!");
                return;
            },
        }
    
        //println!("you guessed is {}", input);
    }
}

fn cmp(a: u32, b: u32) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}
