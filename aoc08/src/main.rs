use std::f32::RADIX;

use utils::input::get_input;

fn main() {
    let trees: Vec<Vec<u32>> = get_input().into_iter().map(|s| s.chars().map(|c| c.to_digit(RADIX).unwrap()).collect()).collect();

}
