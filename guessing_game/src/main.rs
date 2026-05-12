use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*; 

fn main() {

    // Define a struct to represent a guess, ensuring that it is always valid
    pub struct Guess {
        value: u32,
    }

    impl Guess {
        pub fn new(value: u32) -> Result<Guess, String> {
            if value > 100 || value < 1{
                return Err(format!("Guess value must be between 1 and 100, got {}.", value));
            }
            Ok(Guess { value })
        }

        pub fn value(&self) -> u32 {
            self.value
        }
    }
    

    //GAME STARTS HERE
    println!("Welcome to the Guessing Game!"); 

    let secret_number = rand::thread_rng().gen_range(1, 101);
    print!("Secret number is: {} \n", secret_number); // For testing purposes, you can remove this line in production

    loop {
        println!("Please enter your guess (between 1 and 100): ");

  
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");


        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        let guess = Guess::new(guess);
        let guess = match guess {
            Ok(g) => g,
            Err(e) => {
                println!("{}", e.red().bold());
                continue;
            }
        };
        println!("You guessed: {}", guess.value());


        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("{}","Too low!".bright_red().bold()  ), 
            Ordering::Greater => println!("{}","Too high!".red().bold()),
            Ordering::Equal => {
                println!("{}","You got it!".green().italic());
                break;
            }
        }

    }
    
}


// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];


//     {
//         let first = &v[0]; // borrow the first element
//         println!("first = {}", first);
//     } // 'first' goes out of scope here, so we can modify 'v' again
// //     let third = &v[2]; // copy i32 value

// //     println!("third = {}", third);
//     v.push(6);
 
// }