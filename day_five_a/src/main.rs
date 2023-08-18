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
    quantity: i32,
    start: i32,
    end: i32,
}

impl Instruction {
    fn new(quantity: i32, start: i32, end: i32) -> Self {
        Instruction {
            quantity: quantity,
            start: start,
            end: end,
        }
    }
}

#[derive(Debug)]
struct CargoBay {
    number_of_stacks: i32,
    stacks: Vec<Vec<String>>,
}

impl CargoBay {
    fn new(number_of_stacks: i32) -> Self {
        let mut stacks = Vec::new();
        for _ in 0..number_of_stacks {
            stacks.push(Vec::new());
        }
        CargoBay {
            number_of_stacks: number_of_stacks,
            stacks: stacks,
        }
    }

    fn stack_cargo(&mut self, cargo: Vec<&str>) {
        for (i, stack) in self.stacks.iter_mut().enumerate() {
            let current_cargo = cargo.get(i).unwrap();
            if !current_cargo.to_string().is_empty() {
                stack.push(current_cargo.to_string());
            }
        }
    }

    fn move_cargo(&mut self, instruction: Instruction) {
        for _ in 0..instruction.quantity {
            let current_cargo = self.stacks[(instruction.start - 1) as usize].pop().unwrap();
            self.stacks[(instruction.end - 1) as usize].push(current_cargo);
        }
    }

    fn get_top_cargo(&self) -> String {
        let mut top_cargo = Vec::new();
        self.stacks
            .iter()
            .for_each(|stack| top_cargo.push(stack[stack.len() - 1].clone()));
        top_cargo.join(", ")
    }
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

    let mut cargo_bay = CargoBay::new(i32::try_from(cargo_rows[0].len()).unwrap());

    cargo_rows
        .iter()
        .rev()
        .for_each(|row| cargo_bay.stack_cargo(row.to_vec()));

    let instuctions = instuctions
        .lines()
        .map(|instruction| {
            let instruction = instruction
                .split_whitespace()
                .filter_map(|value| {
                    let value = value.parse::<i32>();
                    match value {
                        Ok(value) => Some(value),
                        Err(_) => None,
                    }
                })
                .collect::<Vec<i32>>();

            Instruction::new(instruction[0], instruction[1], instruction[2])
        })
        .collect::<Vec<Instruction>>();

    instuctions
        .into_iter()
        .for_each(|instuction| cargo_bay.move_cargo(instuction));

    let top_cargo = cargo_bay.get_top_cargo();

    println!("{top_cargo:?}");
}
