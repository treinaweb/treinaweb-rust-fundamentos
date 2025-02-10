fn main() {
    for counter in 0..=10 {
        println!("Contador {}", counter);
    }

    let nums = [10, 20, 30, 40, 50];
    for num in nums {
        if num == 30 {
            continue;
        }

        println!("{}", num);
    }
}
