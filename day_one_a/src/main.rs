use std::fs;

fn main() {
    let mut largest_value: i32 = 0;
    let mut current_sum: i32 = 0;

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        if line.is_empty() {
            if current_sum > largest_value {
                largest_value = current_sum;
            }
            current_sum = 0;
        } else {
            current_sum += line.parse::<i32>().unwrap();
        }
    }

    println!("The largest sum is {}", largest_value);
}
