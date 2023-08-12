use std::collections::HashSet;

// TODO:
// Split the lines into 3 line groups
// Check all three lines to find the shared item.

// IDEA: Could make each line a hashset and then iterate one and check if the other two have the
// same value.
/*fn find_matching_item(first_pocket: &str, second_pocket: &str, &str) -> char {
    let mut result = None;
    let set: HashSet<char> = first_pocket.into_iter().collect();

    for item in second_pocket {
        if set.contains(&item) {
            result = Some(item);
            break;
        }
    }
    result.unwrap()
}*/

fn get_item_priority(item: char) -> u8 {
    let mut item_value: u8 = item as u8;

    if item_value > 90 {
        item_value = item_value - 96;
    } else {
        item_value = item_value - 38;
    }
    item_value
}

fn main() {
    let rucksacks = include_str!("../input.txt")
        .lines()
        .map(|sack| -> HashSet<char> { HashSet::from_iter(sack.chars()) })
        .collect::<Vec<HashSet<char>>>();

    let rucksack_groups = rucksacks.chunks(3);

    //TODO: iterate over one of the hashsets per gourp and see if the other two contain the same
    //values.

}

/*fn main() {
    let item_sum: u32 = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let (first_pocket, second_pocket) = line.split_at(line.len() / 2);
            let first_pocket: Vec<char> = first_pocket.chars().collect();
            let second_pocket: Vec<char> = second_pocket.chars().collect();
            let item = find_matching_item(first_pocket, second_pocket);
            get_item_priority(item) as u32
        })
        .sum();
    println!("{}", item_sum);
}*/
