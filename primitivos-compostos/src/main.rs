fn main() {
    println!("Tuplas");

    let tuple = ("Cleyson", 28, 100.51, true);

    println!("{:?}", tuple);
    println!("{}", tuple.0);
    println!("{}", tuple.1);
    println!("{}", tuple.2);
    println!("{}", tuple.3);

    let (a, b, c, d) = tuple;

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);

    println!("Arrays");

    let array = [1, 2, 3, 4, 5];

    println!("{:?}", array);
    println!("{}", array[0]);
    println!("{}", array[1]);
    println!("{}", array[2]);
    println!("{}", array[3]);
    println!("{}", array[4]);
}
