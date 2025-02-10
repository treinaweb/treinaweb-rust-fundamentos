fn main() {
    // Criar
    let s1 = "Olá Mundo!";
    let s2 = String::from("Olá mundo");
    let s3 = s1.to_string();

    println!("{} {} {}", s1, s2, s3);

    // Add
    let mut s = String::from("Olá");
    s.push(' ');
    s.push_str("Mundo!");
    println!("{}", s);

    let s = format!("{} {}", s1, s2);
    println!("{}", s);

    // Tamanho
    let s = String::from("Olá");
    println!("{}", s.len()); // Contagem de bytes
    println!("{}", s.chars().count()); // Contagem de caracteres

    // Fatiamento
    let s1 = &s[0..2]; // Pode gerar panic
    println!("{}", s1);

    // Iterando
    for letter in s.chars() {
        println!("{}", letter);
    }

    // Transformações
    println!("{}", s.to_uppercase());
    println!("{}", s.to_lowercase());

    let s = String::from("     Rust     ");
    println!("{}", s.trim());

    let mut s = String::from("Rust!");
    s.pop();
    println!("{}", s);

    s.truncate(2);
    println!("{}", s);

    let s = "Aprendendo Rust";
    println!("{}", s.replace("Rust", "programação"));

    // Divisão
    let s = "Java,Rust,Python,Go";
    for language in s.split(",") {
        println!("{}", language);
    }

    // Verificação
    println!("{}", s.to_lowercase().contains("rust"));
}
