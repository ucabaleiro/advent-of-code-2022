use utils::input::get_input;

fn main() {
    let lines = get_input();

    let subtotals = vec![0];
    let mut subtotals = lines.into_iter().fold(subtotals, |results, line| update_results(results, line));
    subtotals.sort();

    println!("max: {}", subtotals.iter().max().expect("error getting maximum"));
    println!("sum of 3 highest: {}", subtotals.iter().rev().take(3).sum::<i32>());
}


fn update_results(mut results: Vec<i32>, line: String) -> Vec<i32> {

    if line == "" {
        results.push(0)
    } else {
        let line_as_int: i32= line.parse().expect("error parsing line as integer");
        results.pop().map(|val| results.push(val + line_as_int));
    }
    return results;
}