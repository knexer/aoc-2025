use std::iter::once;

pub fn part2(input: &str) -> i64 {
    let problems = parse(input);

    problems
        .iter()
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

fn transpose<T: Copy>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = v.len();
    let cols = v[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| v[row][col]).collect())
        .collect()
}

fn parse_problem(problem: &[String]) -> (Vec<i64>, Op) {
    let operator = if problem[0].contains('+') {
        Op::Add
    } else {
        Op::Multiply
    };
    let operand_1 = problem[0].trim_matches('+').trim_matches('*');

    let operands: Vec<i64> = once(&operand_1.to_string())
        .chain(problem.iter().skip(1))
        .map(|operand| str::parse::<i64>(operand.trim()).unwrap())
        .collect();

    (operands, operator)
}

fn parse(input: &str) -> Vec<(Vec<i64>, Op)> {
    let chars: Vec<Vec<char>> = input.lines().map(|str| str.chars().collect()).collect();
    let chars = transpose(chars);
    let lines: Vec<String> = chars.iter().map(|chars| chars.iter().collect()).collect();
    let problems: Vec<&[String]> = lines.split(|line| line.trim().is_empty()).collect();

    problems
        .iter()
        .map(|problem: &&[std::string::String]| parse_problem(problem))
        .collect()
}
