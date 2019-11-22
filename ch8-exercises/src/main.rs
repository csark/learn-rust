fn mean(list: &[i32]) -> f32 {
    let mut total = 0;
    for i in list {
        total += i;
    }

    (total as f32)/(list.len() as f32)
}

fn median(list: &[i32]) -> i32 {
    let mid: i32 = (list.len() as i32) / 2;
    list[mid as usize]
}

fn mode(list: &[i32]) -> i32 {
    let example = 4;
    example
}

fn main() {
    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    v.push(4);
    v.push(5);
    v.push(3);
    v.push(2);
    v.push(1);
    v.push(4);

    println!("The mean is: {:.2}", mean(&v));
    v.sort();
    println!("{:?}", v);
    println!("The median is: {:?}", median(&v));
}
