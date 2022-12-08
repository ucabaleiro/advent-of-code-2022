


pub mod input {
    use std::fs::File;
    use std::io::{self, BufRead};

    pub fn get_input() -> Vec<String> {
        let input_file = File::open("./resources/input.txt");
        let lines = input_file
        .map(|input| io::BufReader::new(input).lines());
    
        match lines {
            Ok(lines_buffer) => lines_buffer.into_iter().map(|line| line.expect("error parsing input line")).collect(),
            Err(error) => panic!("input error: {}", error.to_string())
        }
    }
} 


pub mod matrix {
    pub fn transpose<S>(matrix: Vec<Vec<S>>) -> Vec<Vec<S>> {

        let mut transposed: Vec<Vec<S>> = Vec::new();
    
        matrix.first().unwrap().iter().for_each(|_| transposed.push(Vec::new()));
    
        for row in matrix {
            for (j, element) in row.into_iter().enumerate() {
                transposed[j].push(element);
            }
        }
    
        transposed
    }
    
    pub fn rotate_clockwise<S>(matrix: Vec<Vec<S>>) -> Vec<Vec<S>> {
    
        let mut transposed = transpose(matrix);
    
        let mut inverse_transposed = Vec::new();
    
        for row in transposed {
             inverse_transposed.push(row.into_iter().rev().collect::<Vec<S>>()); 
        }
    
        inverse_transposed
    }
}




