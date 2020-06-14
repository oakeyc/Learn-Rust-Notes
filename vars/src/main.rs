fn main() {
    let mut x = 5;
    println!("X is: {}", x);
    x = 6;
    println!("X is: {}", x);

    let spaces = " ";
    let spaces = spaces.len();

    let guess: u32 = "42".parse().expect("Not a number!");

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z = 'Ƶ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
