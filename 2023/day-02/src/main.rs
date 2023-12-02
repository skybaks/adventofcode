use std::fs;

#[derive(Debug)]
enum CubeType {
    Blue(i32),
    Red(i32),
    Green(i32),
}

#[derive(Debug)]
struct GameType {
    id: i32,
    rounds: Vec<(CubeType, CubeType, CubeType)>,
}

impl GameType {
    fn new() -> GameType {
        GameType {
            id: (-1),
            rounds: Vec::new(),
        }
    }
}

fn main() {
    let games = read_input();
    part1(&games);
    part2(&games);
}

fn read_input() -> Vec<GameType> {
    let file_path = "C:\\Users\\Tom\\Documents\\_cp\\adventofcode\\2023\\day-02\\input.txt";
    let contents = fs::read_to_string(file_path).expect("Error reading input file");

    let mut games: Vec<GameType> = Vec::new();

    for line in contents.lines() {
        let mut split_line = line.split(":");
        let game_prefix = split_line
            .next()
            .expect("Should be at least one element")
            .trim();
        let mut new_game = GameType::new();
        if game_prefix.starts_with("Game ") {
            new_game.id = game_prefix[5..]
                .parse::<i32>()
                .expect("Problem parsing game ID");
        }
        let game_details = split_line
            .next()
            .expect("Should be at least 2 elements")
            .trim();

        let split_details = game_details.split(";");

        for (_, c) in split_details.enumerate() {
            let split_cubes = c.split(",");
            let mut new_cubes = (CubeType::Red(0), CubeType::Green(0), CubeType::Blue(0));
            for (_, cubes) in split_cubes.enumerate() {
                let mut split_cube = cubes.trim().split(" ");
                let cubes_count = split_cube
                    .next()
                    .expect("Should have been at least a cube count")
                    .parse::<i32>()
                    .expect("Should have been able to convert cube count to i32");
                let cubes_color = split_cube
                    .next()
                    .expect("Should have been a cube color")
                    .trim();
                match cubes_color {
                    "red" => {
                        new_cubes.0 = CubeType::Red(cubes_count);
                    }
                    "green" => {
                        new_cubes.1 = CubeType::Green(cubes_count);
                    }
                    "blue" => {
                        new_cubes.2 = CubeType::Blue(cubes_count);
                    }
                    _ => {}
                }
            }
            new_game.rounds.push(new_cubes);
        }

        games.push(new_game);
    }

    games
}

fn part1(games: &Vec<GameType>) {
    let mut total_valid: i32 = 0;
    'game_loop: for game in games {
        for round in &game.rounds {
            if let CubeType::Red(amt) = round.0 {
                if amt > 12 {
                    continue 'game_loop;
                }
            }
            if let CubeType::Green(amt) = round.1 {
                if amt > 13 {
                    continue 'game_loop;
                }
            }
            if let CubeType::Blue(amt) = round.2 {
                if amt > 14 {
                    continue 'game_loop;
                }
            }
        }
        total_valid += game.id;
    }
    println!("Part 1: {}", total_valid);
}

fn part2(games: &Vec<GameType>) {
    let mut total_power: i32 = 0;
    for game in games {
        let mut min_colors = (0i32, 0i32, 0i32);
        for round in &game.rounds {
            if let CubeType::Red(amt) = round.0 {
                min_colors.0 = min_colors.0.max(amt);
            }
            if let CubeType::Green(amt) = round.1 {
                min_colors.1 = min_colors.1.max(amt);
            }
            if let CubeType::Blue(amt) = round.2 {
                min_colors.2 = min_colors.2.max(amt);
            }
        }
        total_power += min_colors.0 * min_colors.1 * min_colors.2;
    }
    println!("Part 2: {}", total_power);
}
