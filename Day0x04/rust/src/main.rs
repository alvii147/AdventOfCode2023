use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    fs,
    str::Split,
};

struct ScratchCard {
    id: u32,
    winning_cards: Vec<u32>,
}

fn read_file(filename: &str) -> String {
    let file_contents: String =
        fs::read_to_string(filename).expect("Should have been able to read file");

    return file_contents;
}

fn parse_scratch_card(line: &str) -> ScratchCard {
    lazy_static! {
        static ref RE_CARD: Regex =
            Regex::new(r"Card\s+(\d+):([\d\s]*)\|([\d\s]*)").expect("Regex should compile");
        static ref RE_SPACE: Regex = Regex::new(r"\s+").expect("Regex should compile");
    }

    let caps = RE_CARD.captures(line).expect("Card should be parsed");

    let card_id = caps
        .get(1)
        .expect("Should be able to capture groups")
        .as_str()
        .parse::<u32>()
        .expect("Should be able to parse card ID");

    let mut scratched_nums: HashMap<u32, bool> = HashMap::new();

    for scratched_num_str in RE_SPACE.split(
        caps.get(3)
            .expect("Should be able to capture groups")
            .as_str(),
    ) {
        if let Ok(scratched_num) = scratched_num_str.parse::<u32>() {
            scratched_nums.insert(scratched_num, true);
        }
    }

    let mut n_wins: u32 = 0;
    for winning_num_str in RE_SPACE.split(
        caps.get(2)
            .expect("Should be able to capture groups")
            .as_str(),
    ) {
        if let Ok(winning_num) = winning_num_str.parse::<u32>() {
            if scratched_nums.contains_key(&winning_num) {
                n_wins += 1;
            }
        }
    }

    let scratch_card = ScratchCard {
        id: card_id,
        winning_cards: ((card_id + 1)..(card_id + n_wins + 1)).collect(),
    };

    return scratch_card;
}

fn main() {
    let filename: &str = "../scratchcards.txt";
    let file_contents: String = read_file(filename);
    let lines: Split<char> = file_contents.split::<char>('\n');

    let mut scratch_queue: VecDeque<u32> = VecDeque::new();
    let mut scratch_map: HashMap<u32, ScratchCard> = HashMap::new();
    let mut points_won: u32 = 0;

    for line in lines {
        let scratch_card = parse_scratch_card(line);
        if scratch_card.winning_cards.len() > 0 {
            points_won += u32::pow(2, (scratch_card.winning_cards.len() as u32) - 1);
        }

        scratch_queue.push_back(scratch_card.id);
        scratch_map.insert(scratch_card.id, scratch_card);
    }

    let mut cards_won: u32 = 0;
    while scratch_queue.len() > 0 {
        let scratch_card_id = scratch_queue
            .pop_front()
            .expect("Should be able to dequeue");
        for winning_scratch_card_id in scratch_map
            .get(&scratch_card_id)
            .expect("Should be able to look up scratch card")
            .winning_cards
            .clone()
        {
            scratch_queue.push_back(winning_scratch_card_id);
        }

        cards_won += 1;
    }

    println!("{}", points_won);
    println!("{}", cards_won);
}
