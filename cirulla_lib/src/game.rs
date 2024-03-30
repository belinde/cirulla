use crate::{
    card::Card,
    catching_logic::catching_logic,
    player::{ComparativePoints, Effect, Player},
};
use rand::seq::SliceRandom;

pub struct Game {
    pub deck: Vec<Card>,
    pub players: Vec<Player>,
    pub table: Vec<Card>,
    game_started: bool,
    hand_started: bool,
    pub current_player_index: usize,
    last_player_caught: usize,
    pub win_at: u8,
}

#[derive(Debug)]
pub struct HandResult {
    pub points: Vec<ComparativePoints>,
    pub someone_wins: bool,
    pub pretty_seven: String,
    pub cards: Option<String>,
    pub primiera: Option<String>,
    pub diamonds: Option<String>,
    pub high_ladder: Option<String>,
    pub low_ladder: Option<String>,
}

impl Game {
    pub fn new(win_at: u8) -> Game {
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
            win_at,
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

        for player in self.players.iter_mut() {
            player.start_hand();
        }

        loop {
            self.deck.shuffle(&mut rand::thread_rng());
            let mut aces = 0;
            for _ in 0..4 {
                let card = self.deck.pop().unwrap();
                if card.value() == 1 {
                    aces += 1;
                }
                self.table.push(card);
            }
            if aces > 1 {
                while let Some(card) = self.table.pop() {
                    self.deck.push(card);
                }
            } else {
                break;
            }
        }

        let total_points = self.table.iter().fold(0, |acc, c| acc + c.value());

        if total_points == 15 || total_points == 30 {
            while let Some(c) = self.table.pop() {
                self.players[0].catch(c);
            }
            self.players[0].increment_brooms(total_points / 15);
            self.players[0]
                .effect
                .push(Effect::DeckHandlerBroom(total_points))
        }

        self.hand_started = true;

        Ok(())
    }

    pub fn end_hand(&mut self) -> Result<HandResult, &str> {
        if !self.hand_started {
            return Err("Hand not yet started");
        }

        let last_player = self.players.get_mut(self.last_player_caught).unwrap();
        for card in self.table.iter() {
            last_player.catch(*card);
        }

        let points = self
            .players
            .iter()
            .map(|player| player.hand_points())
            .collect::<Vec<_>>();

        let mut pretty_seven: String = "".to_string();
        let mut high_ladder = None;
        let mut low_ladder = None;
        let mut low_ladder_value = 0;
        let mut cards = None;
        let mut cards_value = 0;
        let mut diamonds = None;
        let mut diamonds_value = 0;
        let mut primiera = None;
        let mut primiera_value = 0;

        points.iter().for_each(|p| {
            if p.pretty_seven {
                pretty_seven = p.player_id.clone();
            }

            if p.high_ladder {
                high_ladder = Some(p.player_id.clone());
            }

            if p.low_ladder > low_ladder_value {
                low_ladder = Some(p.player_id.clone());
                low_ladder_value = p.low_ladder;
            } else if p.low_ladder == low_ladder_value {
                low_ladder = None;
            }

            if p.cards > cards_value {
                cards = Some(p.player_id.clone());
                cards_value = p.cards;
            } else if p.cards == cards_value {
                cards = None;
            }

            if p.diamonds > diamonds_value {
                diamonds = Some(p.player_id.clone());
                diamonds_value = p.diamonds;
            } else if p.diamonds == diamonds_value {
                diamonds = None;
            }

            if p.primiera > primiera_value {
                primiera = Some(p.player_id.clone());
                primiera_value = p.primiera;
            } else if p.primiera == primiera_value {
                primiera = None;
            }
        });

        let mut someone_wins = false;

        for player in self.players.iter_mut() {
            let mut player_hand_points = player.brooms;

            if pretty_seven == player.id {
                player_hand_points += 1;
            }
            if cards == Some(player.id.clone()) {
                player_hand_points += 1;
            }
            if diamonds == Some(player.id.clone()) {
                player_hand_points += 1;
            }
            if primiera == Some(player.id.clone()) {
                player_hand_points += 1;
            }
            if high_ladder == Some(player.id.clone()) {
                player_hand_points += 5;
            }
            if low_ladder_value > 0 && low_ladder == Some(player.id.clone()) {
                player_hand_points += low_ladder_value;
            }

            player.points += player_hand_points;
            self.deck.append(&mut player.catched);

            if player.points >= self.win_at {
                someone_wins = true;
            }
        }

        self.hand_started = false;

        self.players.rotate_left(1);

        Ok(HandResult {
            points,
            someone_wins,
            pretty_seven,
            low_ladder,
            high_ladder,
            cards,
            diamonds,
            primiera,
        })
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
        self.players
            .get_mut(self.current_player_index)
            .unwrap()
            .effect
            .clear();

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
                let last_catcher = self.players.get_mut(self.last_player_caught).unwrap();
                while let Some(card) = self.table.pop() {
                    last_catcher.catched.push(card);
                }
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
