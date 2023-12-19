use core::panic;
use std::{collections::HashSet, fs};

#[derive(Debug)]
struct DesignSpec {
    dir: String,
    amt: i64,
    color: String,
}

fn main() {
    let data = read_input();
    part1(&data);
    part2(&data);
}

fn read_input() -> Vec<DesignSpec> {
    let contents = fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-18\\input.txt")
        .expect("Error reading input file");
    let mut spec = Vec::new();
    for line in contents.lines() {
        let mut split_line = line.split_ascii_whitespace();
        let dir = split_line.next().expect("Error getting dir").to_owned();
        let amt = split_line
            .next()
            .expect("Error getting amt")
            .parse::<i64>()
            .expect("Error parsing amt");
        let color = split_line
            .next()
            .expect("Error getting color")
            .trim_matches('(')
            .trim_matches(')')
            .to_owned();
        spec.push(DesignSpec { dir, amt, color });
    }
    spec
}

fn part1(specs: &Vec<DesignSpec>) {
    let border = calc_border(specs);
    //let tested = flood_fill(&border);
    //println!("Part 1: {}", tested.len() + border.len());
    let verts = calc_verticies(&specs);
    let area = shoelace_formula(&verts);
    let interior_points = picks_theorem(area, border.len() as i64);
    println!("{:?}", interior_points + border.len() as i64);
}

fn part2(specs: &Vec<DesignSpec>) {
    let mut new_specs = Vec::new();
    for spec in specs {
        let mut my_color = spec.color.to_owned();
        let l = my_color.pop().expect("Error getting last char");
        let dir = match l {
            '0' => "R",
            '1' => "D",
            '2' => "L",
            '3' => "U",
            _ => panic!("Unexpected direction integer"),
        }.to_owned();
        new_specs.push(DesignSpec{dir, amt: i64::from_str_radix( my_color.trim_matches('#'), 16).expect("Error getting hex number"), color: String::default()})
    }
    let border = calc_border(&new_specs);
    let verts = calc_verticies(&new_specs);
    let area = shoelace_formula(&verts);
    let interior_points = picks_theorem(area, border.len() as i64);
    println!("{:?}", interior_points + border.len() as i64);
}

fn calc_border(specs: &Vec<DesignSpec>) -> HashSet<(i32, i32)> {
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
    border
}

fn calc_verticies(specs: &Vec<DesignSpec>) -> Vec<(i64, i64)> {
    let mut verticies = Vec::new();
    let mut pos = (0, 0);
    for spec in specs {
        let next_pos = match spec.dir.as_str() {
            "U" => (pos.0, pos.1 - spec.amt),
            "D" => (pos.0, pos.1 + spec.amt),
            "L" => (pos.0 - spec.amt, pos.1),
            "R" => (pos.0 + spec.amt, pos.1),
            _ => panic!("Unexpected dir"),
        };
        verticies.push(next_pos);
        pos = next_pos;
    }
    verticies
}

fn flood_fill(border: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    // flood fill
    let mut flood_stack = HashSet::new();
    flood_stack.insert((1, 1));
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
    tested
}

// https://en.wikipedia.org/wiki/Shoelace_formula
fn shoelace_formula(verticies: &Vec<(i64, i64)>) -> i64 {
    let mut area = 0;
    for i in 0..verticies.len() {
        let curr = *verticies.get(i).expect("Error getting current element");
        let next = *if let Some(vert) = verticies.get(i + 1) {
            vert
        } else {
            verticies.first().expect("Error getting first element")
        };
        area += (curr.0 * next.1) - (curr.1 * next.0);
    }
    area / 2
}

// https://en.wikipedia.org/wiki/Pick%27s_theorem
fn picks_theorem(area: i64, boundary_points: i64) -> i64 {
    // Return the number of interior points according to pick's theorem
    area - boundary_points / 2 + 1
}

fn print_coords(verticies: &Vec<(i64, i64)>, max: &(i64, i64)) {
    for y in 0..=max.1 {
        for x in 0..=max.0 {
            if verticies.contains(&(x, y)) {
                print!("X");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    println!("");
}
