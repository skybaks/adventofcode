use std::fs;

fn main() {
    let data = read_input();
    part1(&data);
}

fn read_input() -> Vec<(i64, i64)> {
    let contents = fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-11\\input.txt")
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

    //println!("{:?}", galaxy_posns);
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
            .for_each(|p| p.0 += 1);
    }
    //println!("{:?}", galaxy_posns);
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
            .for_each(|p| p.1 += 1);
    }
    //println!("{:?}", galaxy_posns);

    galaxy_posns
}

fn part1(galaxy_posns: &Vec<(i64, i64)>) {
    let mut total_dist = 0;
    for (i1, posn) in galaxy_posns.iter().enumerate() {
        for (i2, pair) in galaxy_posns.iter().enumerate() {
            if pair != posn {
                let dist = (pair.0 - posn.0).abs() + (pair.1 - posn.1).abs();
                //println!("{}->{}={}", i1+1, i2+2, dist);
                total_dist += dist;
            }
        }
    }
    println!("{}", total_dist / 2);
}
