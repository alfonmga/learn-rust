use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    dbg!(&secret_number);
}