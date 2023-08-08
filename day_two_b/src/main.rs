use std::fs::read_to_string;

fn get_opponents_value(opponent_choice: &str) -> i32 {
    match opponent_choice {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        &_ => panic!(),
    }
}

fn get_our_value(our_choice: &str, opponent_value: &i32) -> i32 {
    match our_choice {
        "X" => {
            if *opponent_value == 1 {
                3
            } else {
                *opponent_value - 1
            }
        }
        "Y" => *opponent_value + 3,
        "Z" => {
            if *opponent_value == 3 {
                7
            } else {
                *opponent_value + 7
            }
        }
        &_ => panic!(),
    }
}

fn main() {
    let total: i32 = read_to_string("input.txt")
        .unwrap()
        .lines()
        .filter_map(|event| {
                let (opponent_choice, our_choice) = event.split_once(" ").unwrap();
                let opponent_value = get_opponents_value(&opponent_choice);
                Some(get_our_value(&our_choice, &opponent_value))
        })
        .sum();

    println!("The total is {}", total);
}
