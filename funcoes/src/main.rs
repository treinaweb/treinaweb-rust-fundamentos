fn saudacao() {
    println!("Olá, mundo");
}

fn saudacao2(name: &str) {
    println!("Olá, {}!", name);
}

fn sum(a: i32, b: i32) -> i32 {
    // return a + b;
    a + b
}

fn div(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        return 0.0;
    }
    a / b
}

fn saudacao3(name: Option<&str>) {
    let name = name.unwrap_or("Mundo");
    println!("Olá, {}!", name);
}

fn main() {
    saudacao();

    saudacao2("Cleyson");

    let result = sum(10, 5);
    println!("{}", result);

    let result = div(10.0, 0.0);
    println!("{}", result);

    let multiplica = |a: i32, b: i32| a * b;
    println!("{}", multiplica(10, 5));

    saudacao3(Some("Cleyson"));
    saudacao3(None);
}
