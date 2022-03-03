fn main() {
    hello_world_function();
    sum_master_number(22);
    print_labeled_measurement(33, 'h');
    println!("Eleven: {}", eleven());
    println!("plus_one(100): {}", plus_one(100));
}

fn hello_world_function() {
    println!("Hello there!");
}
fn sum_master_number(x: u8) {
   println!("The value of 11 + {} is: {}", x, 11+x);
}
fn print_labeled_measurement(value: u32, unit_label: char) {
   println!("The measurement is: {}{}", value, unit_label);
}
fn eleven() -> u8 {
   11
}
fn plus_one(x: i32) -> i32 {
   x + 1
}
