fn main() {
    // Conjunção (E)
    println!("{}", true && true);
    println!("{}", true && false);
    println!("{}", false && true);
    println!("{}", false && false);

    println!("--------------------------");

    // Disjunção (OU)
    println!("{}", true || true);
    println!("{}", true || false);
    println!("{}", false || true);
    println!("{}", false || false);

    println!("--------------------------");

    // Negação (NÃO)
    println!("{}", !true);
    println!("{}", !false);
}
