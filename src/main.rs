use std::fs;

mod day1 {
    pub mod part1;
}

fn main() {
    let (day, part, test) = parse_args();

    let folder = match test {
        true => "test_inputs",
        false => "inputs",
    };
    let path = format!("{}/{}.txt", folder, day);
    let input = fs::read_to_string(path).expect("Should have been able to read the file!");
    let solution = match (day, part) {
        (1, 1) => day1::part1::part1,
        _ => panic!("No implementation registered for day {day} part {part}!"),
    };
    println!("{:?}", solution(&input));
}

fn parse_args() -> (i32, i32, bool) {
    let args: Vec<String> = std::env::args().collect();
    let (req_args, optional_args) = args.split_at(3);
    let day: i32 = req_args[1].parse().expect("Invalid day: {day}");
    let part: i32 = req_args[2].parse().expect("Invalid part: {part}");
    let test =
        optional_args.contains(&"-t".to_string()) || optional_args.contains(&"--test".to_string());

    (day, part, test)
}
