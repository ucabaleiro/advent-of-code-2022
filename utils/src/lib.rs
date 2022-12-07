


pub mod input {
    use std::fs::File;
    use std::io::{self, BufRead};

    pub fn get_input() -> Vec<String> {
        let input_file = File::open("../resources/input.txt");
        let lines = input_file
        .map(|input| io::BufReader::new(input).lines());
    
        match lines {
            Ok(lines_buffer) => lines_buffer.into_iter().map(|line| line.expect("error parsing input line")).collect(),
            Err(error) => panic!("input error: {}", error.to_string())
        }
    }



} 





