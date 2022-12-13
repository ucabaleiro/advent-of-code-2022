


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
    pub fn transpose<S: Clone>(matrix: &Vec<Vec<S>>) -> Vec<Vec<S>> {

        let mut transposed: Vec<Vec<S>> = Vec::new();
    
        matrix.first().unwrap().iter().for_each(|_| transposed.push(Vec::new()));
    
        for row in matrix {
            for (j, element) in row.into_iter().enumerate() {
                transposed[j].push(element.clone());
            }
        }
    
        transposed
    }
    
    pub fn rotate_clockwise<S: Clone>(matrix: &Vec<Vec<S>>) -> Vec<Vec<S>> {

        let (rows, cols) = (matrix.len(), matrix.first().expect("provided empty matrix").len());

        let mut rotated: Vec<Vec<S>> = Vec::new(); 

        matrix.last().expect("provided empty matrix")
            .iter().for_each(|_| rotated.push(Vec::new()));

        for i in 0..cols {
            for j in 0..rows {
                rotated[i].push(matrix[rows - 1 - j][i].clone());
            }
        }

        rotated
    }
}

pub mod extras {
    use std::{collections::HashSet, hash::Hash};

    pub fn contains_duplicates<T: Eq + Hash>(vector: impl Iterator<Item = T> + Clone) -> bool {
        let initial = vector.clone();
        let len_without_duplicates = HashSet::<_>::from_iter(vector).len();

        initial.count() != len_without_duplicates
    }
}


