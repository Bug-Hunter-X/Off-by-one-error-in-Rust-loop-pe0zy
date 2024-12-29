fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    for number in numbers {
        println!("Number: {}", number);
    }
}

//Alternative solution using iterators and while loop
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let mut iter = numbers.iter();

    while let Some(number) = iter.next() {
        println!("Number: {}", number);
    }
} 