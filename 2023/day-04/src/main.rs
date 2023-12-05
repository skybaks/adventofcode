use std::fs;

#[derive(Debug)]
struct GameType {
    winning: Vec<i32>,
    numbers: Vec<i32>,
}

struct CardBooking {
    points: i32,
}

fn read_numbers(text: &str) -> Vec<i32> {
    let mut numbers = Vec::new();
    for (_, elem) in text.split(" ").filter(|x| x.trim() != "").enumerate() {
        numbers.push(elem.trim().parse::<i32>().expect("Error parsing number"));
    }
    numbers
}

fn main() {
    let data = read_input();
    let mut cards = part1(&data);
    part2(&mut cards);
}

fn read_input() -> Vec<GameType> {
    let file_path = "D:\\Projects\\Code\\adventofcode\\2023\\day-04\\input.txt";
    let contents = fs::read_to_string(file_path).expect("Error reading input file");

    let mut games: Vec<GameType> = Vec::new();

    for line in contents.lines() {
        let mut split_line = line.split(":");
        let _card_num = split_line.next().expect("Error getting card number")[4..]
            .trim()
            .parse::<i32>()
            .expect("Error parsing card number");
        let mut split_line = split_line
            .next()
            .expect("Error getting number text")
            .split("|");
        let winning_num_text = split_line.next().expect("Error getting winning text");
        let number_text = split_line.next().expect("Error getting number text");

        games.push(GameType {
            winning: read_numbers(winning_num_text),
            numbers: read_numbers(number_text),
        })
    }

    games
}

fn part1(data: &Vec<GameType>) -> Vec<CardBooking> {
    let mut total_points = 0;
    let mut card_booking: Vec<CardBooking> = Vec::new();
    for game in data {
        let mut game_points: i32 = 0;
        let mut game_matches: i32 = 0;
        for number in &game.numbers {
            if game.winning.contains(number) {
                game_matches += 1;
                if game_points == 0 {
                    game_points = 1;
                } else {
                    game_points *= 2;
                }
            }
        }
        total_points += game_points;
        card_booking.push(CardBooking {
            points: game_matches,
        });
    }
    println!("Part 1: {}", total_points);

    card_booking
}

fn part2(data: &mut Vec<CardBooking>) {
    let mut total_cards = 0;

    let mut copies: Vec<i32> = vec![0; data.len()];

    for (i, card) in data.iter().enumerate() {
        let current_copies = copies.get(i).expect("Error getting current copies");
        total_cards += 1 + current_copies;
        for _ in 0..current_copies + 1 {
            for j in 0..card.points {
                let next_card = copies
                    .get_mut(i + usize::try_from(j).expect("Error converting i32 to usize") + 1)
                    .expect("Error getting copy card");
                *next_card += 1;
            }
        }
    }
    println!("Part 2: {}", total_cards);
}
