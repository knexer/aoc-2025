pub fn part2(input: &str) -> i64 {
    let ranges = input.split(',').map(parse_range);

    ranges
        .map(|range| {
            (range.0..range.1 + 1)
                .filter(|i| is_invalid(*i))
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

fn is_invalid(id: i64) -> bool {
    let num_digits = id.ilog10() + 1;
    let string_id = id.to_string();
    for i in 1..num_digits {
        if num_digits % i != 0 {
            continue;
        }

        let mut chunks = string_id.as_bytes().chunks_exact(i as usize);
        let first_chunk = chunks.next().unwrap();
        if chunks.all(|chunk| chunk == first_chunk) {
            return true;
        }
    }

    false
}
