fn main() {
    let s1 = String::from("Borrow me!"); // This string content is stored in the heap
    let s2 = s1; // We moved aka shallow copied the stack data (pointer, length and capacity) from s1 to s2 and now s1 has been invalidated
    let s3 = s2.clone(); // Deeply copied aka cloned the heap data from s2 to s3

    let x = 5;
    let y = x; // Integers that have a known size at compile time are stored entirely on the stack. No shallow copies or clone copies.
}
