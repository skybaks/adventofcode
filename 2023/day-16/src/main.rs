use std::{fs, collections::{HashMap, HashSet}};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direct {
    North,
    South,
    East,
    West
}

struct MirrorField {
    objects: HashMap<(i64, i64), char>,
    length: (i64, i64),
}

fn main() {
    let data = read_input();
    part1(&data);
}

fn read_input() -> MirrorField {
    let contents = fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-16\\input.txt").expect("Error reading input file");
    let mut item = HashMap::new();
    let mut length = (0, 0);
    for (y, line) in contents.lines().enumerate() {
        length.1 = length.1.max(y as i64);
        for (x, c) in line.chars().enumerate() {
            length.0 = length.0.max(y as i64);
            match c {
                '.' => {},
                _ => { item.insert((x as i64, y as i64), c); }
            }
        }
    }
    MirrorField{ objects: item, length }
}

fn part1(reflectors: &MirrorField) {
    let mut touched = Vec::new();
    // start at (0, 0) move east
    project_beam((-1, 0), Direct::East, reflectors, &mut touched);
    let mut energized_coords = HashSet::new();
    for (x, y, _) in touched {
        if !energized_coords.contains(&(x, y)) {
            energized_coords.insert((x, y));
        }
    }
    println!("Part 1: {}", energized_coords.len());
    //for y in 0..reflectors.length.1 {
    //    for x in 0..reflectors.length.0 {
    //        if let Some(_) = energized_coords.get(&(x, y)) {
    //            print!("#");
    //        } else {
    //            print!(".");
    //        }
    //    }
    //    print!("\n");
    //}
}

fn project_beam(start: (i64, i64), dir: Direct, reflectors: &MirrorField, touched: &mut Vec<(i64, i64, Direct)>) {
    let next_coord = match dir {
        Direct::North => (start.0, start.1 - 1),
        Direct::South => (start.0, start.1 + 1),
        Direct::East => (start.0 + 1, start.1),
        Direct::West => (start.0 - 1, start.1),
    };

    if next_coord.0 < 0
        || next_coord.1 < 0
        || next_coord.0 > reflectors.length.0
        || next_coord.1 > reflectors.length.1
    {
        return;
    }

    // Put a stop to loops
    for same_coord in touched.iter().filter(|c| c.0 == next_coord.0 && c.1 == next_coord.1) {
        if same_coord.2 == dir {
            return;
        }
    }

    touched.push((next_coord.0, next_coord.1, dir));

    if let Some(reflect_char) = reflectors.objects.get(&next_coord) {
        if ((*reflect_char == '|' && (dir == Direct::North || dir == Direct::South)))
            || (*reflect_char == '-' && (dir == Direct::East || dir == Direct::West))
        {
            project_beam(next_coord, dir, reflectors, touched);
        } else if *reflect_char == '|' {
            project_beam(next_coord, Direct::North, reflectors, touched);
            project_beam(next_coord, Direct::South, reflectors, touched);
        } else if *reflect_char == '-' {
            project_beam(next_coord, Direct::East, reflectors, touched);
            project_beam(next_coord, Direct::West, reflectors, touched);
        } else if *reflect_char == '\\' {
            let next_dir = match dir {
                Direct::North => Direct::West,
                Direct::South => Direct::East,
                Direct::East => Direct::South,
                Direct::West => Direct::North,
            };
            project_beam(next_coord, next_dir, reflectors, touched);
        } else if *reflect_char == '/' {
            let next_dir = match dir {
                Direct::North => Direct::East,
                Direct::South => Direct::West,
                Direct::East => Direct::North,
                Direct::West => Direct::South,
            };
            project_beam(next_coord, next_dir, reflectors, touched);
        }
    } else {
        project_beam(next_coord, dir, reflectors, touched);
    }
}
