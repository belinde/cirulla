use crate::{card::Card, player::Player};
use itertools::Itertools;
use rand::seq::SliceRandom;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Game {
    deck: Vec<Card>,
    players: Vec<Player>,
    table: Vec<Card>,
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

        Game {
            deck,
            players: Vec::new(),
            table: Vec::new(),
        }
    }

    pub fn add_player(&mut self, name: &str) -> Result<usize, &str> {
        let key = self.players.len();
        if key >= 4 {
            return Err("Too many players");
        }
        if name.len() < 2 {
            return Err("Name too short");
        }
        for other in self.players.iter() {
            if other.name == name {
                return Err("Name already taken");
            }
        }

        self.players.push(Player::new(name));

        Ok(key)
    }

    pub fn player_draw(&mut self, player: usize) -> Result<(), &str> {
        if player >= self.players.len() {
            return Err("Player not found");
        }
        if self.deck.is_empty() {
            return Err("Deck empty");
        }
        if let Some(p) = self.players.get_mut(player) {
            p.draw(&mut self.deck);
        }

        Ok(())
    }

    pub fn player_play(&mut self, player: usize, card: &str) -> Result<(), &str> {
        if player >= self.players.len() {
            return Err("Player not found");
        }
        match self.players.get_mut(player) {
            None => Err("Player not found"),
            Some(player) => match player.find_card_in_hand(card) {
                None => Err("Card not found"),
                Some(card) => {
                    // Scopa d'assi
                    if card.value() == 1 {
                        while let Some(c) = self.table.pop() {
                            player.catch(c);
                        }
                        player.catch(card);
                        player.increment_brooms(1);
                        return Ok(());
                    }

                    // Scopa o ciapachinze
                    for k in self.table.len()..0 {
                        println!("Controllo {} carte...", k);
                        let working_cards = self.table.iter().map(|c| c.clone());
                        for permut in working_cards.permutations(k) {
                            let mut value_total = 0;
                            permut.iter().for_each(|c| value_total += c.value());
                            if value_total == card.value() || value_total + card.value() == 15 {
                                for c in permut {
                                    if let Some(key) = self.table.iter().position(|x| *x == c) {
                                        player.catch(self.table.remove(key));
                                    }
                                }
                                player.catch(card);
                                if self.table.is_empty() {
                                    player.increment_brooms(1);
                                }
                                return Ok(());
                            }
                        }
                    }

                    Ok(())
                }
            },
        }
    }

    pub fn start_round(&mut self) -> Result<(), &str> {
        if self.players.len() < 2 {
            return Err("Not enough players");
        }
        if self.deck.len() != 40 {
            return Err("Deck not ready");
        }
        for player in self.players.iter_mut() {
            player.start_hand(&mut self.deck);
        }
        for _ in 0..4 {
            self.table.push(self.deck.pop().unwrap());
        }
        Ok(())
    }

    pub fn end_round(&mut self) -> Result<(), &str> {
        for player in self.players.iter_mut() {
            player.end_hand(&mut self.deck);
        }
        self.deck.append(&mut self.table);

        self.deck.shuffle(&mut rand::thread_rng());
        Ok(())
    }
}
