pub fn part1(input: &str) -> i64 {
    let ranges = input.split(',').map(parse_range);

    // only even-length numbers can be invalid
    // simplest solution: iterate over all numbers in each range and check for validity
    // slightly less dumb: iterate over the first n/2 digits, this yields a smaller list
    // even less dumb: only check the first and last of those; the intermediate ones must be in the range

    // ... wow ok, simplest solution is plenty fast I guess??
    ranges
        .map(|range| {
            (range.0..range.1 + 1)
                .filter(|i| !is_valid(*i))
                .sum::<i64>()
        })
        .sum()
}

fn parse_range(range: &str) -> (i64, i64) {
    let (first, last) = range
        .split_once('-')
        .unwrap_or_else(|| panic!("Invalid range: {range}"));
    (
        first
            .parse::<i64>()
            .unwrap_or_else(|_| panic!("NaN: {first}")),
        last.parse::<i64>()
            .unwrap_or_else(|_| panic!("NaN: {first}")),
    )
}

fn is_valid(id: i64) -> bool {
    let num_digits = id.ilog10() + 1;
    if num_digits % 2 == 1 {
        return true;
    }
    let mid = 10i64.pow(num_digits / 2);
    let (big_part, smol_part) = (id / mid, id % mid);

    big_part != smol_part
}
