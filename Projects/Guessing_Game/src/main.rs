use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    guess();
}
fn guess()  
{
    println!("******************");
    println!("Your Guessing Game");
    println!("******************");

    println!("Please enter your numbre guess 100 and 1");
    let secret_number = rand::thread_rng().gen_range(1..100);
    loop{
        let mut guess = String::new();
        println!("Imagine number guess !");
        io::stdin().read_line(&mut guess).expect("Imaginer un bon nombre");
        let guess :u32 = guess.trim().parse().expect("Un nombre est attendu!");
        println!("Votre nombre : {guess}");
        match guess.cmp(&secret_number) 
        {
            Ordering::Less => println!("Très petit"),
            Ordering::Greater => println!("Très grand"),
            Ordering::Equal => {
                println!("Vous avez gagné");
                break;
            },
        }
    }
}