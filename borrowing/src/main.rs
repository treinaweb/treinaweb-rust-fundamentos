fn calculate_length(s: &String) -> usize {
    s.chars().count()
}

fn change(s: &mut String) {
    s.push_str(" Editado");
}

fn main() {
    let mut s = String::from("TreinaWeb");

    // let size = calculate_length(&s);

    // println!("Tamanho da string {} é {}", s, size);

    // change(&mut s);
    // println!("{}", s);

    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
}
