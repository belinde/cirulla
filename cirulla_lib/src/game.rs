use crate::card::Card;
use rand::seq::SliceRandom;
use std::fmt::Debug;

pub struct Game {
    deck: Vec<Card>,
}

impl Game {
    pub fn new() -> Game {
        let mut deck = Vec::new();
        for i in 1..11 {
            deck.push(Card::Heart(i));
            deck.push(Card::Diamond(i));
            deck.push(Card::Club(i));
            deck.push(Card::Spade(i));
        }
        deck.shuffle(&mut rand::thread_rng());
        Game { deck }
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Deck: {:?}", self.deck)
    }
}
