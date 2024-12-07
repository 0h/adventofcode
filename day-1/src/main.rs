use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("couldnt read file");
    let fmt = contents.replace("\n", "   ");
    let fmt2 = fmt
        .split("   ")
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    let mut vector1 = Vec::new();
    let mut vector2 = Vec::new();

    for (i, j) in fmt2.iter().enumerate() {
        if i % 2 == 0 {
            vector2.push(j);
        } else {
            vector1.push(j);
        }
    }
    //    println!("{:?}", fmt2);

    vector1.sort();
    vector2.sort();
    //    println!("{:?}{:?}", vector1, vector2);

    let mut result = Vec::new();

    for i in 0..vector1.len() {
        if vector1[i] > vector2[i] {
            result.push(vector1[i] - vector2[i]);
        } else {
            result.push(vector2[i] - vector1[i]);
        }
    }
    //    println!("{:?}", result.into_iter().reduce(|a, s| a + s).unwrap());
    let mut occur = vec![];
    for (i, num) in vector1.iter().enumerate() {
        let count = vector2.iter().filter(|a| *a == num).count();
        occur.push(count)
    }
    let mulsum: i32 = vector1
        .iter()
        .zip(occur.iter())
        .map(|(&x, &y)| x * (y as i32))
        .sum();
    println!("{:?}", mulsum);
}
