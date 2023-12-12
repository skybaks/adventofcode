use std::fs;

fn main() {
    let data = read_input();
    part12(&data);
}

fn read_input() -> Vec<(i64, i64)> {
    let contents =
        fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-11\\input.txt")
            .expect("Error reading input file");
    let mut galaxy_posns = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in contents.lines().enumerate() {
        max_y = max_y.max(y);
        for (x, c) in line.chars().enumerate() {
            max_x = max_x.max(x);
            if c == '#' {
                galaxy_posns.push((x as i64, y as i64));
            }
        }
    }

    // For part 1 this should be drift_dist=1, for part 2 its drift_dist=1,000,000-1
    let drift_dist = 1000000 - 1;

    let mut empty_xs = Vec::new();
    for x in 0..=max_x {
        if !galaxy_posns.iter().any(|&p| p.0 == x as i64) {
            empty_xs.push(x);
        }
    }
    empty_xs.sort();
    empty_xs.reverse();
    for x in empty_xs {
        galaxy_posns
            .iter_mut()
            .filter(|p| p.0 > x as i64)
            .for_each(|p| p.0 += drift_dist);
    }
    let mut empty_ys = Vec::new();
    for y in 0..=max_y {
        if !galaxy_posns.iter().any(|&p| p.1 == y as i64) {
            empty_ys.push(y);
        }
    }
    empty_ys.sort();
    empty_ys.reverse();
    for y in empty_ys {
        galaxy_posns
            .iter_mut()
            .filter(|p| p.1 > y as i64)
            .for_each(|p| p.1 += drift_dist);
    }

    galaxy_posns
}

fn part12(galaxy_posns: &Vec<(i64, i64)>) {
    let mut total_dist = 0;
    for posn in galaxy_posns {
        for pair in galaxy_posns {
            if pair != posn {
                let dist = (pair.0 - posn.0).abs() + (pair.1 - posn.1).abs();
                total_dist += dist;
            }
        }
    }
    println!("{}", total_dist / 2);
}
