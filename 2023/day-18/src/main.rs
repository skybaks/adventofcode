use std::{collections::HashSet, fs};

#[derive(Debug)]
struct DesignSpec {
    dir: String,
    amt: i64,
    color: String,
}

fn main() {
    let data = read_input();
    part1(&data);
}

fn read_input() -> Vec<DesignSpec> {
    let contents = fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-18\\input.txt")
        .expect("Error reading input file");
    let mut spec = Vec::new();
    for line in contents.lines() {
        let mut split_line = line.split_ascii_whitespace();
        let dir = split_line.next().expect("Error getting dir").to_owned();
        let amt = split_line
            .next()
            .expect("Error getting amt")
            .parse::<i64>()
            .expect("Error parsing amt");
        let color = split_line
            .next()
            .expect("Error getting color")
            .trim_matches('(')
            .trim_matches(')')
            .to_owned();
        spec.push(DesignSpec { dir, amt, color });
    }
    spec
}

fn part1(specs: &Vec<DesignSpec>) {
    let mut border = HashSet::new();
    let mut pos = (0, 0);
    for spec in specs {
        for _ in 0..spec.amt {
            let next_pos = match spec.dir.as_str() {
                "U" => (pos.0, pos.1 - 1),
                "D" => (pos.0, pos.1 + 1),
                "L" => (pos.0 - 1, pos.1),
                "R" => (pos.0 + 1, pos.1),
                _ => panic!("Unexpected dir"),
            };
            border.insert(next_pos);
            pos = next_pos;
        }
    }

    // flood fill
    let mut flood_stack = HashSet::new();
    flood_stack.insert((1, 1));
    let mut tested = HashSet::new();

    while flood_stack.len() > 0 {
        let point = *flood_stack.iter().next().expect("Error getting next point");
        flood_stack.remove(&point);

        if tested.contains(&point) {
            continue;
        } else {
            tested.insert(point);
        }

        if !border.contains(&(point.0 + 1, point.1)) {
            flood_stack.insert((point.0 + 1, point.1));
        }
        if !border.contains(&(point.0 - 1, point.1)) {
            flood_stack.insert((point.0 - 1, point.1));
        }
        if !border.contains(&(point.0, point.1 + 1)) {
            flood_stack.insert((point.0, point.1 + 1));
        }
        if !border.contains(&(point.0, point.1 - 1)) {
            flood_stack.insert((point.0, point.1 - 1));
        }
    }

    println!("Part 1: {}", tested.len() + border.len());
}
