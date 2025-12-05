pub fn part1(input: &str) -> i64 {
    let grid = parse_grid(input);

    let mut available = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if is_available(&grid, i, j) {
                available += 1;
            }
        }
    }

    available
}

fn is_available(grid: &[Vec<i64>], i: usize, j: usize) -> bool {
    #[rustfmt::skip]
    let offsets = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let adjacent: i64 = offsets
        .iter()
        .filter_map(|(dx, dy)| {
            let x = i as isize + dx;
            let y = j as isize + dy;
            if x >= 0 && y >= 0 {
                grid.get(x as usize).and_then(|row| row.get(y as usize))
            } else {
                None
            }
        })
        .sum();

    adjacent < 4 && grid[i][j] == 1
}

fn parse_grid(input: &str) -> Vec<Vec<i64>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<i64> {
    line.chars()
        .map(|char| match char {
            '.' => 0,
            '@' => 1,
            _ => panic!("Invalid char {char}"),
        })
        .collect()
}
