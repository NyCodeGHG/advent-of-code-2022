use std::fs;

fn main() {
    let input = fs::read_to_string("day-01/input.txt").unwrap();
    let mut elfes: Vec<i32> = input
        .split("\n\n")
        .map(|text| {
            text.lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();

    elfes.sort();

    let most = elfes.last().unwrap();
    println!("{most}");

    let last_three: i32 = elfes.iter().rev().take(3).sum();
    println!("{last_three}");
}
