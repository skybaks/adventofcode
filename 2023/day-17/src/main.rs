use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum DirType {
    North,
    South,
    East,
    West,
    None,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct DijkstraDist {
    pos: (i64, i64),
    dir: DirType,
    dir_amt: i64,
}

fn main() {
    let (data, max_len) = read_input();
    partx(&data, &max_len);
}

fn read_input() -> (HashMap<(i64, i64), i64>, (i64, i64)) {
    let contents =
        fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-17\\example2.txt")
            .expect("Error reading input file");
    let mut coords = HashMap::new();
    let mut max_len = (0, contents.lines().count() as i64);
    for (y, line) in contents.lines().enumerate() {
        max_len.0 = line.chars().count() as i64;
        for (x, c) in line.chars().enumerate() {
            coords.insert(
                (x as i64, y as i64),
                c.to_digit(10).expect("Error parsing digit") as i64,
            );
        }
    }
    (coords, max_len)
}

fn partx(points: &HashMap<(i64, i64), i64>, max_len: &(i64, i64)) {
    let mut point_costs = HashMap::new();
    let mut points_visited = HashSet::new();
    let mut points_to_visit = HashSet::new();

    point_costs.insert(DijkstraDist{pos: (0, 0), dir: DirType::None, dir_amt: 0}, 0);
    points_to_visit.insert(DijkstraDist{pos: (0, 0), dir: DirType::None, dir_amt: 0});

    let all_directions = [(-1, 0, DirType::West), (1, 0, DirType::East), (0, -1, DirType::North), (0, 1, DirType::South)];

    while points_to_visit.len() > 0 {

        let point = *points_to_visit
            .iter()
            .min_by(|&a, &b| {
                point_costs
                    .get(a)
                    .expect("Error getting cost a")
                    .cmp(point_costs.get(b).expect("Error getting cost b"))
            })
            .expect("Error getting next visit point");


        //let point = *points_to_visit.iter().next().expect("Error getting next point");
        points_to_visit.remove(&point);
        points_visited.insert(point);

        let current_cost = *point_costs.get(&point).expect("Error getting current cost");

        for (delta_x, delta_y, dir) in &all_directions {
            let new_x = point.pos.0 + delta_x;
            let new_y = point.pos.1 + delta_y;

            if new_x < 0 || new_x >= max_len.0
                || new_y < 0 || new_y >= max_len.1
                || (*dir == DirType::North && point.dir == DirType::South)
                || (*dir == DirType::South && point.dir == DirType::North)
                || (*dir == DirType::West && point.dir == DirType::East)
                || (*dir == DirType::East && point.dir == DirType::West)
            {
                continue;
            }

            let mut dir_amt = 1;
            if *dir == point.dir {
                dir_amt = point.dir_amt + 1;
                if dir_amt > 10 {
                    continue;
                }
            } else if *dir != point.dir && point.dir != DirType::None && point.dir_amt < 4 {
                continue;
            }

            let new_dist = DijkstraDist{ pos: (new_x, new_y), dir: *dir, dir_amt };
            if let Some(_) = points_visited.get(&new_dist) {
                continue;
            }

            let new_cost = *points.get(&new_dist.pos).expect("Error getting point") + current_cost;

            if !point_costs.contains_key(&new_dist) || *point_costs.get(&new_dist).expect("Error getting point cost") > new_cost {
                point_costs.insert(new_dist, new_cost);
            }
            points_to_visit.insert(new_dist);
        }
    }

    let (k, v) = point_costs
        .iter()
        .filter(|&(c, _)| c.pos.0 == max_len.0 - 1 && c.pos.1 == max_len.1 - 1 && c.dir_amt >= 4)
        .min_by(|a, b| a.1.cmp(b.1))
        .expect("Error getting best cost");
    println!("{}", v);

}
