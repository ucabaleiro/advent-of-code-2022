use utils::input::get_input;

fn main() {
    let lines = get_input();

    let ranges = lines.iter().map(line_to_ranges);

    let contained = ranges.clone().filter(|(first, second)| full_overlap(first, second));

    let overlapped = ranges.filter(|(first, second)| overlapped(first, second));

    println!("[AOC04] part 1 result: \t{}", contained.count());
    println!("[AOC04] part 2 result: \t{}", overlapped.count());
}


fn line_to_ranges(line: &String) -> ((i32, i32), (i32, i32)) {
    let (first, second) = line.split_once(',').expect("malformed input");

    let (lower, upper) = first.split_once('-').expect("malformed input");
    let first_bounds: (i32, i32) = (lower.parse().unwrap(), upper.parse().unwrap());

    let (lower, upper) = second.split_once('-').expect("malformed input");
    let second_bounds: (i32, i32) = (lower.parse().unwrap(), upper.parse().unwrap());
    
    (first_bounds, second_bounds)
}

fn full_overlap((lower1, upper1): &(i32, i32), (lower2, upper2): &(i32, i32)) -> bool {
    (lower1 <= lower2 && upper1 >= upper2) ||
    (lower2 <= lower1 && upper2 >= upper1)
}

fn overlapped((lower1, upper1): &(i32, i32), (lower2, upper2): &(i32, i32)) -> bool {
    full_overlap(&(*lower1, *upper1), &(*lower2, *upper2)) ||
    (lower1 <= lower2 && lower2 <= upper1) ||
    (lower2 <= lower1 && lower1 <= upper2)
}