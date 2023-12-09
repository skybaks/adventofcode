use std::fs;

fn main() {
    let data = read_input();
    part12(&data);
}

fn read_input() -> Vec<Vec<i64>> {
    let contents =
        fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-09\\input.txt")
            .expect("Error reading input file");
    contents
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|v| v.parse::<i64>().expect("Error parsing number"))
                .collect::<Vec<i64>>()
        })
        .collect()
}

fn part12(data: &Vec<Vec<i64>>) {
    let mut total_value = 0;
    let mut total_value_part2 = 0;
    for seq in data {
        let mut diffs = Vec::new();
        diffs.push(seq.to_owned());
        loop {
            let last_diff = diffs.last().expect("Error getting last element");
            if last_diff.iter().all(|&n| n == 0) {
                break;
            }
            diffs.push(diff_vec(&last_diff));
        }
        
        diffs.reverse();
        let mut next_seq_num = 0;
        diffs.iter().map(|s| s.last().expect("Error getting last element")).for_each(|&l| next_seq_num += l);
        total_value += next_seq_num;

        let mut prev_seq_num = 0;
        diffs.iter().map(|s| s.first().expect("Error getting first element")).for_each(|&f| prev_seq_num = f - prev_seq_num);
        total_value_part2 += prev_seq_num;
    }
    println!("Part 1: {}", total_value);
    println!("Part 2: {}", total_value_part2);
}

fn diff_vec(seq: &Vec<i64>) -> Vec<i64> {
    let mut diff = Vec::new();
    for (i, num) in seq.iter().enumerate() {
        if let Some(next_num) = seq.get(i+1) {
            diff.push(next_num - num);
        } else {
            break;
        }
    }
    diff
}

