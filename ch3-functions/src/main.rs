// fn main() {
//     another_function(5, 6);
// }
//
// fn another_function(x: i32, y: i32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }

// fn main() {
//     let x = 5;
//
//     // y = 4 as the assignment of y uses an expression
//     let y = {
//         let x = 3;
//         x + 1
//     };
//
//     println!("The value of y is: {}", y);
// }

// // This function returns the number 5 as an 32 bit integer
// // Adding a semi colon would make this a statement and the
// // function would not work.
// // STATEMENTS DON'T EVALUATE TO A VALUE, EXPRESSIONS DO
// fn five() -> i32 {
//     5
// }
//
// fn main() {
//     let x = five();
//
//     println!("The value of x is: {}", x);
// }

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
