use std::fs::read_to_string;

fn decipher_value(value: &str) -> i32 {
    match value {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        &_ => panic!(),
    }
}

fn get_outcome(choices: &mut Vec<i32>) -> i32 {
    let mut score = 0;
    if choices.len() >= 2 {
        score += choices[1];
        if choices[0] == 1 && choices[1] == 3 {
            score += 0;
        } else if choices[0] == 3 && choices[1] == 1 {
            score += 6;
        } else if choices[0] < choices[1] {
            score += 6;
        } else if choices[0] == choices[1] {
            score += 3;
        }
    }
    println!("{}", score);
    score
}

fn main() {
    let total: i32 = read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(|event| {
            let mut event_choices = event
                .split_whitespace()
                .map(|choice| decipher_value(choice))
                .collect();
            get_outcome(&mut event_choices)
        })
        .sum::<i32>();
    println!("{}", total);
}
