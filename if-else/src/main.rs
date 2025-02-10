use std::io::{self, Write};

fn main() {
    let mut age = String::new();
    print!("Digite sua idade: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut age).unwrap();
    let age = age.trim().parse::<i32>().unwrap();

    // if age >= 18 {
    //     println!("Você é maior de idade");
    // } else if age >= 16 {
    //     println!("Você é menor de idade, mas já pode votar");
    // } else {
    //     println!("Você é menor de idade");
    // };

    let message = if age >= 18 {
        "Você é maior de idade"
    } else if age >= 16 {
        "Você é menor de idade, mas já pode votar"
    } else {
        "Você é menor de idade"
    };

    println!("{}", message);
}
