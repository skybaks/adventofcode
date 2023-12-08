use std::{fs};

#[derive(Debug)]
struct MapDirection {
    id: String,
    dirs: [String; 2],
}

fn main() {
    let (dirs, maps) = read_input();
    part1(&dirs, &maps);
    part2(&dirs, &maps);
}

fn read_input() -> (Vec<usize>, Vec<MapDirection>) {
    let contents = fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-08\\input.txt").expect("Error opening input file");

    let mut lines = contents.lines().filter(|l| l.trim()!="");

    let mut dirs: Vec<usize> = Vec::new();
    if let Some(first_line) = lines.next() {
        for c in first_line.chars() {
            let dir_index = match c {
                'L' => 0,
                'R' => 1,
                _ => panic!("Unexpected input character")
            };
            dirs.push(dir_index);
        }
    }

    let mut maps: Vec<MapDirection> = Vec::new();
    for line in lines {
        let id = &line[0..3];
        let opt1 = &line[7..10];
        let opt2 = &line[12..15];
        let new_dir = MapDirection{ id: id.to_owned(), dirs: [opt1.to_owned(), opt2.to_owned()] };
        maps.push(new_dir);
    }

    (dirs, maps)
}

fn part1(dirs: &Vec<usize>, maps: &Vec<MapDirection>) {
    let mut total_steps = 0;
    let mut i = 0;
    let mut curr_map = maps.iter().find(|&m| m.id == "AAA").expect("Error finding AAA map");
    loop {
        if let Some(dir) = dirs.get(i) {
            let next_map_id = curr_map.dirs.get(*dir).expect("Error getting next map id");
            curr_map = maps.iter().find(|&m| *m.id == *next_map_id).expect("Error finding next map");

            total_steps += 1;
            i += 1;
        } else {
            i = 0;
        }

        if curr_map.id == "ZZZ" {
            break;
        }
    }

    println!("Part 1: {}", total_steps);
}

fn part2(dirs: &Vec<usize>, maps: &Vec<MapDirection>) {
    let mut total_steps = 0;
    let mut i = 0;
    let mut curr_nodes: Vec<&MapDirection> = maps.iter().filter(|&m| m.id.ends_with("A")).collect();
    for node in curr_nodes {
        let result = part2_1(dirs, maps, node);
        println!("{}", result);
        // The answer is the least common multiple of all these
        // computed using wolfram alpha: lcm(20569, 18727, 14429, 13201, 18113, 22411) = 10921547990923
    }
    /*
    loop {
        total_steps += 1;
        for node in curr_nodes.iter_mut() {
            if let Some(dir) = dirs.get(i) {
                let next_map_id = node.dirs.get(*dir).expect("Error getting next map id");
                *node = maps.iter().find(|&m| *m.id == *next_map_id).expect("Error finding next map");
            }
        }

        if curr_nodes.iter().all(|&m| m.id.ends_with("Z")) {
            break;
        }

        i += 1;
        if i >= dirs.len() {
            i = 0;
        }
    }
    
    println!("Part 2: {}", total_steps);
    */
}


fn part2_1(dirs: &Vec<usize>, maps: &Vec<MapDirection>, start: &MapDirection) -> i64 {
    let mut total_steps = 0;
    let mut i = 0;
    let mut curr_map = start;
    loop {
        if let Some(dir) = dirs.get(i) {
            let next_map_id = curr_map.dirs.get(*dir).expect("Error getting next map id");
            curr_map = maps.iter().find(|&m| *m.id == *next_map_id).expect("Error finding next map");

            total_steps += 1;
            i += 1;
        } else {
            i = 0;
        }

        if curr_map.id.ends_with("Z") {
            break;
        }
    }
    total_steps
}