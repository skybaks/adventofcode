use std::fs;

struct PatternType {
    horiz: Vec<String>,
    verti: Vec<String>,
}

fn main() {
    let data = read_input();
    part1(&data);
}

fn read_input() -> Vec<PatternType> {
    let contents = fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-13\\input.txt")
        .expect("Error loading input file");

    let mut patterns = Vec::new();
    let mut pattern_lines = Vec::new();
    for line in contents.lines() {
        if line.is_empty() {
            patterns.push(pattern_lines);
            pattern_lines = Vec::new();
        } else {
            pattern_lines.push(line.to_owned());
        }
    }
    patterns.push(pattern_lines);

    let mut processed_patterns = Vec::new();
    for pattern in patterns {
        let mut verticals: Vec<String> = Vec::new();
        for (i, horiz) in pattern.iter().enumerate() {
            if i == 0 {
                for _ in 0..horiz.len() {
                    verticals.push(String::new());
                }
            }
            for (c, verti) in horiz.chars().zip(verticals.iter_mut()) {
                verti.push(c);
            }
        }
        processed_patterns.push(PatternType {
            horiz: pattern,
            verti: verticals,
        });
    }

    processed_patterns
}

fn part1(patterns: &Vec<PatternType>) {
    let mut total_value = 0;
    for (i, pattern) in patterns.iter().enumerate() {
        let mut pattern_value = 0;
        if let (true, index) = test_symmetry(&pattern.verti) {
            pattern_value += index + 1;
        } else if let (true, index) = test_symmetry(&pattern.horiz) {
            pattern_value += 100 * (index + 1);
        } else {
            panic!("NO MATCHES?!?!");
        }
        println!("{}/{}: {}", 1 + i, patterns.len(), pattern_value);
        total_value += pattern_value;
    }
    println!("Part 1: {}", total_value);
}

fn test_symmetry(elements: &Vec<String>) -> (bool, usize) {
    for i in 0..elements.len() - 1 {
        let curr_elem = elements.get(i).expect("Error getting current element");
        let next_elem = elements.get(i + 1).expect("Error getting next element");
        if curr_elem == next_elem {
            let test1 = elements[..i + 1].iter().rev();
            let test2 = elements[i + 1..].iter();
            if test1.zip(test2).all(|(a, b)| a == b) {
                return (true, i);
            }
        }
    }
    return (false, 0);
}
