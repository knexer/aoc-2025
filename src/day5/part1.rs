pub fn part1(input: &str) -> i64 {
    let (ranges, ids) = parse_input(input);

    // naive version: check each id against every range
    // can optimize by sorting first, then zipping, but I doubt it will be necessary

    ids.iter()
        .filter(|id| ranges.iter().any(|range| id_in_range(*range, **id)))
        .count() as i64
}

fn id_in_range((begin, end): (i64, i64), id: i64) -> bool {
    id >= begin && id <= end
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
