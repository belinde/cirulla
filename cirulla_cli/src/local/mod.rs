use cirulla_lib::{Game, NextAction};
use std::io::stdin;

#[derive(Debug)]
pub struct LocalGame {
    game: Game,
}

impl LocalGame {
    pub fn new(players: &Vec<String>) -> LocalGame {
        let mut game = Game::new();
        players.iter().for_each(|name| {
            game.add_player(name).unwrap();
        });
        LocalGame { game }
    }

    fn draw_table(&self) {
        println!("Mazzo: {} carte", self.game.deck.len());
        println!("Table: {:?}", self.game.table);
    }

    fn ask_for_card(&self) -> String {
        let player = self.game.current_player();
        println!("{}'s turn", player.name);
        println!("Your cards: {:?}", player.hand);
        let mut card = String::new();
        stdin().read_line(&mut card).expect("Inserire una carta valida");
        
        card.trim().to_string()
    }

    pub fn start(&mut self) {
        self.game.start_game().unwrap();
        'game: loop {
            self.game.start_hand().unwrap();
            'hand: loop {
                self.game.start_round().unwrap();
                'round: loop {
                    self.draw_table();
                    let card = self.ask_for_card();
                    self.game.player_play(&card).unwrap();
                    let next_action = self.game.next_round_action().unwrap();
                    match next_action {
                        NextAction::NextPlayer => {
                            continue 'round;
                        }
                        NextAction::NextRound => {
                            continue 'hand;
                        }
                        NextAction::EndHand => {
                            let someone_wins = self.game.end_hand().unwrap();
                            if someone_wins {
                                break 'game;
                            } else {
                                continue 'game;
                            }
                        }
                    }
                }
            }
        }
    }
}
