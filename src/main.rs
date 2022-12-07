use std::io;

fn main() {
    // use comments 
    // use comments multiples lines
    println!("Donner votre nom");
    let mut nom = String::new();
    io::stdin().read_line(&mut nom).expect("Erreur de lecture");
    // hello();
    show(nom);
}
 fn hello()
{   
    let apples = "Apples";
    println!("Tech for good {}", &apples);
    // &apples = "test";
    let mut age = String::new();
    let mut nom_complet = String::new();
    println!("Donner votre age ");
    io::stdin().read_line(&mut age).expect("Erreur de lecture ");
    println!("Donner votre nom complet");
    io::stdin().read_line(&mut nom_complet).expect("Erreur de lecture");

    println!("Votre nom est {} et votre age est {}", nom_complet, age);
}

fn show(msg : String){
    println!("Votre nom is {}", msg);
}
