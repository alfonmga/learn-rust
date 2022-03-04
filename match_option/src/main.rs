fn main() {
    let eleven = Some(11);
    let twelve = plus_one(eleven);
    dbg!(&twelve);
    assert_eq!(twelve, Some(12));

    let none = plus_one(None);
    dbg!(&none);
    assert_eq!(none, None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
