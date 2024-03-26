use std::fmt::{Debug, Display};

use crate::card::Card;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    pub catched: Vec<Card>,
    pub brooms: u8,
    pub points: u8,
    pub hand_visible: bool,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            hand: Vec::new(),
            catched: Vec::new(),
            brooms: 0,
            points: 0,
            hand_visible: false,
        }
    }

    pub fn start_game(&mut self) {
        self.points = 0;
        self.brooms = 0;
        self.hand_visible = false;
    }

    pub fn end_game(&mut self) -> u8 {
        self.points
    }

    pub fn start_hand(&mut self, deck: &mut Vec<Card>) {
        self.brooms = 0;
        self.hand_visible = false;
        self.draw(deck);
    }

    pub fn end_hand(&mut self, deck: &mut Vec<Card>) {
        // TODO: conteggio punti
        self.points += self.brooms;
        deck.append(&mut self.catched);
    }

    pub fn draw(&mut self, deck: &mut Vec<Card>) {
        for _ in 0..3 {
            self.hand.push(deck.pop().unwrap());
        }
        let mut tot_points = 0;
        let mut last_value = 0;
        let mut all_equal = true;
        for card in self.hand.iter() {
            let value = card.value();
            if last_value == 0 {
                last_value = value;
            } else if last_value != value {
                all_equal = false;
            }
            tot_points += value;
        }
        if all_equal {
            self.increment_brooms(10);
            self.hand_visible = true;
        }
        if tot_points <= 9 {
            self.increment_brooms(3);
            self.hand_visible = true;
        }
    }

    pub fn catch(&mut self, card: Card) {
        self.catched.push(card);
    }

    pub fn increment_brooms(&mut self, value: u8) {
        self.brooms += value;
    }

    pub fn find_card_in_hand(&mut self, card: &str) -> Option<Card> {
        for (i, c) in self.hand.iter().enumerate() {
            if c.to_string() == card {
                return Some(self.hand.remove(i));
            }
        }

        None
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.points)
    }
}
