fn main() {
    println!("°F 101 to °C: {}", fahrenheit_to_celsius(101));
}

fn fahrenheit_to_celsius(f: i32) -> i32 {
    (f-32)*5/9
}
