use std::{cmp::Ordering, collections::HashMap, convert::TryInto, fs, iter::zip};

fn read_file(filename: &str) -> String {
    let file_contents: String =
        fs::read_to_string(filename).expect("Should have been able to read file");

    return file_contents;
}

#[derive(Copy, Clone, Debug, Hash)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

impl Card {
    fn new(c: char, allow_joker: bool) -> Self {
        return match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => match allow_joker {
                true => Self::Joker,
                false => Self::Jack,
            },
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Unknown card: {}", c),
        };
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        return (*self as u8) == (*other as u8);
    }
}

impl Eq for Card {}

#[derive(Copy, Clone, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
    hand_type: Option<HandType>,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if let (Some(self_hand_type), Some(other_hand_type)) = (&self.hand_type, &other.hand_type) {
            let s = *self_hand_type as u8;
            let o = *other_hand_type as u8;

            if s > o {
                return Ordering::Greater;
            } else if s < o {
                return Ordering::Less;
            }
        } else {
            panic!("Should have hand type");
        }

        for (&self_card, &other_card) in zip(&self.cards, &other.cards) {
            let s = self_card as u8;
            let o = other_card as u8;

            if s > o {
                return Ordering::Greater;
            } else if s < o {
                return Ordering::Less;
            }
        }

        return Ordering::Equal;
    }
}

impl Hand {
    fn new(line: &str, allow_joker: bool) -> Self {
        let line_split: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
        if line_split.len() != 2 {
            panic!("Expected exactly two splits");
        }

        let cards: [Card; 5] = match line_split[0]
            .chars()
            .map(|c| Card::new(c, allow_joker))
            .collect::<Vec<Card>>()
            .try_into()
        {
            Ok(c) => c,
            Err(_) => panic!("Expected vector of length 5"),
        };

        let bid: u32 = line_split[1]
            .parse::<u32>()
            .expect("Should be able to parse bid amount");

        let mut hand = Hand {
            cards: cards,
            bid: bid,
            hand_type: None,
        };

        hand.hand_type = Some(hand.compute_hand_type(allow_joker));

        return hand;
    }

    fn compute_hand_type(&self, allow_joker: bool) -> HandType {
        let mut cards_map: HashMap<Card, u32> = HashMap::new();

        for card in self.cards {
            match cards_map.get_mut(&card) {
                Some(count) => *count += 1,
                None => _ = cards_map.insert(card, 1),
            }
        }

        if allow_joker {
            let mut max_non_joker_card: Card = Card::Ace;
            let mut max_non_joker_count: u32 = 0;
            for (&card, &count) in &cards_map {
                if card == Card::Joker {
                    continue;
                }

                if count > max_non_joker_count {
                    max_non_joker_card = card;
                    max_non_joker_count = count;
                }
            }

            cards_map.insert(
                max_non_joker_card,
                cards_map.get(&Card::Joker).unwrap_or(&0) + max_non_joker_count,
            );
            cards_map.remove(&Card::Joker);
        }

        let cards_map_len = cards_map.len();
        if cards_map_len == 1 {
            return HandType::FiveOfAKind;
        }

        if cards_map_len == 4 {
            return HandType::OnePair;
        }

        if cards_map_len == 5 {
            return HandType::HighCard;
        }

        if cards_map_len == 2 {
            for &count in cards_map.values() {
                if count == 4 {
                    return HandType::FourOfAKind;
                }
            }

            return HandType::FullHouse;
        }

        for &count in cards_map.values() {
            if count == 3 {
                return HandType::ThreeOfAKind;
            }
        }

        return HandType::TwoPair;
    }
}

fn main() {
    let filename: &str = "../hands.txt";
    let file_contents: String = read_file(filename);
    let lines: Vec<&str> = file_contents.split::<char>('\n').collect::<Vec<&str>>();

    let mut hands: Vec<Hand> = lines
        .iter()
        .map(|l| Hand::new(l, false))
        .collect::<Vec<Hand>>();
    let mut hands_with_joker: Vec<Hand> = lines
        .iter()
        .map(|l| Hand::new(l, true))
        .collect::<Vec<Hand>>();
    hands.sort_by(|h1, h2| h1.cmp(h2));
    hands_with_joker.sort_by(|h1, h2| h1.cmp(h2));

    let mut winnings: u32 = 0;
    for (i, hand) in hands.iter().enumerate() {
        winnings += (i as u32 + 1) * hand.bid;
    }

    println!("{:?}", winnings);

    let mut winnings_with_joker: u32 = 0;
    for (i, hand) in hands_with_joker.iter().enumerate() {
        winnings_with_joker += (i as u32 + 1) * hand.bid;
    }

    println!("{:?}", winnings_with_joker);
}
