use std::fs;

fn compare_elfs_inventory(elfs_inventory: &mut i32, largest_inventroies: &mut Vec<i32>) {
    for inventory in largest_inventroies {
        if *elfs_inventory > *inventory {
            let temp = *elfs_inventory;
            *elfs_inventory = *inventory;
            *inventory = temp;
        }
    }
}

fn main() {
    let mut largest_inventories: Vec<i32> = Vec::new();
    let mut current_inventory_sum: i32 = 0;

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        if line.is_empty() {
            if largest_inventories.len() < 3 {
                largest_inventories.push(current_inventory_sum);
            } else {
                compare_elfs_inventory(&mut current_inventory_sum, &mut largest_inventories);
            }
            current_inventory_sum = 0;
        } else {
            current_inventory_sum += line.parse::<i32>().unwrap();
        }
    }
    let sum: i32 = largest_inventories.iter().sum();
    println!("{}", sum);
}
