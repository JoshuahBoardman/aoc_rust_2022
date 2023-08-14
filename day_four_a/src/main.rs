#[derive(Debug)]
struct Assignment(i32, i32);

#[derive(Debug)]
struct AssignmentPair {
    elf_a: Assignment,
    elf_b: Assignment,
}

impl AssignmentPair {
    fn check_for_overlap(self) -> bool {
        (self.elf_a.0 <= self.elf_b.0 && self.elf_a.1 >= self.elf_b.1)
            || (self.elf_a.0 >= self.elf_b.0 && self.elf_a.1 <= self.elf_b.1)
    }
}

fn main() {
    let number_of_overlapping_assignments: i32 = include_str!("../input.txt")
        .lines()
        .map(|pair| {
            let (a, b) = pair.split_once(",").unwrap();
            let (a_low, a_high) = a.split_once("-").unwrap();
            let (b_low, b_high) = b.split_once("-").unwrap();

            let assingment_a = Assignment(
                a_low.parse::<i32>().unwrap(),
                a_high.parse::<i32>().unwrap(),
            );

            let assingment_b = Assignment(
                b_low.parse::<i32>().unwrap(),
                b_high.parse::<i32>().unwrap(),
            );

            let assingment_pair = AssignmentPair {
                elf_a: assingment_a,
                elf_b: assingment_b,
            };

            assingment_pair.check_for_overlap() as i32
        })
        .sum();

    println!("{number_of_overlapping_assignments:?}");
}
