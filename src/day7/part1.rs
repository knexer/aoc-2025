pub fn part1(input: &str) -> i64 {
    let manifold = parse(input);
    manifold
        .iter()
        .fold((0, manifold[0].clone()), propagate_lazars)
        .0
}

fn propagate_lazars((splits, lazars): (i64, Vec<bool>), splitters: &Vec<bool>) -> (i64, Vec<bool>) {
    (
        splits
            + lazars
                .iter()
                .zip(splitters)
                .filter(|(&lazar, &splitter)| lazar && splitter)
                .count() as i64,
        (0..lazars.len())
            .map(|index| {
                lazars[index] && !splitters[index]
                    || index > 0 && lazars[index - 1] && splitters[index - 1]
                    || index < lazars.len() - 1 && lazars[index + 1] && splitters[index + 1]
            })
            .collect(),
    )
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
