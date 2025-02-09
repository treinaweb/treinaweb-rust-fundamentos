fn main() {
    println!("Inteiros com sinal");

    let byte: i8 = 127;
    let short: i16 = 360;
    let int: i32 = 1000;
    let long: i64 = 10000;
    let very_long: i128 = 1000000;

    println!("{}", byte);
    println!("{}", short);
    println!("{}", int);
    println!("{}", long);
    println!("{}", very_long);

    println!("Inteiros sem sinal");

    let ubyte: u8 = 255;
    let ushort: u16 = 360;
    let uint: u32 = 1000;
    let ulong: u64 = 10000;
    let uvery_long: u128 = 1000000;

    println!("{}", ubyte);
    println!("{}", ushort);
    println!("{}", uint);
    println!("{}", ulong);
    println!("{}", uvery_long);

    println!("Fonto flutuante");

    let simple: f32 = 10.5;
    let double: f64 = 100.5;

    println!("{}", simple);
    println!("{}", double);

    println!("Booleanos");

    let t: bool = true;
    let f: bool = false;

    println!("{}", t);
    println!("{}", f);

    println!("Chars");

    let letter: char = 'c';
    let rust: char = '🦀';

    println!("{}", letter);
    println!("{}", rust);
}
