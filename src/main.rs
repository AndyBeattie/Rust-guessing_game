use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess The Number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    loop {
        println!("Please Input Your Guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed To read Line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed: {}", guess);

        //print!("The Secret Number was {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your Guess Was To Low"),
            Ordering::Greater => println!("Your Guess Was To High"),
            Ordering::Equal => {
                println!("Great Guess!!!");
                break;
            }
        }

        let (_x, _y, _z) = tup; //deconstructing a tuple into separate variables, variables are  prefixed with an underscore as they are currently not used

        print!("Values In Tuple Are : {} {} {}", tup.0, tup.1, tup.2); //accsessing individual elements of a tuple directly via numbered elements
    }
}
