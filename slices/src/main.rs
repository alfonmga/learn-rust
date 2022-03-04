fn main() {
    let master_numbers = [11,22,33];
    assert_eq!(&master_numbers[..1], &[11]);

    let s = String::from("hola mundo");
    let len = s.len();   

    let hola = &s[0..4];
    let mundo = &s[6..10];
    let mundo2 = &[6..];
    let holamundo = &[..];
    let holamundo2 = &[..len];

    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[0..6]);

    println!("first_word(s) return value: {}", first_word(&s));
    println!("first_word(my_string_literal) return value: {}", first_word(my_string_literal));
    println!("first_word(word) return value: {}", first_word(word));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
