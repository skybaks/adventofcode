use std::{fs, fmt};

#[derive(Debug, Clone)]
struct Rock {
    x: usize, y: usize, c: char
}

#[derive(Debug, Clone)]
struct ReflectorDish {
    rocks: Vec<Rock>,
    length: (usize, usize),
}

impl fmt::Display for ReflectorDish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lines = String::new();
        for y in 0..=self.length.1 {
            for x in 0..=self.length.0 {
                if let Some(r) = self.rocks.iter().find(|r| r.x == x && r.y == y) {
                    lines.push(r.c);
                } else {
                    lines.push('.');
                }
            }
            lines.push('\n');
        }
        write!(f, "{}", lines)
    }
}

fn main() {
    let (data, mut dish) = read_input();
    part1(&data);
    part2(&mut dish);
}

fn read_input() -> (Vec<String>, ReflectorDish) {
    let contents = fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-14\\input.txt")
        .expect("Error reading input file");
    let mut verticals = Vec::new();
    let mut rocks = Vec::new();
    let mut length = (0, 0);
    for (i, line) in contents.lines().enumerate() {
        length.1 = length.1.max(i);
        for (x, c) in line.chars().enumerate() {
            length.0 = length.0.max(x);
            match c {
                'O' => { rocks.push(Rock{x, y: i, c}); },
                '#' => { rocks.push(Rock{x, y: i, c}); },
                _ => {},
            }
        }

        if i == 0 {
            for _ in 0..line.len() {
                verticals.push(String::new());
            }
        }
        for (c, verti) in line.chars().zip(verticals.iter_mut()) {
            verti.push(c);
        }
    }
    let dish = ReflectorDish{rocks, length};
    (verticals, dish)
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
    println!("Part 1: {}", total_value);
}

fn part2(dish: &mut ReflectorDish) {
    // 1 cycle = north -> west -> south -> east

    let mut cycle_value = 0;
    // Use 1000 and count on the rocks to be on a cycle
    let cycles = 1000;
    for _ in 0..cycles {
        // north
        dish.rocks.sort_by(|a, b| a.y.cmp(&b.y));
        for x in 0..=dish.length.0 {
            let mut curr_min = 0;
            for curr_rock in dish.rocks.iter_mut().filter(|r| r.x == x) {
                if curr_rock.c == '#' {
                    curr_min = curr_rock.y + 1;
                } else {
                    curr_rock.y = curr_min;
                    curr_min += 1;
                }
            }
        }

        // west
        dish.rocks.sort_by(|a, b| a.x.cmp(&b.x));
        for y in 0..=dish.length.1 {
            let mut curr_min = 0;
            for curr_rock in dish.rocks.iter_mut().filter(|r| r.y == y) {
                if curr_rock.c == '#' {
                    curr_min = curr_rock.x + 1;
                } else {
                    curr_rock.x = curr_min;
                    curr_min += 1;
                }
            }
        }

        // south
        dish.rocks.sort_by(|a, b| a.y.cmp(&b.y));
        //dish.rocks.reverse();
        for x in 0..=dish.length.0 {
            let mut curr_min = dish.length.1;
            for curr_rock in dish.rocks.iter_mut().filter(|r| r.x == x).rev() {
                if curr_rock.c == '#' {
                    if curr_rock.y > 0 {
                        curr_min = curr_rock.y - 1;
                    }
                } else {
                    curr_rock.y = curr_min;
                    if curr_min > 0 {
                        curr_min -= 1;
                    }
                }
            }
        }

        // east
        dish.rocks.sort_by(|a, b| a.x.cmp(&b.x));
        for y in 0..=dish.length.1 {
            let mut curr_min = dish.length.0;
            for curr_rock in dish.rocks.iter_mut().filter(|r| r.y == y).rev() {
                if curr_rock.c == '#' {
                    if curr_rock.x > 0 {
                        curr_min = curr_rock.x - 1;
                    }
                } else {
                    curr_rock.x = curr_min;
                    if curr_min > 0 {
                        curr_min -= 1;
                    }
                }
            }
        }

        cycle_value = calc_value(dish);
    }
    println!("Part 2: {}", cycle_value);


}

fn calc_value(dish: &ReflectorDish) -> usize {
    let mut current_value = 0;
    for rock in dish.rocks.iter().filter(|r| r.c == 'O') {
        let val = (dish.length.1 + 1) - rock.y;
        current_value += val;
    }
    current_value
}
