use std::collections::HashMap;

fn mean(list: &[i32]) -> f64 {
    let mut total = 0;
    for i in list {
        total += i;
    }

    (total as f64)/(list.len() as f64)
}

fn median(list: &[i32]) -> f64 {
    // Original attempt by me (which works)
    // let mid: i32 = (list.len() as i32) / 2;
    // list[mid as usize]

    // Answer from someone online
    let len = list.len();
    let mid = len / 2;
    if len % 2 == 0 {
        mean(&list[(mid - 1)..(mid + 1)])
    } else {
        f64::from(list[mid])
    }
}

fn mode(list: &[i32]) -> i32 {
    let mut map = HashMap::new();

    let mut highest = 0;
    let mut highest_count = 0;
    for i in list {
        let count = map.entry(i).or_insert(0);
        *count += 1;
        if highest_count < *count {
            highest = *i;
            highest_count = *count
        }
    }
    highest
}

fn main() {
    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    // v.push(3);
    v.push(4);
    v.push(5);
    v.push(4);
    v.push(5);
    v.push(5);
    v.push(5);
    v.push(1);
    v.push(4);

    println!("The mean is: {:.2}", mean(&v));
    v.sort();
    // println!("{:?}", v);
    println!("The median is: {:.2}", median(&v));
    println!("The mode is: {}", mode(&v));
}
