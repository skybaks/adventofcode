use std::fs;

struct SpringCondition {
    arrangement: Vec<i64>,
    condition: Vec<char>,
}

fn main() {
    let data = read_input();
    part1(&data);
    //part2(&data);
}

fn read_input() -> Vec<SpringCondition> {
    let contents =
        fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-12\\example.txt")
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
    let mut total_value = 0;
    for (i, spring) in springs.iter().enumerate() {
        let repl_num = spring.condition.iter().filter(|&c| *c == '?').count();
        println!("{}", repl_num);

        let mut valids = 0;
        for i in 0..2u128.pow(repl_num as u32) {
            let bits: Vec<char> = (0..repl_num)
                .map(|n| (i >> n) & 1)
                .map(|b| if b == 0 { '.' } else { '#' })
                .collect();
            if test_valid(spring, &bits) {
                valids += 1;
            }
        }
        total_value += valids;
        println!("{}: {}", i, valids);
    }
    println!("Part 1: {}", total_value);
}

fn part2(springs: &Vec<SpringCondition>) {
    let mut new_springs = Vec::new();
    for spring in springs {
        let mut new_arrange = Vec::new();
        let mut new_condition = Vec::new();
        for i in 0..5 {
            if i > 0 {
                new_condition.push('?');
            }
            new_arrange.extend(spring.arrangement.iter());
            new_condition.extend(spring.condition.iter());
        }
        for c in &new_condition {
            print!("{}", c);
        }
        println!(" {:?}", new_arrange);
        new_springs.push(SpringCondition {
            arrangement: new_arrange,
            condition: new_condition,
        });
    }
    //part1(&new_springs);
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
