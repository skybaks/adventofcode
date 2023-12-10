use std::{collections::HashMap, fs};

#[derive(PartialEq, Debug, Eq)]
enum ConnectionDirs {
    West,
    East,
    North,
    South,
    None,
    All,
}

impl ConnectionDirs {
    fn complement(&self, dir: &ConnectionDirs) -> bool {
        return (*self == ConnectionDirs::West && *dir == ConnectionDirs::East)
            || (*self == ConnectionDirs::East && *dir == ConnectionDirs::West)
            || (*self == ConnectionDirs::North && *dir == ConnectionDirs::South)
            || (*self == ConnectionDirs::South && *dir == ConnectionDirs::North)
            || (*self == ConnectionDirs::All && *dir == ConnectionDirs::East)
            || (*self == ConnectionDirs::All && *dir == ConnectionDirs::West)
            || (*self == ConnectionDirs::All && *dir == ConnectionDirs::South)
            || (*self == ConnectionDirs::All && *dir == ConnectionDirs::North)
            || (*self == ConnectionDirs::West && *dir == ConnectionDirs::All)
            || (*self == ConnectionDirs::East && *dir == ConnectionDirs::All)
            || (*self == ConnectionDirs::North && *dir == ConnectionDirs::All)
            || (*self == ConnectionDirs::South && *dir == ConnectionDirs::All);
    }

    fn get_complement(&self) -> ConnectionDirs {
        match *self {
            ConnectionDirs::North => ConnectionDirs::South,
            ConnectionDirs::South => ConnectionDirs::North,
            ConnectionDirs::East => ConnectionDirs::West,
            ConnectionDirs::West => ConnectionDirs::East,
            ConnectionDirs::All => ConnectionDirs::All,
            ConnectionDirs::None => ConnectionDirs::None,
        }
    }

    fn equivalent(&self, other: &ConnectionDirs) -> bool {
        return (*self == ConnectionDirs::All && *other != ConnectionDirs::None)
            || (*self != ConnectionDirs::None && *other == ConnectionDirs::All)
            || self == other;
    }
}

fn main() {
    let data = read_input();
    part1(&data);
}

fn read_input() -> HashMap<(usize, usize), char> {
    let contents =
        fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-10\\input.txt")
            .expect("Error reading input file");
    let mut points = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            points.insert((x, y), c);
        }
    }
    points
}

fn part1(points: &HashMap<(usize, usize), char>) {
    let start = points
        .iter()
        .find(|&k| *k.1 == 'S')
        .expect("Error getting S element");
    //println!("{:?}", start);

    let starting_cxns = get_connections(points, start.0);
    //println!("{:?}", starting_cxns);
    let mut prev_coord = *start.0;
    let mut next_coord = *starting_cxns
        .first()
        .expect("Error getting first starting connection");
    let mut total_steps = 0;
    loop {
        total_steps += 1;
        let enter_dir = get_direction(&prev_coord, &next_coord);
        let next_char = points.get(&next_coord).expect("Error getting next connection char");
        if *next_char == 'S' {
            break;
        }
        let next_dirs = char_dirs(next_char);
        let next_dir = next_dirs
            .iter()
            .find(|d| !d.equivalent(&enter_dir.get_complement()))
            .expect("Error getting next dir");
        prev_coord = next_coord;
        next_coord = get_coord(&next_coord, next_dir);
        //println!("{:?}-> {} ->{:?}", enter_dir, next_char, next_dir);
    }
    println!("Part 1: {}", total_steps/2);
}

fn get_connections(
    points: &HashMap<(usize, usize), char>,
    point: &(usize, usize),
) -> Vec<(usize, usize)> {
    // left (x-1), right (x+1), up (y-1), down (y+1)
    // remember to bounds check to prevent underflow
    let (x, y) = point.clone();
    let mut connections: Vec<(usize, usize)> = Vec::new();
    if x > 0 && points.contains_key(&(x - 1, y)) && connection_valid(points, point, &(x - 1, y)) {
        connections.push((x - 1, y));
    }
    if points.contains_key(&(x + 1, y)) && connection_valid(points, point, &(x + 1, y)) {
        connections.push((x + 1, y));
    }
    if y > 0 && points.contains_key(&(x, y - 1)) && connection_valid(points, point, &(x, y - 1)) {
        connections.push((x, y - 1));
    }
    if points.contains_key(&(x, y + 1)) && connection_valid(points, point, &(x, y + 1)) {
        connections.push((x, y + 1));
    }
    connections
}

fn connection_valid(
    points: &HashMap<(usize, usize), char>,
    point1: &(usize, usize),
    point2: &(usize, usize),
) -> bool {
    let dir = get_direction(point1, point2);
    let c1 = points
        .get(point1)
        .expect("Error getting element for point 1");
    let c2 = points
        .get(point2)
        .expect("Error getting element for point 2");
    let dirs1 = char_dirs(c1);
    let dirs2 = char_dirs(c2);

    for d1 in &dirs1 {
        for d2 in &dirs2 {
            if d1.equivalent(&dir) && d2.equivalent(&dir.get_complement()) {
                //println!("{}->{} {:?}: {:?}, {:?}", c1, c2, dir, d1, d2);
                return true;
            }
        }
    }

    return false;
}

fn char_dirs(c: &char) -> [ConnectionDirs; 2] {
    match c {
        '|' => [ConnectionDirs::North, ConnectionDirs::South],
        '-' => [ConnectionDirs::West, ConnectionDirs::East],
        'L' => [ConnectionDirs::North, ConnectionDirs::East],
        'J' => [ConnectionDirs::North, ConnectionDirs::West],
        '7' => [ConnectionDirs::South, ConnectionDirs::West],
        'F' => [ConnectionDirs::South, ConnectionDirs::East],
        '.' => [ConnectionDirs::None, ConnectionDirs::None],
        'S' => [ConnectionDirs::All, ConnectionDirs::All],
        _ => panic!("Error unexpected character"),
    }
}

fn get_direction(point1: &(usize, usize), point2: &(usize, usize)) -> ConnectionDirs {
    let diff = (
        i64::try_from(point2.0).expect("Error casting to i64")
            - i64::try_from(point1.0).expect("Error casting to i64"),
        i64::try_from(point2.1).expect("Error casting to i64")
            - i64::try_from(point1.1).expect("Error casting to i64"),
    );

    if diff.0 == 1 && diff.1 == 0 {
        ConnectionDirs::East
    } else if diff.0 == -1 && diff.1 == 0 {
        ConnectionDirs::West
    } else if diff.0 == 0 && diff.1 == 1 {
        ConnectionDirs::South
    } else if diff.0 == 0 && diff.1 == -1 {
        ConnectionDirs::North
    } else {
        ConnectionDirs::None
    }
}

fn get_coord(start: &(usize, usize), dir: &ConnectionDirs) -> (usize, usize) {
    match dir {
        ConnectionDirs::North => (start.0, start.1-1),
        ConnectionDirs::South => (start.0, start.1+1),
        ConnectionDirs::East => (start.0+1, start.1),
        ConnectionDirs::West => (start.0-1, start.1),
        _ => *start
    }
}
