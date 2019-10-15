fn main() {
    let mut s = String::new(); // create and empty string

    // Initialize string with data
    let data = "initial contents";
    let s = data.to_string();

    // or
    let s = "initial contents".to_string();

    // or
    let s = String::from("initial contents");

    // Concatenation
    let mut s = String::from("foo");
    s.push_str("bar");

    // or
    let mut s = String::from("lo");
    s.push('l');

    // or
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3: {}", s3);

    // or
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s: {}", s);

    // or simplified with 'format!' which doesn't take ownership of any parameters
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {}", s);

    // String slices
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    println!("s: {}", s);
}
