use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 11);
    scores.insert(String::from("Red"), 22);
    dbg!(&scores);

    // Vector to Hash Map
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    dbg!(scores2);

    // Hash Maps and Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // field_name and field_value are invalid at this point!

    // Accessing Values in a Hash Map
    let target_team_name = String::from("Red");
    let target_team_score = scores.get(&target_team_name);
    assert_eq!(target_team_score, Some(&22));

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating a Hash Map

    let mut scores0 = HashMap::new();
    scores0.insert(String::from("Alfon"), 33);
    scores0.insert(String::from("Alfon"), 11); // Overwriting a Value
    assert_eq!(scores0.get(&String::from("Alfon")), Some(&11));

    // 2. Only Inserting a Value if the Key Has no Value

    scores0.entry(String::from("Alfon")).or_insert(22);    
    assert_eq!(scores0.get(&String::from("Alfon")), Some(&11));

    scores0.entry(String::from("John")).or_insert(22);    
    assert_eq!(scores0.get(&String::from("John")), Some(&22));

    // 3. Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

}
