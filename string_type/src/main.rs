extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut s = String::new(); // empty new `String`

    // Create a `String` from a string literal
    let s_literal = "initial contents";
    let s_literal_to_string = s_literal.to_string();

    // Storing greetings in different languages UTF-8 encoded
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Appending to `String` with `push_str` and `push`
    let mut name = String::from("Alfon");
    name.push_str("so");
    println!("Hello {}!", name);
    
    // Adding one character to a `String` value using `push`
    let mut s0 = String::from("lo");
    s0.push('l');
    dbg!(s0);

    // Concatenation with + Operator or the format! Macro
    let concat0 = String::from("Hello, ");
    let concat1 = String::from("world!");
    let concat2 = concat0 + &concat1;
    dbg!(concat2);
    
    let tic = String::from("tic");  
    let tac = String::from("tac");  
    let toe = String::from("toe");
    let tictactoe = format!("{}-{}-{}", tic, tac, toe);

   // Internal Representation
   let hola = String::from("hola");
   assert_eq!(hola.len(), 4); // 4 bytes aka each character is 1 byte.
   let hello_cyrillic = String::from("Здравствуйте");
   assert_eq!(hello_cyrillic.len(), 24); // This string takes 24 bytes.

   // Bytes and Scalar Values and Grapheme Clusters
   let namaste = String::from("नमस्ते");
   dbg!(namaste.as_bytes()); // namaste bytes representation
   dbg!(namaste.chars()); // namaste `chars` representation
   dbg!(UnicodeSegmentation::graphemes(namaste.as_str(), true).collect::<Vec<&str>>()); // namaste Grapheme Cluster representation

   // Slicing Strings
   let slicing_target = "Здравствуйте";
   // let sliced_target = &slicing_target[0..1]; // this panics because byte index 1 is not a char boundary
   let sliced_target = &slicing_target[0..4];

   // Iterating Over Strings
   for c in "नमस्ते".chars() {
       println!("{} char", c);
   }
   for b in "नमस्ते".bytes() {
       println!("{} byte", b);
   }
}
