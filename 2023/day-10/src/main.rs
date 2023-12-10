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
    let path_coords = part1(&data);
    part2(&data, &path_coords);
}

fn read_input() -> HashMap<(usize, usize), char> {
    let contents = fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-10\\input.txt")
        .expect("Error reading input file");
    let mut points = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            points.insert((x, y), c);
        }
    }
    points
}

fn part1(points: &HashMap<(usize, usize), char>) -> Vec<(usize, usize)> {
    let start = points
        .iter()
        .find(|&k| *k.1 == 'S')
        .expect("Error getting S element");

    let starting_cxns = get_connections(points, start.0);
    let mut prev_coord = *start.0;
    let mut next_coord = *starting_cxns
        .first()
        .expect("Error getting first starting connection");
    let mut total_steps = 0;
    let mut coords = vec![next_coord];
    loop {
        total_steps += 1;
        let enter_dir = get_direction(&prev_coord, &next_coord);
        let next_char = points
            .get(&next_coord)
            .expect("Error getting next connection char");
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
        coords.push(next_coord.clone());
    }
    println!("Part 1: {}", total_steps / 2);
    coords
}

fn part2(points: &HashMap<(usize, usize), char>, path_coords: &Vec<(usize, usize)>) {
    let mut enclosed_points = Vec::new();
    // use ray casting algorithm: https://en.wikipedia.org/wiki/Point_in_polygon#Ray_casting_algorithm
    for point in points.keys() {
        if path_coords.contains(point) {
            continue;
        }
        let mut rayx = point.clone();
        let mut intersectionsx = 0;
        while points.contains_key(&rayx) {
            rayx.0 += 1;
            if path_coords.contains(&rayx) {
                let c = points.get(&rayx).expect("Error getting path char");
                // Assume the ray im casting is slightly above the polygon border
                if !['-', '7', 'F', 'S'].contains(c) {
                    intersectionsx += 1;
                }
            }
        }
        let mut rayy = point.clone();
        let mut intersectionsy = 0;
        while points.contains_key(&rayy) {
            rayy.1 += 1;
            if path_coords.contains(&rayy) {
                let c = points.get(&rayy).expect("Error getting path char");
                // Assume the ray im casting is slightly to the left of the polygon border
                if !['L', '|', 'F', 'S'].contains(c) {
                    intersectionsy += 1;
                }
            }
        }

        if intersectionsx % 2 != 0 && intersectionsy % 2 != 0 {
            enclosed_points.push(point.clone());
        }
    }
    println!("Part 2: {}", enclosed_points.len());
}

fn get_connections(
    points: &HashMap<(usize, usize), char>,
    point: &(usize, usize),
) -> Vec<(usize, usize)> {
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
        ConnectionDirs::North => (start.0, start.1 - 1),
        ConnectionDirs::South => (start.0, start.1 + 1),
        ConnectionDirs::East => (start.0 + 1, start.1),
        ConnectionDirs::West => (start.0 - 1, start.1),
        _ => *start,
    }
}
