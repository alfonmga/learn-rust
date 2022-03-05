#[derive(Debug)]
enum SpreadsheetCell { Int(i32), Float(f64), Text(String), }

fn main() {
    let v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    let mut mutable_vector: Vec<u8> = vec![];
    mutable_vector.push(11);
    mutable_vector.push(22);

    // Reading Elements of Vectors

    let eleven = mutable_vector[0];
    assert_eq!(eleven, 11u8);

    match v.get(0) {
        Some(first) => println!("The first element is {}",first),
        None => println!("There is no third element."),
    }

    // Iterating Over the Values in a Vector
    let iterate_me = vec![25,50,75,100];
    for i in &v {
        println!("{}", i);
    }
   
    let mut iterate_me_mut = vec![25,50,75,100];
    for i in &mut iterate_me_mut {
        *i += 50;
    }
    dbg!(&iterate_me_mut);

    // Using an Enum to Store Mutiple Types
    let cell_row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    dbg!(cell_row);
}
