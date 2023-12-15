use std::fs;

fn main() {
    let data = read_input();
    part1(&data);
}

fn read_input() -> String {
    fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-15\\input.txt")
        .expect("Error reading input file")
}

fn part1(s: &String) {
    let mut total_value = 0;
    for group in s.split(',') {
        total_value += hash(group);
    }
    println!("Part 1: {}", total_value);
}

fn hash(s: &str) -> u64 {
    let mut value = 0;
    for c in s.as_bytes() {
        value += u64::from(*c);
        value *= 17;
        value %= 256;
    }
    value
}
