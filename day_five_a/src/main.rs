use std::str;

//IDEA:
//  Split the file by \n\n :check
//  Convert each stack into a vec
//  Split each instuction by " "
//  filter the instuction vec inot only values that can be coerced into u16
// (Maybe) put them into a sturct called instructiosn
// Create a functions that can manipulate the stacks based on the instuctions
// (Maybe) create a struct for the CargoBay

#[derive(Debug)]
struct Instruction {
    quantity: u32,
    start: u32,
    end: u32,
}

impl Instruction {
    fn new(quantity: u32, start: u32, end: u32) -> Self {
        Instruction {
            quantity: quantity,
            start: start,
            end: end,
        }
    }
}

struct CargoBay {
    number_of_stacks: u32,
    stacks: Vec<Vec<String>>, 
}

fn main() {
    let (cargo, instuctions) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let mut cargo_rows = cargo
        .lines()
        .map(|row| {
            row.as_bytes()
                .chunks(4)
                .map(str::from_utf8)
                .map(|store| store.unwrap().trim())
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>();
    cargo_rows.pop();

    let instuctions = instuctions
        .lines()
        .map(|instruction| {
            let instruction = instruction
                .chars()
                .filter(|character| character.is_numeric())
                .map(|character| character.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            Instruction::new(instruction[0], instruction[1], instruction[2])
        })
        .collect::<Vec<Instruction>>();

    for instruction in instuctions {
        dbg!(instruction);
    }

    for row in cargo_rows {
        dbg!(row);
    }
}
