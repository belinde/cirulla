use cirulla_lib::{Game, NextAction};

pub struct LocalGame {
    game: Game,
    ui: crate::ui::UI,
}

impl LocalGame {
    pub fn new(players: &Vec<String>, win_at: u8) -> LocalGame {
        let mut game = Game::new(win_at);
        players.iter().for_each(|name| {
            game.add_player(name).unwrap();
        });
        LocalGame {
            game,
            ui: crate::ui::UI::new(),
        }
    }

    pub fn start(&mut self) {
        self.game.start_game().unwrap();
        'game: loop {
            self.game.start_hand().unwrap();
            'hand: loop {
                self.game.start_round().unwrap();
                'round: loop {
                    self.ui.draw_table(&self.game);
                    let card = self.ui.ask_for_card(&self.game).unwrap();
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
        self.ui.reset();
    }
}
