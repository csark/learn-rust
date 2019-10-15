#![allow(unused_variables)]
fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1,2,3]
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("The third element is {}", third),
    }
    // Bad Mutable/Immutable references, This will make the compiler angry
    // let w = vec![1,2,3,4,5];
    // let first = &w[0];
    //
    // w.push(6);
    //
    // println!("The first element is: {}", first);
    // End angry compiler

    // Loop over a vector
    let a = vec![100,32,57];
    for i in &a {
        println!("{}", i);
    }

    // Loop over a vector with ability to change contents
    let mut b = vec![100,32,57];
    for i in &mut b { // mutable reference of a vector
        println!("Old value i: {}", i);
        *i += 50; // dereferencing i to get to the value stored there
        println!("New value i: {}", i);
    }

    println!("Vector B values are now: ");
    for i in &mut b {
        println!("Value of i in b: {}", i);
    }

    // Using an enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
