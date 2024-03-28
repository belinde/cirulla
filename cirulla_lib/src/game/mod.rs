mod catching_logic;

use crate::{card::Card, player::Player};
use catching_logic::catching_logic;
use rand::seq::SliceRandom;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Game {
    pub deck: Vec<Card>,
    pub players: Vec<Player>,
    pub table: Vec<Card>,
    game_started: bool,
    hand_started: bool,
    pub current_player_index: usize,
    last_player_caught: usize,
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

        Game {
            deck,
            players: Vec::new(),
            table: Vec::new(),
            game_started: false,
            hand_started: false,
            current_player_index: 0,
            last_player_caught: 1000,
        }
    }

    pub fn current_player(&self) -> &Player {
        self.players.get(self.current_player_index).unwrap()
    }

    pub fn add_player(&mut self, name: &str) -> Result<usize, &str> {
        if self.game_started {
            return Err("Game already started");
        }
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

    pub fn start_game(&mut self) -> Result<(), &str> {
        if self.players.len() < 2 {
            return Err("Not enough players");
        }

        for player in self.players.iter_mut() {
            player.start_game();
        }

        self.game_started = true;

        Ok(())
    }

    pub fn start_hand(&mut self) -> Result<(), &str> {
        if !self.game_started {
            return Err("Game not yet started");
        }
        if self.deck.len() != 40 {
            return Err("Deck not ready");
        }
        self.deck.shuffle(&mut rand::thread_rng());

        for player in self.players.iter_mut() {
            player.start_hand();
        }

        let mut total_points = 0;
        for _ in 0..4 {
            let card = self.deck.pop().unwrap();
            total_points += card.value();
            self.table.push(card);
        }
        if total_points == 15 || total_points == 30 {
            for card in self.table.iter() {
                self.players[0].catch(*card);
            }
            self.players[0].increment_brooms(total_points / 15);
        }

        self.hand_started = true;

        Ok(())
    }

    pub fn end_hand(&mut self) -> Result<bool, &str> {
        if !self.hand_started {
            return Err("Hand not yet started");
        }

        let last_player = self.players.get_mut(self.last_player_caught).unwrap();
        for card in self.table.iter() {
            last_player.catch(*card);
        }

        let mut someone_wins = false;

        for player in self.players.iter_mut() {
            player.end_hand(&mut self.deck);
            if player.points >= 51 {
                someone_wins = true;
            }
        }

        self.hand_started = false;

        Ok(someone_wins)
    }

    pub fn start_round(&mut self) -> Result<(), &str> {
        if !self.hand_started {
            return Err("Hand not yet started");
        }
        for player in self.players.iter_mut() {
            player.draw(&mut self.deck);
        }

        self.current_player_index = 0;

        Ok(())
    }

    pub fn player_play(&mut self, card: &str) -> Result<(), &str> {
        let player = self.players.get_mut(self.current_player_index).unwrap();
        let can_broom = !self.deck.is_empty();

        match player.give_card_from_hand(card) {
            None => Err("Card not found"),
            Some(card) => {
                let caught = catching_logic(&mut self.table, player, card, can_broom);
                if caught {
                    self.last_player_caught = self.current_player_index;
                }
                Ok(())
            }
        }
    }

    pub fn next_round_action(&mut self) -> Result<NextAction, &str> {
        self.current_player_index += 1;
        if self.current_player_index >= self.players.len() {
            self.current_player_index = 0;
        }

        if self
            .players
            .get(self.current_player_index)
            .unwrap()
            .hand
            .is_empty()
        {
            return if self.deck.is_empty() {
                Ok(NextAction::EndHand)
            } else {
                Ok(NextAction::NextRound)
            };
        }

        Ok(NextAction::NextPlayer)
    }
}

pub enum NextAction {
    NextPlayer,
    NextRound,
    EndHand,
}
