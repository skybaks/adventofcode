use std::fs;

fn main() {
    let data = read_input();
    part1(&data.1);
}

fn read_input() -> (Vec<String>, Vec<String>) {
    let contents = fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-14\\input.txt")
        .expect("Error reading input file");
    let mut verticals = Vec::new();
    let mut horizontals = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        if i == 0 {
            for _ in 0..line.len() {
                verticals.push(String::new());
            }
        }
        horizontals.push(line.to_owned());
        for (c, verti) in line.chars().zip(verticals.iter_mut()) {
            verti.push(c);
        }
    }
    (horizontals, verticals)
}

fn part1(verticals: &Vec<String>) {
    let mut total_value = 0;
    for row in verticals {
        let length = row.len();
        let mut start_pos = 0;
        for group in row.split("#") {
            let rock_count = group.chars().filter(|&c| c == 'O').count();
            for pos in 0..rock_count {
                let value = length - (start_pos + pos);
                total_value += value;
            }
            start_pos += group.len() + 1;
        }
    }
    println!("{}", total_value);
}
