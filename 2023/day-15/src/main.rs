use std::fs;

#[derive(Debug, Clone)]
struct Lens {
    id: String,
    val: usize,
}

fn main() {
    let data = read_input();
    part1(&data);
    part2(&data);
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

fn part2(s: &String) {
    let mut boxes = std::iter::repeat(vec![])
        .take(256)
        .collect::<Vec<Vec<Lens>>>();
    for group in s.split(',') {
        if group.contains("=") {
            let mut split = group.split("=");
            let label = split.next().expect("Error getting lens label");
            let value = split
                .next()
                .expect("Error getting lens value")
                .parse::<usize>()
                .expect("Error parsing number for lens");
            let box_index = usize::try_from(hash(label)).expect("Error getting box index");
            let b = boxes.get_mut(box_index).expect("Error getting box");
            if let Some(lens) = b.iter_mut().find(|l| l.id == label) {
                lens.val = value;
            } else {
                b.push(Lens {
                    id: label.to_owned(),
                    val: value,
                });
            }
        } else if group.contains("-") {
            let mut split = group.split("-");
            let label = split.next().expect("Error getting lens label");
            let box_index = usize::try_from(hash(label)).expect("Error getting box index");
            let b = boxes.get_mut(box_index).expect("Error getting box");
            b.retain(|l| l.id != label);
        }
    }
    let mut focusing_power = 0;
    for (box_num, b) in boxes.iter().enumerate() {
        let box_power = box_num + 1;
        for (slot_index, slot) in b.iter().enumerate() {
            let slot_num = slot_index + 1;
            focusing_power += box_power * slot_num * slot.val;
        }
    }
    println!("Part 2: {}", focusing_power);
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
