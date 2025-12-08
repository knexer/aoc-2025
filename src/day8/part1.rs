use std::collections::HashSet;

pub fn part1(input: &str) -> i64 {
    let coords = parse(input);
    // I hate it when you have to special case the test input like this...
    let num_pairs = if coords.len() == 20 { 10 } else { 1000 };

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

    // Merge things together until they won't merge no more

    // how merge work? so much modifying-while-iterating
    // could go element by element... pick a pair to merge into, then foreach element, merge in all other sets
    // that way we at least know when we're done (no more elements to search for)
    let mut unmerged_pairs = closest_pairs;
    let mut set_sizes: Vec<i64> = vec![];
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
        println!("{:?}", merge_set);
        set_sizes.push(merge_set.len() as i64);
    }

    set_sizes.sort();
    set_sizes.reverse();
    println!("{:?}", set_sizes);
    set_sizes.iter().take(3).product()
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
