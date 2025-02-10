fn print_str(s: String) -> String {
    println!("{}", s);
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.chars().count();

    (s, length)
}

fn main() {
    let s = String::from("TreinaWeb");

    let (s, size) = calculate_length(s);

    println!("Tamanho da string {} é {}", s, size);
}
