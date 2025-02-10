fn main() {
    let a: i32 = 5;
    let b: i32 = 2;

    // Soma
    let sum = a + b;
    println!("{}", sum);

    // Subtração
    let sub = a - b;
    println!("{}", sub);

    // Multiplicação
    let mult = a * b;
    println!("{}", mult);

    // Divisão
    let div = a / b;
    println!("{}", div);

    // Modulo
    let modulo = a % b;
    println!("{}", modulo);

    let div2 = a as f64 / b as f64;
    println!("{}", div2);

    let a2: i8 = 5;
    let sum2 = a2 + b as i8;
    println!("{}", sum2);

    let mut c = 10;
    c += 10; // c = c + 10
    println!("{}", c);

    c -= 10; // c = c - 10
    println!("{}", c);

    c *= 10; // c = c * 10
    println!("{}", c);

    c /= 10; // c = c / 10
    println!("{}", c);

    c %= 10; // c = c % 10
    println!("{}", c);
}
