fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![11,22,33];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 'b', 'c'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
