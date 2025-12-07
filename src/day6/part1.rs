pub fn part1(input: &str) -> i64 {
    let (operands, operators) = parse(input);
    let operands = transpose(operands);

    operands
        .iter()
        .zip(operators)
        .map(|(operands, operator)| match operator {
            Op::Add => operands.iter().sum::<i64>(),
            Op::Multiply => operands.iter().product(),
        })
        .sum()
}

enum Op {
    Add,
    Multiply,
}

fn transpose(v: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let rows = v.len();
    let cols = v[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| v[row][col]).collect())
        .collect()
}

fn parse(input: &str) -> (Vec<Vec<i64>>, Vec<Op>) {
    let elements: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    let (operators, operands) = elements.split_last().unwrap();
    let operands = operands
        .iter()
        .map(|row| {
            row.iter()
                .map(|str| str::parse::<i64>(str).unwrap())
                .collect()
        })
        .collect();
    let operators = operators
        .iter()
        .map(|str| match *str {
            "+" => Op::Add,
            "*" => Op::Multiply,
            _ => panic!(),
        })
        .collect();
    (operands, operators)
}
