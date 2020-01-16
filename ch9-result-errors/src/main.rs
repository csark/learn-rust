use std::io;
// use std::io::Read;
// use std::fs::File;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut s = String::new();
    //
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    //
    // Ok(s)

    fs::read_to_string("hello.txt")
}

fn main() {
    // let u = read_username_from_file();
    // let mut username = match u {
    //     Ok(s) => s,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error)
    //     },
    // };
    let mut username = read_username_from_file().expect("Failed to get username");
    username.pop();
    println!("Username is: {}", username);
}
