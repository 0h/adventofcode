use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let sample = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    let content = fs::read_to_string("input.txt").unwrap();
    let workspace = content;
    let rules: Vec<(u32, u32)> = workspace
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('|'); // Split each line by '|'
            if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
                Some((
                    first.parse::<u32>().unwrap(),
                    second.parse::<u32>().unwrap(),
                ))
            } else {
                None
            }
        })
        .collect();

    let updates = workspace
        .split("\n\n")
        .last()
        .unwrap()
        .split("\n")
        .map(|a| {
            a.split(",")
                .map(|a| a.parse::<u32>().unwrap_or(0))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fn is_update_valid(rules: &Vec<(u32, u32)>, update: &Vec<u32>) -> bool {
        let page_positions: HashMap<u32, usize> = update
            .iter()
            .enumerate()
            .map(|(i, &page)| (page, i))
            .collect();

        for &(x, y) in rules {
            if let (Some(&x_pos), Some(&y_pos)) = (page_positions.get(&x), page_positions.get(&y)) {
                if x_pos > y_pos {
                    return false;
                }
            }
        }
        true
    }

    fn reorder_update(rules: &Vec<(u32, u32)>, update: &mut Vec<u32>) {
        let mut swapped = true;

        while swapped {
            swapped = false;

            for &(x, y) in rules {
                let x_pos = update.iter().position(|&page| page == x);
                let y_pos = update.iter().position(|&page| page == y);

                if let (Some(x_pos), Some(y_pos)) = (x_pos, y_pos) {
                    if x_pos > y_pos {
                        // Swap the pages to ensure the rule is satisfied
                        update.swap(x_pos, y_pos);
                        swapped = true;
                    }
                }
            }
        }
    }

    fn print_queue(updates: Vec<Vec<u32>>, rules: &Vec<(u32, u32)>) -> u32 {
        let mut result: Vec<u32> = Vec::new();
        for update in &updates {
            if is_update_valid(rules, update) {
                result.push(update[update.len() / 2]);
                // println!("{:?}", update[update.len() / 2]);
            }
        }
        result.into_iter().reduce(|a, s| a + s).unwrap()
        // println!("{:?}", result);
        // 32
    }

    let mut middle_values = Vec::new();

    for mut update in updates {
        if !is_update_valid(&rules, &update) {
            reorder_update(&rules, &mut update);
            // println!("{:?}", update);
            let middle = update[update.len() / 2];
            middle_values.push(middle);
        }
    }

    let result: u32 = middle_values.iter().sum();
    println!("Sum of middle page numbers: {}", result);

    // println!("{:?}", print_queue(updates, &rules));
}
