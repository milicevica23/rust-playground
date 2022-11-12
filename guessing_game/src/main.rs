use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop{
        println!("New secret number is ready!");
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("Please start to guess!");

        let mut count = 0;
        loop{
            
            let mut guess = String::new();

            io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            match guess.cmp(&secret_number){
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal =>  {
                    println!("You win!");
                    break;
                }
            } 
            count+=1;
            //println!("count is {count}");
        }
        println!("Bravo! You tried {count} times!");
    }   
}   
