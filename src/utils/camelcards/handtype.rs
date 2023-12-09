use std::collections::{hash_map::Entry, HashMap};

use crate::utils::camelcards::Card;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum HandType {
    HighCard,  // [1, 1, 1, 1, 1]
    OnePair,   // [1, 1, 1, 2]
    TwoPair,   // [1, 2, 2]
    ThreeKind, // [1, 1, 3]
    FullHouse, // [2, 3]
    FourKind,  // [1, 4]
    FiveKind,  // [5]
}

impl HandType {
    pub fn determine_hand_type(cards: [Card; 5]) -> Option<HandType> {
        // Count how many of each card there is in the hand
        let mut card_counts: HashMap<Card, usize> = HashMap::new();
        for c in cards {
            if let Entry::Vacant(e) = card_counts.entry(c) {
                e.insert(1);
            } else {
                *card_counts.get_mut(&c).unwrap() += 1;
            }
        }
        // Order the counts and check
        let mut ordered_counts = card_counts.values().copied().collect::<Vec<usize>>();
        ordered_counts.sort();
        match ordered_counts.len() {
            1 => return Some(HandType::FiveKind),
            2 => {
                if ordered_counts[1] == 4 {
                    return Some(HandType::FourKind);
                } else { // if ordered_counts[1] == 3 {
                    return Some(HandType::FullHouse);
                }
            }
            3 => {
                if ordered_counts[2] == 3 {
                    return Some(HandType::ThreeKind);
                } else { // if ordered_counts[2] == 2 {
                    return Some(HandType::TwoPair);
                }
            }
            4 => {
                return Some(HandType::OnePair);
            }
            5 => {
                return Some(HandType::HighCard);
            }
            _ => {
                return None;
            }
        }
    }

    pub fn get_joker_wild_hand_type(hand_type: HandType, jokers: usize) -> HandType {
        match hand_type {
            HandType::HighCard => {
                match jokers {
                    0 => hand_type,
                    1 => HandType::OnePair,
                    _ => panic!("Invalid hand!")
                }
            },
            HandType::OnePair => {
                match jokers {
                    0 => hand_type,
                    1 | 2 => HandType::ThreeKind,
                    _ => panic!("Invalid hand!")
                }
            },
            HandType::TwoPair => {
                match jokers {
                    0 => hand_type,
                    1 => HandType::ThreeKind,
                    2 => HandType::FourKind,
                    _ => panic!("Invalid hand!")
                }
            },
            HandType::ThreeKind => {
                match jokers {
                    0 => hand_type,
                    1 | 3 => HandType::FourKind,
                    _ => panic!("Invalid hand!")
                }
            },
            HandType::FullHouse =>  {
                match jokers {
                    0 => hand_type,
                    2 | 3 => HandType::FiveKind,
                    _ => panic!("Invalid hand!")
                }
            },
            HandType::FourKind => {
                match jokers {
                    0 => hand_type,
                    1 | 4 => HandType::FiveKind,
                    _ => panic!("Invalid hand!")
                }
            },
            HandType::FiveKind => {
                HandType::FiveKind
            },
        }
    }
}