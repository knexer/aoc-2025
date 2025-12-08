pub fn part2(input: &str) -> i64 {
    let manifold = parse(input);
    manifold
        .iter()
        .fold(
            manifold[0]
                .iter()
                .map(|start| if *start { 1 } else { 0 })
                .collect(),
            propagate_lazars,
        )
        .iter()
        .sum()
}

fn propagate_lazars(lazars: Vec<i64>, splitters: &Vec<bool>) -> Vec<i64> {
    (0..lazars.len())
        .map(|index| {
            let from_left = if index > 0 && splitters[index - 1] {
                lazars[index - 1]
            } else {
                0
            };
            let from_right = if index < lazars.len() - 1 && splitters[index + 1] {
                lazars[index + 1]
            } else {
                0
            };
            let from_center = if !splitters[index] { lazars[index] } else { 0 };
            from_left + from_center + from_right
        })
        .collect()
}

fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| matches!(char, 'S' | '^'))
                .collect::<Vec<bool>>()
        })
        .collect()
}
