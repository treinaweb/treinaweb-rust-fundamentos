const MINHA_CONSTANTE: i32 = 10;

fn main() {
    let mut name: &str = "Cleyson";
    let age = 28;

    let _minha_variavel = 10.0;

    println!("{} - {}", name, age);

    name = "Elton";

    println!("{}", name);

    println!("{}", MINHA_CONSTANTE);
}
