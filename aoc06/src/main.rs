use std::{fs, collections::VecDeque};

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

    pub fn contains_duplicates(&self) -> bool { //todo: optimize, slow af
        self.content.iter().any(|c| self.occurances(c) > 1)
    }

    fn occurances(&self, c: &char) -> usize {
        self.content.iter().filter(|c2| *c2 == c).count()
    }

    pub fn complete(&self) -> bool {
        self.content.len() == self.size_limit
    }
}



fn main() {
    let input = fs::read_to_string("./resources/input.txt").expect("error opening input file");

    println!("First start of packet:    {:?}", find_first_sequence_of_N_without_duplicates(input.clone(), 4));
    println!("First start of message:   {:?}", find_first_sequence_of_N_without_duplicates(input, 14));
}


fn find_first_sequence_of_N_without_duplicates(input: String, N: usize) -> Option<usize> {
    let mut current_4_chars = LimitedSizeCharVecDeque::new(N);
    
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