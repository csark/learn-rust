
#![allow(unused_variables)]
fn main() {
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // r1 goes out of scope here, so we can make a new reference with no problems.

// println!("{}", r1);

let r2 = &mut s;

println!("{}", r2);
}
