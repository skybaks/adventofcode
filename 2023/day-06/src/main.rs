use std::fs;

#[derive(Debug)]
struct RaceInfo {
    time_ms: i64,
    dist_mm: i64,
}

impl RaceInfo {
    fn predict_race(&self, charge_ms: i64) -> bool {
        let driving_ms = self.time_ms - charge_ms;
        let distance_mm = charge_ms * driving_ms;
        distance_mm > self.dist_mm
    }
}

fn main() {
    let (data, data2) = read_input();
    part1(&data);
    part2(&data2);
}

fn read_input() -> (Vec<RaceInfo>, RaceInfo) {
    let contents = fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-06\\input.txt")
        .expect("Error reading input file");
    let mut times: Vec<i64> = Vec::new();
    let mut dists: Vec<i64> = Vec::new();

    let mut race2 = RaceInfo {
        time_ms: -1,
        dist_mm: -1,
    };

    for line in contents.lines() {
        if line.starts_with("Time:") {
            times.extend(
                line[5..]
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<i64>().expect("Error parsing race time"))
                    .into_iter(),
            );

            race2.time_ms = line[5..]
                .replace(" ", "")
                .parse::<i64>()
                .expect("Error parsing race time2");
        } else if line.starts_with("Distance:") {
            dists.extend(
                line[9..]
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<i64>().expect("Error parsing race distance"))
                    .into_iter(),
            );

            race2.dist_mm = line[9..]
                .replace(" ", "")
                .parse::<i64>()
                .expect("Error parsing race dist2");
        }
    }

    let mut races: Vec<RaceInfo> = Vec::new();

    for (time, dist) in times.iter().zip(dists.iter()) {
        races.push(RaceInfo {
            time_ms: *time,
            dist_mm: *dist,
        });
    }

    (races, race2)
}

fn part1(races: &Vec<RaceInfo>) {
    let mut total_margin: i64 = -1;
    for race in races {
        let mut wins = 0;
        for i in 1..race.time_ms {
            if race.predict_race(i) {
                wins += 1;
            } else if wins > 0 {
                // break out early after we pass the upper limit of the curve
                break;
            }
        }

        if total_margin < 0 {
            total_margin = wins;
        } else {
            total_margin *= wins;
        }
    }
    println!("Part 1: {}", total_margin);
}

fn part2(race: &RaceInfo) {
    let mut wins = 0;
    for i in 1..race.time_ms {
        if race.predict_race(i) {
            wins += 1;
        } else if wins > 0 {
            // break out early after we pass the upper limit of the curve
            break;
        }
    }

    println!("Part 2: {}", wins);
}
