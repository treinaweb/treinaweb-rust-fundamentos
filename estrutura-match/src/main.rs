fn main() {
    let n = 3;

    // match n {
    //     1 | 2 => println!("O número é 1 ou 2"),
    //     3 => println!("O número é 3"),
    //     _ => println!("O número não é 1, 2 ou 3"),
    // }

    let message = match n {
        1 | 2 => "O número é 1 ou 2",
        3 => "O número é 3",
        _ => "O número não é 1, 2 ou 3",
    };

    println!("{}", message);
}
