use std::collections::HashSet;

fn get_item_priority(item: char) -> u8 {
    let mut item_value: u8 = item as u8;

    if item_value > 90 {
        item_value = item_value - 96;
    } else {
        item_value = item_value - 38;
    }
    item_value
}

fn check_for_badges(ruck_sacks: &[HashSet<char>]) -> char {
    let mut badge = None;
    for item in ruck_sacks[0].iter() {
        if ruck_sacks[1].get(item).is_some() && ruck_sacks[2].get(item).is_some() {
            badge = Some(item);
            break;
        }
    }
    *badge.unwrap()
}

fn main() {
    let rucksacks = include_str!("../input.txt")
        .lines()
        .map(|sack| -> HashSet<char> { HashSet::from_iter(sack.chars()) })
        .collect::<Vec<HashSet<char>>>();

    let rucksack_groups = rucksacks.chunks(3);

    let priority_sum: u32 = rucksack_groups
        .map(|group| {
            let badge = check_for_badges(group);
            get_item_priority(badge) as u32
        })
        .sum();

    println!("{priority_sum:?}");
}
