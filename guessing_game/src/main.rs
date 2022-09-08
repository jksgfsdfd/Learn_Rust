use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    //generate the secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("Secret Number is {secret_number}");
    //read number from user
    loop{
        println!("Enter your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("error....couldn't read input");

        //read line reads input in form of string hence guess is declared as string
        //println!("Your guess is {} ",guess); gives an extra new line in output as guess is now a
        //string and has the 'enter' typed stored in it
        
        //compare secret_number and guess...but guess is now a string and secret_number is i32

        //let guess : i32 = guess.trim().parse().expect("inputted character cannot be parsed to a number");
        //handling invalid input with expect crashes the program....expect() either just stops the program execution 
        //if guess.trim().parse() returns an error or returns the value if guess.trim().parse() returns OK

        let guess : i32 = match guess.trim().parse() {
            Ok(val) => val,
            Err(_) => continue
            //in case of any error just skip this iteration of loop
        };
        println!("Guessed Number : {guess}");
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Try bigger"),
            Ordering::Greater => println!("Try smaller"),
            Ordering::Equal => {
                println!("Congrats...You win");
                break;
            }
        }
    }
}
