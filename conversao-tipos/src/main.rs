fn main() {
    let i1 = 129;
    let i2 = i1 as i8;

    println!("{} as {}", i1, i2);

    let f1 = i1 as f64;
    let f2 = 200.314342374;
    let f3 = f2 as f32;
    println!("{} as {}", i1, f1);
    println!("{} as {}", f2, f3);

    let f4 = 100.25;
    let s1 = f4.to_string();
    let s2 = "42";
    let i3 = s2.parse::<i32>().unwrap();
    println!("{} as {}", f4, s1);
    println!("{} as {}", s2, i3);

    let i4 = 100;
    let s3 = format!("{}", i4);
    println!("{}", s3);
}
