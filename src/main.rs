use std::{
    cmp::Ordering,
    collections::HashMap,
    env,
    fs::{self},
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, Clone, Copy)]
struct Hand {
    cards: [char; 5],
    hand_type: HandType,
    bid: u64,
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for (i, c) in self.cards.into_iter().enumerate() {
            let left = get_hand_card_value(c);
            let right = get_hand_card_value(other.cards[i]);
            if left == right {
                if i == 4 {
                    return Some(Ordering::Equal);
                }
                continue;
            } else if left < right {
                return Some(Ordering::Less);
            } else {
                return Some(Ordering::Greater);
            }
        }
        return None;
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        for (i, c) in self.cards.into_iter().enumerate() {
            if get_hand_card_value(c) == get_hand_card_value(other.cards[i]) {
                continue;
            } else {
                return false;
            }
        }
        return true;
    }
}
impl Eq for Hand {}
type Hands = Vec<Hand>;
fn main() {
    let input_path = env::var("aoc_2023_07_path").unwrap() + "/input.txt";
    let hands = parse_input(&input_path);
    let mut hands_by_type = sort_hands_by_type(hands);
    sort_hands_by_rank(&mut hands_by_type);
    let part1_result = calculate_part1(&hands_by_type);
    println!("{}", part1_result);
}
fn calculate_part1(hands_by_type: &[Vec<Hand>; 7]) -> u64 {
    let mut rank = 1;
    let mut res = 0;
    for ht in hands_by_type {
        for h in ht {
            res += h.bid * rank;
            rank += 1;
        }
    }
    return res;
}
fn sort_hands_by_rank(hands: &mut [Vec<Hand>; 7]) {
    for hand in hands {
        hand.sort();
    }
}
fn sort_hands_by_type(hands: Hands) -> [Hands; 7] {
    let mut five_of_a_kind_hands: Vec<Hand> = vec![];
    let mut four_of_a_kind_hands: Vec<Hand> = vec![];
    let mut full_house_hands: Vec<Hand> = vec![];
    let mut three_of_a_kind_hands: Vec<Hand> = vec![];
    let mut two_pair_hands: Vec<Hand> = vec![];
    let mut one_pair_hands: Vec<Hand> = vec![];
    let mut high_card_hands: Vec<Hand> = vec![];

    for hand in hands {
        match hand.hand_type {
            HandType::FiveOfAKind => five_of_a_kind_hands.push(hand),
            HandType::FourOfAKind => four_of_a_kind_hands.push(hand),
            HandType::FullHouse => full_house_hands.push(hand),
            HandType::ThreeOfAKind => three_of_a_kind_hands.push(hand),
            HandType::TwoPair => two_pair_hands.push(hand),
            HandType::OnePair => one_pair_hands.push(hand),
            HandType::HighCard => high_card_hands.push(hand),
        }
    }
    let result = [
        high_card_hands,
        one_pair_hands,
        two_pair_hands,
        three_of_a_kind_hands,
        full_house_hands,
        four_of_a_kind_hands,
        five_of_a_kind_hands,
    ];
    return result;
}
fn get_hand_card_value(c: char) -> u64 {
    match c {
        'T' => return 10,
        'J' => return 1,
        'Q' => return 11,
        'K' => return 12,
        'A' => return 13,
        n => {
            if n.is_ascii_digit() {
                return n.to_digit(10).unwrap() as u64;
            }
            panic!("Error matching card name and value");
        }
    }
}
fn get_hand_type(cards: &[char; 5]) -> HandType {
    let mut set: HashMap<char, u64> = HashMap::new();
    let mut arr_matches: Vec<u64> = vec![];
    for c in cards {
        if !set.contains_key(c) {
            set.insert(*c, 1);
        } else {
            let a = set.get_mut(c).unwrap();
            *a += 1;
        }
    }
    let jokers = *set.get(&'J').unwrap_or(&0);
    for entry in set {
        arr_matches.push(entry.1);
    }
    arr_matches.sort_by(|a, b| b.cmp(a));
    let max = arr_matches[0];
    if max == 5 {
        return HandType::FiveOfAKind;
    }
    let next = arr_matches[1];
    if max == 4 {
        if jokers == 4 {
            return HandType::FiveOfAKind;
        }
        if jokers == 1 {
            return HandType::FiveOfAKind;
        }
        return HandType::FourOfAKind;
    } else if max == 3 {
        if next == 2 {
            if jokers == 3 {
                return HandType::FiveOfAKind;
            }
            if jokers == 2 {
                return HandType::FiveOfAKind;
            } else {
                return HandType::FullHouse;
            }
        } else if next == 1 {
            if jokers == 3 {
                return HandType::FourOfAKind;
            }
            if jokers == 1 {
                return HandType::FourOfAKind;
            } else {
                return HandType::ThreeOfAKind;
            }
        }
    } else if max == 2 {
        if next == 2 {
            if jokers == 2 {
                return HandType::FourOfAKind;
            } else if jokers == 1 {
                return HandType::FullHouse;
            } else {
                return HandType::TwoPair;
            }
        } else if next == 1 {
            if jokers == 2 {
                return HandType::ThreeOfAKind;
            } else if jokers == 1 {
                return HandType::ThreeOfAKind;
            } else {
                return HandType::OnePair;
            }
        }
    } else if max == 1 {
        if jokers == 1 {
            return HandType::OnePair;
        }
        return HandType::HighCard;
    }

    panic!("error getting hand type");
}
fn parse_input(input_path: &String) -> Hands {
    let file = fs::File::open(input_path).unwrap();
    let reader = BufReader::new(file);
    let mut hands: Vec<Hand> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let mut arr: [&str; 2] = ["", ""];
        line.split(' ').for_each(|x| {
            if arr[0] == "" {
                arr[0] = x;
                return;
            } else {
                arr[1] = x;
                return;
            }
        });
        let mut cards: [char; 5] = ['1', '1', '1', '1', '1'];
        for (i, c) in arr[0].char_indices() {
            cards[i] = c;
        }

        let hand_type = get_hand_type(&cards);
        let bid = u64::from_str_radix(arr[1], 10).unwrap();
        let hand = Hand {
            cards,
            hand_type,
            bid,
        };
        hands.push(hand);
    }
    return hands;
}
