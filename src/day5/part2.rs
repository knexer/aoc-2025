pub fn part2(input: &str) -> i64 {
    let (mut ranges, _) = parse_input(input);

    // sort ranges by beginning id
    // merge overlapping ranges
    // sum sizes of each non-overlapping range

    ranges.sort();

    let mut merged_ranges: Vec<(i64, i64)> = Vec::new();
    let mut current_range = ranges[0];
    for (begin, end) in ranges {
        if begin > current_range.1 {
            // start new range
            merged_ranges.push(current_range);
            current_range = (begin, end);
        } else {
            // update existing range
            current_range = (current_range.0, current_range.1.max(end));
        }
    }
    merged_ranges.push(current_range);

    merged_ranges
        .iter()
        .map(|(begin, end)| end - begin + 1)
        .sum()
}

fn parse_input(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let (ranges_input, ids_input) = input.split_once("\n\n").unwrap();
    let ranges = ranges_input.lines().map(parse_range);
    let ids = ids_input.lines().map(|id| str::parse::<i64>(id).unwrap());

    (ranges.collect(), ids.collect())
}

fn parse_range(range: &str) -> (i64, i64) {
    let (begin, end) = range.split_once('-').unwrap();
    (str::parse(begin).unwrap(), str::parse(end).unwrap())
}
