use std::fs;

fn main() {
    let filename = "input.txt";
    let content = fs::read_to_string(filename).expect("couldnt read file");

    let arr: Vec<Vec<_>> = content
        .split("\n")
        .collect::<Vec<_>>()
        .iter()
        .filter(|a| !a.is_empty())
        .map(|s| {
            s.split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect()
        })
        .collect();

    fn is_valid_sequence(arr: &[i32]) -> bool {
        if arr.len() < 2 {
            return true; // A single element or empty array is trivially valid
        }

        let mut increasing = true;
        let mut decreasing = true;

        for window in arr.windows(2) {
            let diff = (window[1] as i32 - window[0] as i32).abs();
            if diff < 1 || diff > 3 {
                return false; // Difference constraint violated
            }
            if window[1] < window[0] {
                increasing = false;
            }
            if window[1] > window[0] {
                decreasing = false;
            }
        }

        increasing || decreasing // Valid if strictly increasing or decreasing
    }

    fn can_be_safe_with_dampener(arr: &[i32]) -> bool {
        if is_valid_sequence(arr) {
            return true;
        }

        for i in 0..arr.len() {
            let mut temp = arr.to_vec();
            temp.remove(i);
            if is_valid_sequence(&temp) {
                return true;
            }
        }
        return false;
    }

    let valid_seq = arr
        .iter()
        .filter(|arr| can_be_safe_with_dampener(arr))
        .count();
    println!("{:?}", valid_seq);
}
