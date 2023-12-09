use std::cmp::Ordering;

use crate::utils::camelcards::{Card, HandType};

#[derive(Clone, Copy)]
pub struct CardHand {
    cards: [Card; 5],
    hand_type: HandType,
}

impl CardHand {
    pub fn new(cards: [Card; 5]) -> Self {
        Self {
            cards,
            hand_type: HandType::determine_card_type(cards).unwrap(),
        }
    }
}

impl Ord for CardHand {
    fn cmp(&self, other: &Self) -> Ordering {
        // Check if the hands have different types
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }
        // Check each of the cards in the hands in turn
        for i in 0..5 {
            match self.cards[i].cmp(&other.cards[i]) {
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => (),
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
        self.cards == other.cards && self.hand_type == other.hand_type
    }
}

impl Eq for CardHand {}
