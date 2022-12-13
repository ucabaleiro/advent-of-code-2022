use std::{fs, collections::VecDeque};
use utils::extras;

struct LimitedSizeCharVecDeque {
    size_limit: usize,
    content: VecDeque<char>
}

impl LimitedSizeCharVecDeque {
    pub fn new(limit: usize) -> Self {
        Self {size_limit: limit, content: VecDeque::new()}
    }

    pub fn push(&mut self, c: char) {
        if self.complete() {self.content.pop_front();}
        self.content.push_back(c);
    }

    pub fn contains_duplicates(&self) -> bool { 
        extras::contains_duplicates(self.content.iter())
    }

    pub fn complete(&self) -> bool {
        self.content.len() == self.size_limit
    }
}



fn main() {
    let input = fs::read_to_string("./resources/input.txt").expect("error opening input file");

    println!("[AOC06] Part 1 - First start of packet:    {:?}", index_of_first_sequence_without_n_duplicates(input.clone(), 4));
    println!("[AOC06] Part 2 - First start of message:   {:?}", index_of_first_sequence_without_n_duplicates(input, 14));
}


fn index_of_first_sequence_without_n_duplicates(input: String, n: usize) -> Option<usize> {
    let mut current_4_chars = LimitedSizeCharVecDeque::new(n);
    
    input.chars().into_iter().enumerate()
        .fold(Option::<usize>::None, |mut accum, (index, c)| {
            current_4_chars.push(c);
            if current_4_chars.complete() && !current_4_chars.contains_duplicates() {
                accum = accum.or_else(|| Some(index + 1))
            }
                accum
            }
        )
}