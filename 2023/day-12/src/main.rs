use std::{collections::HashMap, fs};

struct SpringCondition {
    arrangement: Vec<i64>,
    condition: Vec<char>,
}

fn main() {
    let data = read_input();
    part1(&data);
}

fn read_input() -> Vec<SpringCondition> {
    let contents = fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-12\\input.txt")
        .expect("Error reading input file");
    let mut springs = Vec::new();
    for line in contents.lines() {
        let mut split = line.split_ascii_whitespace();
        let condition_string = split.next().expect("Error getting first in split");
        let layout_string = split.next().expect("Error getting next in split");

        let conditions = condition_string.chars().collect::<Vec<char>>();
        let layout = layout_string
            .split(",")
            .map(|l| l.parse::<i64>().expect("Error parsing layout to number"))
            .collect::<Vec<i64>>();

        springs.push(SpringCondition {
            arrangement: layout,
            condition: conditions,
        });
    }

    springs
}

fn part1(springs: &Vec<SpringCondition>) {
    let mut permutations = HashMap::new();
    let mut total_value = 0;
    for spring in springs {
        let repl_num = spring.condition.iter().filter(|&c| *c == '?').count();

        if !permutations.contains_key(&repl_num) {
            let mut new_permutation = Vec::new();
            for i in 0..2u32.pow(repl_num as u32) {
                let bits: Vec<char> = (0..repl_num)
                    .map(|n| (i >> n) & 1)
                    .map(|b| if b == 0 { '.' } else { '#' })
                    .collect();
                new_permutation.push(bits);
            }
            permutations.insert(repl_num, new_permutation);
        }

        if let Some(values) = permutations.get(&repl_num) {
            let valids = values.iter().filter(|v| test_valid(spring, v)).count();
            //println!("{:?}: {}", spring.arrangement, valids);
            total_value += valids;
        }
    }
    println!("Part 1: {}", total_value);
}

fn test_valid(spring: &SpringCondition, replacements: &Vec<char>) -> bool {
    let mut condition_replaced = spring.condition.clone();
    condition_replaced
        .iter_mut()
        .filter(|c| **c == '?')
        .zip(replacements.iter())
        .for_each(|(c, r)| *c = *r);
    let mut groups = Vec::new();
    let mut group_count = 0;
    for c in &condition_replaced {
        if *c == '#' {
            group_count += 1;
        } else {
            if group_count > 0 {
                groups.push(group_count);
            }
            group_count = 0;
        }
    }
    if group_count > 0 {
        groups.push(group_count);
    }
    groups == spring.arrangement
}
