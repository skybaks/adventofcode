
use std::fs;


fn part1() {
    let file_path = "C:\\Users\\Tom\\Documents\\_cp\\adventofcode\\2023\\day-01\\input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut total_val: i32 = 0;

    for line in contents.lines() {
        let mut first_digit = -1i32;
        let mut last_digit = -1i32;

        for c in line.chars() {
            if c.is_digit(10) {
                last_digit = c.to_string().parse().unwrap();
                if first_digit < 0 {
                    first_digit = last_digit;
                }
            }
        }

        let line_value = first_digit * 10i32 + last_digit;

        total_val += line_value;
    }
    println!("Part 1 Value: {}", total_val);
}

fn part2() {
    let file_path = "C:\\Users\\Tom\\Documents\\_cp\\adventofcode\\2023\\day-01\\input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut total_value: i32 = 0;

    for line in contents.lines() {
        let mut first_digit: i32 = -1;
        let mut last_digit: i32 = -1;

        for (i, c) in line.chars().enumerate() {
            let sub = &line[i..];
            let written: i32 = if sub.starts_with("zero") {
                0
            } else if sub.starts_with("one") {
                1
            } else if sub.starts_with("two") {
                2
            } else if sub.starts_with("three") {
                3
            } else if sub.starts_with("four") {
                4
            } else if sub.starts_with("five") {
                5
            } else if sub.starts_with("six") {
                6
            } else if sub.starts_with("seven") {
                7
            } else if sub.starts_with("eight") {
                8
            } else if sub.starts_with("nine") {
                9
            } else {
                -1
            };

            if written > 0 {
                last_digit = written;
            }

            if c.is_digit(10) {
                last_digit = c.to_string().parse().unwrap();
            }

            if first_digit < 0 {
                first_digit = last_digit;
            }
        }

        let line_value: i32 = first_digit * 10i32 + last_digit;
        total_value += line_value;
    }
    println!("Part 2 Value: {}", total_value);
}

fn main() {
    part1();
    part2();
}
