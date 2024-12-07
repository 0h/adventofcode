use regex::Regex;
use std::fs;

fn main() {
    let filename = "input.txt";
    let content = fs::read_to_string(filename).unwrap();
    // let re = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();
    // let captured = re
    // .captures_iter(&content)
    // .filter_map(|cap| {
    // let x = cap[1].parse::<u32>().ok();
    // let y = cap[2].parse::<u32>().ok();
    // match (x, y) {
    // (Some(x), Some(y)) => Some(x * y),
    // _ => None,
    // }
    // })
    // .collect::<Vec<_>>()
    // .into_iter()
    // .reduce(|acc, e| acc + e);

    fn sum_enabled_multiplications(input: &str) -> i32 {
        let mut enabled = true;
        let mut sum = 0;

        // Regular expressions for instructions
        let re_instruction = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),\s*(\d+)\)").unwrap();

        for cap in re_instruction.captures_iter(input) {
            if let Some(instruction) = cap.get(0) {
                match instruction.as_str() {
                    "do()" => enabled = true,
                    "don't()" => enabled = false,
                    _ => {
                        if enabled {
                            // Parse and calculate multiplication
                            if let (Some(x), Some(y)) =
                                (cap[1].parse::<i32>().ok(), cap[2].parse::<i32>().ok())
                            {
                                sum += x * y;
                            }
                        }
                    }
                }
            }
        }

        sum
    }

    println!("{:?}", sum_enabled_multiplications(&content));
}
