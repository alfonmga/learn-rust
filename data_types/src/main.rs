fn main() {
    // Integer literals
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    // Floating-Point types
    let sixtyfourbitsf = 2.0; // f64
    let thirtytwobitsf: f32 = 3.0; // f32

    // Numeric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.8 / 32.2;
    let floored = 2 / 3;
    let remainder = 43 % 5;

    // Boolean type
    let t = true;
    let f: bool = false;

    // Character type
    let z = 'ℤ';
    let nature = '🐝';

    // Compound type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    println!("The tup variable compound value is: {}, {}, {}", x, y, z);
    
    let line_segment: (u32, u32) = (33,11);
    println!("Line segment value: {}x, {}y", line_segment.0, line_segment.1);

    // Array type
    let master_numbers: [u32; 3]  = [11,22,33];
    let week_days = ["Sunday", "Monday", "Tuesday", "Wendsday", "Thursday", "Friday", "Saturday"];
    let fixed_array_with_initial_val = [3;5];

    let eleven = master_numbers[0];
}
