use std::cmp::Ordering;

use crate::utils::camelcards::{Card, HandType};

#[derive(Clone, Copy)]
pub struct CardHand {
    cards: [Card; 5],
    hand_type: HandType,
    hand_type_joker_wild: HandType,
    joker_wild: bool,
}

impl CardHand {
    pub fn new(cards: [Card; 5]) -> Self {
        let hand_type = HandType::determine_hand_type(cards).unwrap();
        let jokers = cards.iter().filter(|&&c| c == Card::Jack).count();
        Self {
            cards,
            hand_type: HandType::determine_hand_type(cards).unwrap(),
            hand_type_joker_wild: HandType::get_joker_wild_hand_type(hand_type, jokers),
            joker_wild: false,
        }
    }

    /// Sets the value of the "joker_wild" field.
    pub fn set_joker_wild(&mut self, joker_wild: bool) {
        self.joker_wild = joker_wild;
    }
}

impl Ord for CardHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_type = {
            if !self.joker_wild {
                self.hand_type
            } else {
                self.hand_type_joker_wild
            }
        };
        let other_hand_type = {
            if !other.joker_wild {
                other.hand_type
            } else {
                other.hand_type_joker_wild
            }
        };
        // Check if the hands have different types
        if hand_type != other_hand_type {
            return hand_type.cmp(&other_hand_type);
        }
        // Check each of the cards in the hands in turn
        for i in 0..5 {
            let card = self.cards[i];
            let other_card = other.cards[i];
            if self.joker_wild {
                if card == Card::Jack && other_card == Card::Jack {
                    continue;
                } else if card == Card::Jack && other_card != Card::Jack {
                    return Ordering::Less;
                } else if card != Card::Jack && other_card == Card::Jack {
                    return Ordering::Greater;
                }
            }
            match card.cmp(&other_card) {
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => continue,
                Ordering::Greater => return Ordering::Greater,
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for CardHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CardHand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
            && self.hand_type == other.hand_type
            && self.hand_type_joker_wild == other.hand_type_joker_wild
            && self.joker_wild == other.joker_wild
    }
}

impl Eq for CardHand {}
