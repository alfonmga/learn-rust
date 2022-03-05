fn main() {
    let config_max = Some(11u8);

    //match config_max {
    //    Some(max) => println!("The maximum is configured to be {}", max),
    //    _ => (),
    //}

    // if-let is shorter
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        dbg!(config_max);
        println!("`config_max` value isn't Some(11u8)");
    }
}
