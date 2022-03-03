fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition `number < 5` was true");
    } else {
        println!("Condition `number < 5` was false");
    }

    if number != 0 {
        println!("`number` was something other than zero");
    }

    if number % 4 == 0 {
        println!("`number` is divisible by 4");
    } else if number % 3 == 0 {
        println!("`number` is divisible by 3");
    }

    let condition = true;
    let inline_condition = if condition { 5 } else { 6 };
    println!("The value of `inline_condition`: {}", inline_condition);
}
