pub fn part1(input: &str) -> i64 {
    let red_tiles: Vec<(i64, i64)> = parse_input(input);
    let mut best_area = 0;
    for i in 0..red_tiles.len() - 1 {
        for j in i + 1..red_tiles.len() {
            let area = area(red_tiles[i], red_tiles[j]);
            if area > best_area {
                best_area = area;
            }
        }
    }

    best_area
}

fn area((x_1, y_1): (i64, i64), (x_2, y_2): (i64, i64)) -> i64 {
    ((x_2 - x_1).abs() + 1) * ((y_2 - y_1).abs() + 1)
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x_str, y_str)| (x_str.parse().unwrap(), y_str.parse().unwrap()))
        .collect()
}
