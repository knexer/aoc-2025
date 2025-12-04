pub fn part1(input: &str) -> i64 {
    input.lines().map(line_to_bank).map(bank_to_joltage).sum()
}

fn line_to_bank(line: &str) -> Vec<i64> {
    line.chars()
        .map(|char| char.to_digit(10).unwrap() as i64)
        .collect()
}

fn bank_to_joltage(bank: Vec<i64>) -> i64 {
    let max = *bank.iter().max().unwrap();
    let first_max = bank.iter().position(|&val| val == max).unwrap();
    let (prefix, suffix) = bank.split_at(first_max);
    let (_, suffix) = suffix.split_at(1);
    if !suffix.is_empty() {
        let suffix_max = *suffix.iter().max().unwrap();
        max * 10 + suffix_max
    } else {
        let prefix_max = *prefix.iter().max().unwrap();
        prefix_max * 10 + max
    }
}
