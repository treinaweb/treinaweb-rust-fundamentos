fn main() {
    let a = 10;
    let b = 10;

    println!("{}", a == b);
    println!("{}", a != b);
    println!("{}", a > b);
    println!("{}", a < b);
    println!("{}", a >= b);
    println!("{}", a <= b);

    let d = 10.7;
    println!("{}", (a as f64) < d);
}
