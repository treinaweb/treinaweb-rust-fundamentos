fn main() {
    let mut counter = 0;

    loop {
        println!("Contador: {}", counter);
        counter += 1;

        if counter >= 10 {
            break;
        }
    }
}
