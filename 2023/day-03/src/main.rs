use std::fs;

#[derive(Debug)]
struct NumberToken(usize, usize);

#[derive(Debug)]
struct LineData {
    number_indexes: Vec<NumberToken>,
    symbol_indexes: Vec<usize>,
    gear_indexes: Vec<usize>,
    text: String,
}

impl LineData {
    fn new() -> LineData {
        LineData {
            number_indexes: Vec::new(),
            symbol_indexes: Vec::new(),
            gear_indexes: Vec::new(),
            text: String::new(),
        }
    }
}

fn main() {
    let data = read_input();
    part1(&data);
    part2(&data);
}

fn read_input() -> Vec<LineData> {
    let file_path = "D:\\Projects\\Code\\adventofcode\\2023\\day-03\\input.txt";
    let contents = fs::read_to_string(file_path).expect("Error reading file");

    let mut line_datas: Vec<LineData> = Vec::new();

    for line in contents.lines() {
        let mut in_number = false;
        let mut line_data = LineData::new();
        line_data.text = line.to_owned();
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if in_number {
                    let last = line_data
                        .number_indexes
                        .last_mut()
                        .expect("No last item in vec?");
                    last.1 = i;
                } else {
                    in_number = true;
                    line_data.number_indexes.push(NumberToken(i, i));
                }
            } else {
                in_number = false;
                if c != '.' {
                    line_data.symbol_indexes.push(i);
                    if c == '*' {
                        line_data.gear_indexes.push(i);
                    }
                }
            }
        }
        line_datas.push(line_data);
    }

    line_datas
}

fn part1(data: &Vec<LineData>) {
    let mut total_value: i32 = 0;

    for i in 0..data.len() {
        let current_line_data = data.get(i).expect("Error getting current line data");
        'number_loop: for number_index in &current_line_data.number_indexes {
            // Valid from same line
            for symbol_index in &current_line_data.symbol_indexes {
                let owned_symbol = symbol_index.to_owned();
                if ((number_index.0 > 0 && owned_symbol >= number_index.0 - 1)
                    || owned_symbol >= number_index.0)
                    && owned_symbol <= number_index.1 + 1
                {
                    let valid_value = current_line_data.text[number_index.0..number_index.1 + 1]
                        .parse::<i32>()
                        .expect("Error parsing integer from string");
                    total_value += valid_value;
                    continue 'number_loop;
                }
            }

            // Valid from prev line
            if i > 0 {
                let prev_line_data = data.get(i - 1).expect("Error getting previous line data");
                for symbol_index in &prev_line_data.symbol_indexes {
                    let owned_symbol = symbol_index.to_owned();
                    if ((number_index.0 > 0 && owned_symbol >= number_index.0 - 1)
                        || owned_symbol >= number_index.0)
                        && owned_symbol <= number_index.1 + 1
                    {
                        let valid_value = current_line_data.text
                            [number_index.0..number_index.1 + 1]
                            .parse::<i32>()
                            .expect("Error parsing integer from string");
                        total_value += valid_value;
                        continue 'number_loop;
                    }
                }
            }

            // Valid from next line
            if i + 1 < data.len() {
                let next_line_data = data.get(i + 1).expect("Error getting next line data");
                for symbol_index in &next_line_data.symbol_indexes {
                    let owned_symbol = symbol_index.to_owned();
                    if ((number_index.0 > 0 && owned_symbol >= number_index.0 - 1)
                        || owned_symbol >= number_index.0)
                        && owned_symbol <= number_index.1 + 1
                    {
                        let valid_value = current_line_data.text
                            [number_index.0..number_index.1 + 1]
                            .parse::<i32>()
                            .expect("Error parsing integer from string");
                        total_value += valid_value;
                        continue 'number_loop;
                    }
                }
            }
        }
    }

    println!("Part 1: {}", total_value);
}

fn part2(data: &Vec<LineData>) {
    let mut total_value: i32 = 0;

    for (i, line_data) in data.iter().enumerate() {
        for gear_index in &line_data.gear_indexes {
            let mut gear_ratios: Vec<i32> = Vec::new();

            // collect numbers from current line
            for number_index in &line_data.number_indexes {
                if (*gear_index > 0 && *gear_index - 1 == number_index.1)
                    || *gear_index + 1 == number_index.0
                {
                    let gear_value = line_data.text[number_index.0..number_index.1 + 1]
                        .parse::<i32>()
                        .expect("Error parsing integer from string");
                    gear_ratios.push(gear_value);
                }
            }

            // collect numbers from prev line
            if i > 0 {
                let prev_line_data = data.get(i - 1).expect("Error getting prev line");
                for number_index in &prev_line_data.number_indexes {
                    if ((number_index.0 > 0 && *gear_index >= number_index.0 - 1)
                        || *gear_index >= number_index.0)
                        && *gear_index <= number_index.1 + 1
                    {
                        let gear_value = prev_line_data.text[number_index.0..number_index.1 + 1]
                            .parse::<i32>()
                            .expect("Error parsing integer from string");
                        gear_ratios.push(gear_value);
                    }
                }
            }

            // collect numbers from next line
            if i + 1 < data.len() {
                let next_line_data = data.get(i + 1).expect("Error getting next line");
                for number_index in &next_line_data.number_indexes {
                    if ((number_index.0 > 0 && *gear_index >= number_index.0 - 1)
                        || *gear_index >= number_index.0)
                        && *gear_index <= number_index.1 + 1
                    {
                        let gear_value = next_line_data.text[number_index.0..number_index.1 + 1]
                            .parse::<i32>()
                            .expect("Error parsing integer from string");
                        gear_ratios.push(gear_value);
                    }
                }
            }

            if gear_ratios.len() > 1 {
                let mut gear_ratio = gear_ratios.pop().expect("Error popping element");
                for gr in gear_ratios {
                    gear_ratio *= gr;
                }
                total_value += gear_ratio;
            }
        }
    }
    println!("Part 2: {}", total_value);
}
