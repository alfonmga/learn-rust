use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap(); // extracts value result or panics if `hello.txt` doesn't exists
    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // Same as above but we specified an error message in its call to `panic!`
}
