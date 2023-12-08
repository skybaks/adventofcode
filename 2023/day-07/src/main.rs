use std::fs;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CamelHand {
    class: i64,
    hand_nums: [i64; 5],
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

fn card_to_num_joker(card: &char) -> i64 {
    match card {
        'J' => 1,
        _ => card_to_num(card),
    }
}

fn classify_hand(hand: &[i64; 5]) -> i64 {
    let mut card_counts: Vec<usize> = Vec::new();
    card_counts.reserve(13);
    for i in 2..15 {
        let instances = hand.iter().filter(|&v| *v == i).count();
        card_counts.push(instances);
    }
    let num_jokers = hand.iter().filter(|&v| *v == 1).count();

    if card_counts.contains(&5)
        || (num_jokers == 5)
        || (num_jokers == 4 && card_counts.contains(&1))
        || (num_jokers == 3 && card_counts.contains(&2))
        || (num_jokers == 2 && card_counts.contains(&3))
        || (num_jokers == 1 && card_counts.contains(&4))
    {
        return 7;
    } else if card_counts.contains(&4)
        || (num_jokers == 4)
        || (num_jokers == 3 && card_counts.contains(&1))
        || (num_jokers == 2 && card_counts.contains(&2))
        || (num_jokers == 1 && card_counts.contains(&3))
    {
        return 6;
    } else if card_counts.contains(&3)
        || (num_jokers == 3)
        || (num_jokers == 2 && card_counts.contains(&1))
        || (num_jokers == 1 && card_counts.contains(&2))
    {
        if (card_counts.contains(&3) && card_counts.contains(&2))
            || (num_jokers == 3 && card_counts.contains(&2))
            || (num_jokers == 2 && card_counts.contains(&1) && card_counts.contains(&2))
            || (num_jokers == 1 && card_counts.iter().filter(|&v| *v == 2).count() == 2)
        {
            return 5;
        } else {
            return 4;
        }
    } else if card_counts.contains(&2)
        || (num_jokers == 2)
        || (num_jokers == 1 && card_counts.contains(&1))
    {
        if card_counts.iter().filter(|&v| *v == 2).count() == 2
            || (card_counts.contains(&2) && num_jokers == 2)
            || (card_counts.contains(&2) && num_jokers == 1 && card_counts.contains(&1))
        {
            return 3;
        } else {
            return 2;
        }
    } else {
        return 1;
    }
}

fn main() {
    let (data, data_joker) = read_input();
    partx(&data, 1);
    partx(&data_joker, 2);
}

fn read_input() -> (Vec<CamelHand>, Vec<CamelHand>) {
    let contents = fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-07\\input.txt")
        .expect("Error reading input file");

    let mut hands: Vec<CamelHand> = Vec::new();
    let mut hands_joker: Vec<CamelHand> = Vec::new();

    for line in contents.lines() {
        let mut split_line = line.split_ascii_whitespace();
        let hand_str = split_line.next().expect("Error getting hand string").trim();
        let mut hand_nums = [0; 5];
        for (c, i) in hand_str.chars().zip(hand_nums.iter_mut()) {
            *i = card_to_num(&c);
        }
        let mut hand_nums_joker = [0; 5];
        for (c, i) in hand_str.chars().zip(hand_nums_joker.iter_mut()) {
            *i = card_to_num_joker(&c);
        }
        let bid = split_line
            .next()
            .expect("Error getting score string")
            .trim()
            .parse::<i64>()
            .expect("Error parsing score number");

        let class = classify_hand(&hand_nums);
        let class_joker = classify_hand(&hand_nums_joker);

        hands.push(CamelHand {
            class,
            hand_nums,
            bid,
        });
        hands_joker.push(CamelHand {
            class: class_joker,
            hand_nums: hand_nums_joker,
            bid,
        })
    }

    hands.sort();
    hands_joker.sort();

    (hands, hands_joker)
}

fn partx(hands: &Vec<CamelHand>, part: i64) {
    let total_winnings = hands
        .iter()
        .enumerate()
        .map(|t| t.1.bid * (i64::try_from(t.0).expect("Error converting index to int") + 1))
        .fold(0, |a, b| a + b);
    println!("Part {}: {}", part, total_winnings);
}
