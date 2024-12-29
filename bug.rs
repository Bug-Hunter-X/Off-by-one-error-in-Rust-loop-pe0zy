fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let mut index = 0;

    loop {
        if index >= numbers.len() {
            break;
        }

        println!("Number: {}", numbers[index]);
        index = index + 1;

    }
}