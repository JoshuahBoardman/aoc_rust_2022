fn main() {

   let _ = include_str!("../input.txt")
       .lines()
       // Change for_each to map
       .for_each(|line| { 
           let (first_pocket, second_pocket) = line.split_at(line.len()/2);
           println!("First -  {}", first_pocket);
           println!("Second - {}", second_pocket);
       });
}
