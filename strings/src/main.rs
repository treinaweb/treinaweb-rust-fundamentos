fn main() {
    // &'static str' - string literal, string escrita diretamento no código
    // &str - string slice
    // String - struct
    let mut name = "Cleyson";
    let mut name2 = String::new();

    name2.push('b');
    name2.push_str(" teste");
    println!("{}", name2);
}
