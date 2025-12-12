use std::convert::identity;

pub fn part1(input: &str) -> i64 {
    let machines = parse(input);
    machines.iter().map(solve).sum()
}

#[derive(Debug, Clone)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    // joltages: Vec<usize>,
}

fn solve(machine: &Machine) -> i64 {
    for num_presses in 0..machine.buttons.len() + 1 {
        if solve_rec(machine.lights.clone(), &machine.buttons, num_presses, 0) {
            return num_presses as i64;
        }
    }
    panic!("Unsolvable!!????!?!?!");
}

fn solve_rec(
    lights: Vec<bool>,
    buttons: &Vec<Vec<usize>>,
    num_presses: usize,
    starting_index: usize,
) -> bool {
    if num_presses == 0 {
        return lights.iter().copied().all(identity);
    }
    let mut lights_if_pressed = lights.clone();
    for light in &buttons[starting_index] {
        lights_if_pressed[*light] = !lights_if_pressed[*light];
    }
    solve_rec(
        lights_if_pressed,
        buttons,
        num_presses - 1,
        starting_index + 1,
    ) || num_presses <= buttons.len() - (starting_index + 1)
        && solve_rec(lights, buttons, num_presses, starting_index + 1)
}

fn parse_machine(line: &str) -> Machine {
    let lights: Vec<bool> = line[1..line.find(']').unwrap()]
        .chars()
        .map(|char| char == '.')
        .collect();
    let buttons: Vec<Vec<usize>> = line
        .split('(')
        .skip(1)
        .map(|schematic| {
            schematic
                .split_once(')')
                .unwrap()
                .0
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    Machine { lights, buttons }
}

fn parse(input: &str) -> Vec<Machine> {
    input.lines().map(parse_machine).collect()
}
