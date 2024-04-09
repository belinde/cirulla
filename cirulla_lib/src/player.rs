use serde::Serialize;
use uuid::Uuid;

use crate::card::Card;
use std::fmt::Display;

#[derive(Clone, Serialize)]
pub enum Effect {
    Knocked(u8),
    DeckHandlerBroom(u8),
}

#[derive(Clone, Serialize)]
pub struct ComparativePoints {
    pub player_id: String,
    pub cards: u8,
    pub primiera: u8,
    pub diamonds: u8,
    pub pretty_seven: bool,
    pub high_ladder: bool,
    pub low_ladder: u8,
    pub cards_taken: Vec<Card>,
}

struct PrimieraEvaluation {
    diamonds: u8,
    hearts: u8,
    clubs: u8,
    spades: u8,
}

impl PrimieraEvaluation {
    fn get_max(a: u8, b: u8) -> u8 {
        if a > b {
            a
        } else {
            b
        }
    }
    pub fn check_card(&mut self, card: &Card) {
        let p = card.primiera_value();
        match card {
            Card::Diamond(_) => self.diamonds = PrimieraEvaluation::get_max(self.diamonds, p),
            Card::Heart(_) => self.hearts = PrimieraEvaluation::get_max(self.hearts, p),
            Card::Club(_) => self.clubs = PrimieraEvaluation::get_max(self.clubs, p),
            Card::Spade(_) => self.spades = PrimieraEvaluation::get_max(self.spades, p),
        }
    }
}

pub struct Player {
    pub id: String,
    pub name: String,
    pub hand: Vec<Card>,
    pub catched: Vec<Card>,
    pub brooms: u8,
    pub points: u8,
    pub hand_visible: bool,
    pub effect: Vec<Effect>,
}

impl Player {
    pub fn new(name: &str, id: Option<String>) -> Player {
        Player {
            id: id.unwrap_or(Uuid::new_v4().to_string()),
            name: name.to_string(),
            hand: Vec::new(),
            catched: Vec::new(),
            brooms: 0,
            points: 0,
            hand_visible: false,
            effect: Vec::new(),
        }
    }

    pub fn hand_points(&self) -> ComparativePoints {
        let mut all_diamonds: Vec<u8> = Vec::new();
        let mut primiera: PrimieraEvaluation = PrimieraEvaluation {
            diamonds: 0,
            hearts: 0,
            clubs: 0,
            spades: 0,
        };

        let mut cards_taken = Vec::new();

        for card in self.catched.iter() {
            cards_taken.push(card.clone());
            primiera.check_card(card);
            if let Card::Diamond(v) = card {
                all_diamonds.push(*v);
            }
        }

        let mut low_ladder = 0;
        for i in 1..8 {
            if all_diamonds.contains(&i) {
                low_ladder += 1;
            } else {
                break;
            }
        }
        if low_ladder < 3 {
            low_ladder = 0;
        }

        ComparativePoints {
            player_id: self.id.clone(),
            cards_taken,
            cards: self.catched.len() as u8,
            primiera: primiera.diamonds + primiera.hearts + primiera.clubs + primiera.spades,
            diamonds: all_diamonds.len() as u8,
            pretty_seven: all_diamonds.contains(&7),
            high_ladder: all_diamonds.contains(&8)
                && all_diamonds.contains(&9)
                && all_diamonds.contains(&10),
            low_ladder,
        }
    }

    pub fn start_game(&mut self) {
        self.points = 0;
    }

    pub fn start_hand(&mut self) {
        self.brooms = 0;
    }

    pub fn draw(&mut self, deck: &mut Vec<Card>) {
        self.hand_visible = false;
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
            self.effect.push(Effect::Knocked(10));
            self.increment_brooms(10);
            self.hand_visible = true;
        }
        if tot_points <= 9 {
            self.effect.push(Effect::Knocked(3));
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

    pub fn give_card_from_hand(&mut self, card: &str) -> Option<Card> {
        let card = card.to_ascii_uppercase();
        for (i, c) in self.hand.iter().enumerate() {
            if c.to_string().to_ascii_uppercase() == card {
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
