use std::fs;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CamelHand {
    class: i64,
    hand_nums: [i64; 5],
    hand_str: String,
    bid: i64,
}

fn card_to_num(card: &char) -> i64 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("Unexpected input card symbol {}", card),
    }
}

fn classify_hand(hand: &[i64; 5]) -> i64 {
    let mut card_counts: Vec<usize> = Vec::new();
    card_counts.reserve(13);
    for i in 2..15 {
        let instances = hand.into_iter().filter(|&v| *v == i).count();
        card_counts.push(instances);
    }

    if card_counts.contains(&5) {
        //println!("Five of a kind {:?}", hand);
        return 7;
    } else if card_counts.contains(&4) {
        //println!("Four of a kind {:?}", hand);
        return 6;
    } else if card_counts.contains(&3) {
        if card_counts.contains(&2) {
            //println!("Full house {:?}", hand);
            return 5;
        } else {
            //println!("Three of a kind {:?}", hand);
            return 4;
        }
    } else if card_counts.contains(&2) {
        if card_counts.into_iter().filter(|&v| v == 2).count() == 2 {
            //println!("Two pair {:?}", hand);
            return 3;
        } else {
            //println!("One pair {:?}", hand);
            return 2;
        }
    } else {
        //println!("High card {:?}", hand);
        return 1;
    }
}

fn main() {
    let data = read_input();
    part1(&data);
}

fn read_input() -> Vec<CamelHand> {
    let contents = fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-07\\input.txt")
        .expect("Error reading input file");

    let mut hands: Vec<CamelHand> = Vec::new();

    for line in contents.lines() {
        let mut split_line = line.split_ascii_whitespace();
        let hand_str = split_line
            .next()
            .expect("Error getting hand string")
            .trim()
            .to_owned();
        let mut hand_nums = [0; 5];
        for (c, i) in hand_str.chars().zip(hand_nums.iter_mut()) {
            *i = card_to_num(&c);
        }
        let bid = split_line
            .next()
            .expect("Error getting score string")
            .trim()
            .parse::<i64>()
            .expect("Error parsing score number");

        let class = classify_hand(&hand_nums);

        hands.push(CamelHand {
            hand_str,
            hand_nums,
            class,
            bid,
        });
    }

    hands.sort();

    hands
}

fn part1(hands: &Vec<CamelHand>) {
    let mut total_winnings = 0;
    for (i, hand) in hands.into_iter().enumerate() {
        total_winnings += hand.bid * (i64::try_from(i).expect("Erro converting index to int") + 1);
    }
    println!("Part 1: {}", total_winnings);
}
