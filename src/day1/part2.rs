pub fn part2(input: &str) -> i64 {
    let net_sums = input.lines().map(line_to_increments).scan(50, |sum, next| {
        *sum += next;
        Some(*sum)
    });

    let zero_crossings = std::iter::once(50)
        .chain(net_sums.clone())
        .zip(net_sums)
        .map(|(old, new)| match new > old {
            // when going right, ending on a 0 means changing the quotient (e.g. 99->100 is 0->1)
            true => ((new).div_euclid(100) - (old).div_euclid(100)).abs(),
            // when going left, ending on a 0 doesn't change the quotient, so correct for that (e.g. 1->0 is 0->0)
            false => ((new - 1).div_euclid(100) - (old - 1).div_euclid(100)).abs(),
        });

    zero_crossings.sum()
}

fn line_to_increments(line: &str) -> i64 {
    let (prefix, num) = line.split_at(1);
    let num = num.parse::<i64>().expect("Should be an integer: {num}");
    match prefix {
        "L" => -num,
        "R" => num,
        _ => panic!("Invalid prefix: {prefix}"),
    }
}

// 50, then [-18, -48, 0, -5, 55, 0, -1, -100, -86, -168]
// [1, 0, 1, 0, 1, 1, 0, 1, 0, 1] = 6
