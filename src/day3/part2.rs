pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(line_to_bank)
        .map(|bank| bank_to_joltage(bank, 12))
        .sum()
}

fn line_to_bank(line: &str) -> Vec<i64> {
    line.chars()
        .map(|char| char.to_digit(10).unwrap() as i64)
        .collect()
}

fn bank_to_joltage(bank: Vec<i64>, batteries: usize) -> i64 {
    if batteries == 0 {
        return 0;
    }

    let (prefix, _) = bank.split_at(bank.len() - batteries + 1);
    let prefix_max = *prefix.iter().max().unwrap();
    let first_max_index = prefix.iter().position(|&val| val == prefix_max).unwrap();

    let (_, suffix) = bank.split_at(first_max_index + 1);
    let remaining_joltage = bank_to_joltage(suffix.to_vec(), batteries - 1);

    prefix_max * 10i64.pow(batteries as u32 - 1) + remaining_joltage
}
