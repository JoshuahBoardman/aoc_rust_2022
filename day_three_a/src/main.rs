use std::collections::HashSet;

// lowercase letters subtract 96
// uppercase letters subtract 38

fn find_matching_item(first_pocket: Vec<char>, second_pocket: Vec<char>) -> char {
    let mut result = None;
    let set: HashSet<char> = first_pocket.into_iter().collect();

    for item in second_pocket {
        if set.contains(&item) {
            result = Some(item);
            break;
        }
    }
    result.unwrap()
}

fn get_item_value(item: char) -> u8 {
    let mut item_value: u8 = item as u8;

    if item_value > 90 {
        item_value = item_value - 96;
    } else {
        item_value = item_value - 38;
    }
    item_value
}

fn main() {
    let item_sum: u32 = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let (first_pocket, second_pocket) = line.split_at(line.len() / 2);
            let first_pocket: Vec<char> = first_pocket.chars().collect();
            let second_pocket: Vec<char> = second_pocket.chars().collect();
            let item = find_matching_item(first_pocket, second_pocket);
            get_item_value(item) as u32
        })
        .sum();
    println!("{}", item_sum);
}
