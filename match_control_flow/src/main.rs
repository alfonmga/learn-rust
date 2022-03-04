#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match &self {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }
}


fn main() {
    let coin = Coin::Penny;
    let coin_value = coin.value_in_cents();
    dbg!(coin_value);

    let alaska_coin = Coin::Quarter(UsState::Alaska);
    let alaska_coin_value = alaska_coin.value_in_cents();
    dbg!(alaska_coin_value);
}
