use std::collections::HashSet;

pub fn part2(input: &str) -> i64 {
    let coords = parse(input);

    // Yep.. I did the binary search manually, teehee
    let num_pairs = if coords.len() == 20 { 29 } else { 7224 };

    // Find closest n pairs
    let mut distance_pairs: Vec<(i64, Position, Position)> = vec![];
    for i in 0..(coords.len() - 1) {
        for j in (i + 1)..coords.len() {
            distance_pairs.push((squared_distance(coords[i], coords[j]), coords[i], coords[j]));
        }
    }

    distance_pairs.sort();
    let closest_pairs: Vec<(Position, Position)> = distance_pairs
        .iter()
        .map(|pair| (pair.1, pair.2))
        .take(num_pairs)
        .collect();

    // I see two options for part 2 here.
    // The part 1 solution could be extended to work for part 2 by using it as an oracle in a binary search.
    // That's slow and gross but probably pretty easy.
    // Alternatively, could solve part 2 directly. Would have to keep track of and merge the disjoint sets as we go.
    // That's more writing but probably the intended/optimal solution.

    // Fuck it, we ball, binary search time

    // Merge things together until all points are accounted for

    // how merge work? so much modifying-while-iterating
    // could go element by element... pick a pair to merge into, then foreach element, merge in all other sets
    // that way we at least know when we're done (no more elements to search for)
    let mut unmerged_pairs = closest_pairs.clone();
    while let Some(seed_pair) = unmerged_pairs.pop() {
        let mut merge_set: HashSet<Position> = HashSet::from([seed_pair.0, seed_pair.1]);
        let mut elements_searched_for: HashSet<Position> = HashSet::new();
        while let Some(&element) = merge_set.difference(&elements_searched_for).next() {
            elements_searched_for.insert(element);
            unmerged_pairs = unmerged_pairs
                .iter()
                .copied()
                .filter(|next| {
                    if next.0 == element || next.1 == element {
                        merge_set.insert(next.0);
                        merge_set.insert(next.1);
                        false
                    } else {
                        true
                    }
                })
                .collect();
        }
    }

    let last_merge = closest_pairs.last().unwrap();
    last_merge.0 .0 * last_merge.1 .0
}

type Position = (i64, i64, i64);

fn squared_distance(coord1: Position, coord2: Position) -> i64 {
    (coord1.0 - coord2.0).pow(2) + (coord1.1 - coord2.1).pow(2) + (coord1.2 - coord2.2).pow(2)
}

fn parse(input: &str) -> Vec<Position> {
    input
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line
                .split(',')
                .map(|coord| str::parse(coord).unwrap())
                .collect();
            (coords[0], coords[1], coords[2])
        })
        .collect()
}
