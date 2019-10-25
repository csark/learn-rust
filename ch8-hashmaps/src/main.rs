#![allow(unused_variables)]
fn main() {
    use std::collections::HashMap;

    let mut scores1 = HashMap::new();

    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);

    // OR
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("Team {} has {:#?} points", "Blue", scores1.get("Blue"));

    // Iterate!!
    for (key, value) in &scores1 {
        println!("Team {} has {} points!", key, value);
    }

    let mut scores = HashMap::new();

    // Overwrite a value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Only insert value is key is empty
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Look up current value and update based on value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
