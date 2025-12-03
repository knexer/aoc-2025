use std::{fs, str};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file!");
    let zeroes = input
        .lines()
        .map(line_to_increments)
        .scan(50, |sum, next| {
            *sum = (*sum + next) % 100;
            Some(*sum)
        })
        .filter(|item| *item == 0);
    println!("{:?}", zeroes.count());
}

fn line_to_increments(line: &str) -> i32 {
    let (prefix, num) = line.split_at(1);
    let num = num.parse::<i32>().expect("Should be an integer: {num}");
    match prefix {
        "L" => -num,
        "R" => num,
        _ => panic!("Invalid prefix: {prefix}"),
    }
}
