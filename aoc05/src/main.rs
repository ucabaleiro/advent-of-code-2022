use utils::input::get_input;
use utils::matrix::rotate_clockwise;

struct Instruction(usize, usize, usize);

impl Instruction {
    fn from_string(line: &String) -> Self {
        let parts = line.chars().filter(|c| !c.is_alphabetic()).collect::<String>();
        let mut parts = parts.split(char::is_whitespace)
        .filter(|str| *str != "")
        .map(|part| part.parse::<usize>().unwrap());
        
        let amount = parts.next().unwrap();
        let from = parts.next().unwrap() - 1;
        let to = parts.next().unwrap() - 1;


        Self(amount, from, to)
    }

    // repeated logic
    fn apply_on(&self, stacks: &mut Vec<Vec<char>>) {
        let Instruction(amount, from, to) = *self;
        
        let final_len = stacks[from].len() - amount;
        let mut to_append: Vec<char> = stacks[from].drain(final_len..).rev().collect();
        stacks[to].append(&mut to_append);
    }

    fn appply_without_changin_order_on(&self, stacks: &mut Vec<Vec<char>>) {
        let Instruction(amount, from, to) = *self;
        
        let final_len = stacks[from].len() - amount;
        let mut to_append: Vec<char> = stacks[from].drain(final_len..).collect();
        stacks[to].append(&mut to_append);
    }
}

fn main() {
    let lines = get_input();
    let mut input = lines.split(|line| line == "");

    let stack_lines = input.next().expect("error splitting input into stacks and instructions");
    let instructions = input.next()
        .expect("error splitting input into stacks and instructions")
        .iter().map(Instruction::from_string);

    let stacks: Vec<Vec<char>>= stack_lines.into_iter()
        .map(|stack_line| stack_line.chars().collect()).collect();

    let mut stacks: Vec<Vec<char>> = rotate_clockwise(stacks).into_iter()
        .filter(|line| *line.first().unwrap() != ' ')
        .map(|mut stack| stack.drain(1..).filter(|c| !c.is_whitespace()).collect())
        .collect();
    println!("{:?}", stacks);

    let mut stacks_part_2 = stacks.clone();



    instructions.clone().for_each(|instr| instr.apply_on(&mut stacks));


    let result_part_1 = stacks.iter()
        .map(|stack| stack.last())
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect::<String>();

    

    instructions.for_each(|instr| instr.appply_without_changin_order_on(&mut stacks_part_2));
    
    let result_part_2 = stacks_part_2.iter()
    .map(|stack| stack.last())
    .filter(Option::is_some)
    .map(Option::unwrap)
    .collect::<String>();


    println!("[AOC05] Part 1 - Result: \t {}", result_part_1);
    println!("[AOC05] Part 2 - Result: \t {}", result_part_2);

}