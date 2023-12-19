use std::{fs, collections::{HashSet}};

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
    let contents = fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-18\\input.txt").expect("Error reading input file");
    let mut spec = Vec::new();
    for line in contents.lines() {
        let mut split_line = line.split_ascii_whitespace();
        let dir = split_line.next().expect("Error getting dir").to_owned();
        let amt = split_line.next().expect("Error getting amt").parse::<i64>().expect("Error parsing amt");
        let color = split_line.next().expect("Error getting color").trim_matches('(').trim_matches(')').to_owned();
        spec.push(DesignSpec{dir, amt, color});
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
    let min_x = border.iter().map(|&(x, _)| x).min().expect("Error getting min x");
    let max_x = border.iter().map(|&(x, _)| x).max().expect("Error getting min x");
    let min_y = border.iter().map(|&(_, y)| y).min().expect("Error getting min y");
    let max_y = border.iter().map(|&(_, y)| y).max().expect("Error getting min y");
    //println!("{} {} {} {}", min_x, max_x, min_y, max_y);
    /*
    let mut interior_points = Vec::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if border.contains(&(x, y)) {
                continue;
            }

            let mut last_was_delim = false;
            let mut intersectionsx = 0;
            let mut rayx = (x, y);
            while rayx.0 <= max_x {
                rayx.0 += 1;
                if border.contains(&rayx) {
                    if !last_was_delim {
                        intersectionsx += 1;
                    }
                    last_was_delim = true;
                } else {
                    last_was_delim = false;
                }
            }
            let mut intersectionsy = 0;
            let mut rayy = (x, y);
            while rayy.1 <= max_y {
                rayy.1 += 1;
                if border.contains(&rayy) {
                    if !last_was_delim {
                        intersectionsy += 1;
                    }
                    last_was_delim = true;
                } else {
                    last_was_delim = false;
                }
            }

            if intersectionsx % 2 != 0 && intersectionsy % 2 != 0 {
                interior_points.push((x, y));
            }
        }
    }
    */

    // flood fill
    let mut flood_stack = HashSet::new();
    flood_stack.insert((1,1));
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

    println!("{:?}", tested.len() + border.len());
    //let interior = border.len() + interior_points.len();
    //println!("{}", interior);
    //println!("{:?}", interior_points);
}
