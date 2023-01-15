use std::io::{BufWriter,stdout};
use ferris_says::say;
// Program

fn main() {
    // use hello world 
    test();
}

fn test(){
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
// fn hello() 
// {
//     let apples = "Apples";
//     println!("Tech for good {}", &apples);
//     &apples = "test";
//     let mut age = String::new();
//     let mut nom_complet = String::new();
//     println!("Donner votre age ");
//     io::stdin().read_line(&mut age).expect("Erreur de lecture ");
//     println!("Donner votre nom complet");
//     io::stdin().read_line(&mut nom_complet).expect("Erreur de lecture");
//     println!("Votre nom est {} et votre age est {}", nom_complet, age);
// }

// fn show(msg : String){
//     println!("Votre nom is {}", msg);
// }
