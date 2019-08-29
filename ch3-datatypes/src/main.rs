fn main() {
    ////floats
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    ////Numric operations
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

    ////boolean
    let t = true;

    let f: bool = false; // with explicit type annotation

    ////Char (unicode)
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    ////Compound Types
    //tuples (fixed length)
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //Access tuple values through destructuring
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    //Access tuple values directly
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    //arrays (fixed length)
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a = [3;5]; // same as let a = [3, 3, 3, 3, 3]
    
}
