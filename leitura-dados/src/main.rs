use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let input: i32 = input.trim().parse().unwrap();

    println!("Você digitou: {}", input);
}
