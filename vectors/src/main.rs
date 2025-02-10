fn main() {
    let _v: Vec<i32> = Vec::new();
    let _v: Vec<i32> = vec![];
    let _v: Vec<i32> = [].to_vec();

    let _v = vec![1, 2, 3];
    let mut v = [1, 2, 3].to_vec();
    println!("{:?}", v);

    v.push(4);
    v.push(5);
    v.push(6);
    println!("{:?}", v);

    let last = v.pop();
    println!("{:?}, {:?}", v, last);

    let mut v: Vec<i32> = vec![];
    let last = v.pop();
    println!("{:?}, {:?}", v, last);

    let mut v = vec!["Cleyson", "Elton", "Wesley", "Amauri"];
    for item in &v {
        println!("{}", item);
    }

    let removed = v.remove(3);
    println!("{:?}, {}", v, removed);

    println!("{}", v[0]);

    println!("{:?}", v.get(0));
    println!("{:?}", v.get(10));

    let mut v = vec![1, 2, 3, 4, 5];
    println!("{:?}", v);
    v.reverse();
    println!("{:?}", v);

    let v = vec![String::from("Cleyson"), String::from("Elton")];
    let first = &v[0];
    println!("{:?} {}", v, first);
}
