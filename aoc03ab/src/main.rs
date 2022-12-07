#![feature(iter_array_chunks)]
use std::collections::HashSet;

use utils::input::get_input;

fn main() {

    let lines = get_input();

    let halves = lines.iter().map(|line| {
        let len = line.len();
        (HashSet::<_>::from_iter(line[(len/2)..].as_bytes()),
         HashSet::<_>::from_iter(line[..(len/2)].as_bytes()))
    });

    let repeated = halves.map( |(first, second)| {
            first.into_iter().filter(move |letter| second.contains(letter))
        }
    ).flatten().collect::<Vec<_>>();

    println!("[AOC03] Part 1 result: \t {}", repeated.iter().map(|c| value(*c)).sum::<i32>());
    println!("[AOC03] Part 2 result: {}", get_aggregated_badge_values(lines.into_iter()));

}

fn value(c: &u8) -> i32 {
    match *c as char {
        'a'..='z' => (c - 96).into(),
        'A'..='Z' => (c - 65 + 27).into(),
        _ => panic!("invalid item in rucksack: {}", *c as char)
    }
}

fn get_aggregated_badge_values(rucksacks: impl Iterator<Item = String>) -> i32 {
    
    let groups: Vec<[String; 3]> = rucksacks.array_chunks::<3>().collect();
    let badges = groups.iter().map(|[first, second, third]| {
        let index = first.find(|c| second.contains(c) && third.contains(c)).expect("no badge in this group");
        first.bytes().nth(index).unwrap()
    });
    badges.map(|badge| value(&badge)).sum()
}