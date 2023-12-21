use std::{fs, collections::{HashMap, HashSet}};

fn main() {
    let data = read_input();
    part1_walking(&data);
}

fn read_input() -> HashMap<(i64, i64), char> {
    let contents = fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-21\\input.txt").expect("Error reading input file");
    let mut points = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            points.insert((x as i64, y as i64), c);
        }
    }
    points
}

fn part1_walking(points: &HashMap<(i64, i64), char>) {
    let target_steps = 64;
    let test_dirs = [(0, 1), (0, -1), (-1, 0), (1, 0)];
    let s_coord = points.iter().filter(|&(_, v)| *v == 'S').map(|d| *d.0).next().expect("Error getting start coord");
    let mut step_queue = HashSet::new();
    step_queue.insert((s_coord, 0_i64, (0, 0)));
    let mut dest_points = HashSet::new();
    let mut pts_tested = HashSet::new();
    while !step_queue.is_empty() {
        let (start_coord, start_steps, last_dir) = *step_queue.iter().next().expect("Error getting next coord");
        step_queue.remove(&(start_coord, start_steps, last_dir));
        pts_tested.insert((start_coord, start_steps, last_dir));
        for dir in test_dirs {

            let test_coord = (start_coord.0 + dir.0, start_coord.1 + dir.1);
            let test_steps = start_steps + 1;

            if let Some(c) = points.get(&test_coord) {
                if *c != '#' {
                    if test_steps == target_steps {
                        dest_points.insert(test_coord);
                    } else {
                        if !pts_tested.contains(&(test_coord, test_steps, dir)) {
                            step_queue.insert((test_coord, test_steps, dir));
                        }
                    }
                }
            }
        }
    }
    print_map(points, &dest_points);
    println!("{}", dest_points.len());
}

fn part1(points: &HashMap<(i64, i64), char>) {
    // unconstrained by rocks, the solution would be (n+1)^2
    // so 65^2=4225
    // a diamond shape is formed starting on S (for ever numbers) and landing every other space
}

fn print_map(points: &HashMap<(i64, i64), char>, dests: &HashSet<(i64, i64)>) {
    for y in 0..131 {
        for x in 0..131 {
            if dests.contains(&(x, y)) {
                print!("O");
            } else if let Some(c) = points.get(&(x, y)) {
                print!("{}", c);
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}
