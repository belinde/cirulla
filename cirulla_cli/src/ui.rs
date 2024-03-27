use cirulla_lib::{Card, Game, Player};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    QueueableCommand,
};
use std::{io::{stdout, Error, Stdout, Write}, process};

pub struct UI {
    stdout: Stdout,
}

impl UI {
    pub fn new() -> UI {
        UI { stdout: stdout() }
    }

    pub fn draw_table(&mut self, game: &Game) {
        self.clear().unwrap();

        self.table(&game.table, game.deck.len()).unwrap();

        for (i, player) in game.players.iter().enumerate() {
            self.player(player, i as u16, i == game.current_player_index)
                .unwrap();
        }

        self.apply().unwrap();
    }

    pub fn ask_for_card(&mut self, game: &Game) -> Result<String, Error> {
        let player = game.current_player();
        let mut pointer: usize = 0;
        enable_raw_mode().unwrap();
        self.stdout.queue(Hide)?;
        loop {
            self.stdout
                .queue(MoveTo(11, game.current_player_index as u16 * 10 + 6))?
                .queue(Print("               ".to_string()))?
                .queue(MoveTo(
                    (11 + pointer * 6) as u16,
                    game.current_player_index as u16 * 10 + 6,
                ))?
                .queue(Print("^^^".to_string()))?
                .flush()?;

            match read()? {
                Event::Key(evt) => match evt.code {
                    KeyCode::Left => {
                        if pointer > 0 {
                            pointer -= 1;
                        } else {
                            pointer = player.hand.len() - 1;
                        }
                    }
                    KeyCode::Right => {
                        if pointer < player.hand.len() - 1 {
                            pointer += 1;
                        } else {
                            pointer = 0;
                        }
                    }
                    KeyCode::Char('q') | KeyCode::Char('c') => {
                        disable_raw_mode().unwrap();
                        self.clear().unwrap();
                        self.stdout.flush()?;
                        process::exit(0);
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        disable_raw_mode().unwrap();
                        self.stdout.queue(Show)?.flush()?;

                        return Ok(player.hand[pointer].to_string());
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn clear(&mut self) -> Result<(), Error> {
        self.stdout
            .queue(MoveTo(0, 0))?
            .queue(Clear(ClearType::All))?;
        Ok(())
    }

    fn apply(&mut self) -> Result<(), Error> {
        self.stdout.flush()
    }

    fn table(&mut self, table: &Vec<Card>, deck: usize) -> Result<(), Error> {
        self.stdout
            .queue(MoveTo(40, 1))?
            .queue(Print(format!("Carte nel mazzo: {}", deck)))?;

        table.iter().enumerate().for_each(|(i, card)| {
            self.card(
                card,
                (40 + (i % 4) as u16 * 6, (3 + (i / 4) * 4) as u16),
                true,
            )
            .unwrap();
        });

        Ok(())
    }

    fn player(&mut self, player: &Player, ord: u16, active: bool) -> Result<(), Error> {
        self.stdout
            .queue(MoveTo(0, ord * 10))?
            .queue(Print("┌─────────────────────────────┐".to_string()))?
            .queue(MoveTo(0, ord * 10 + 1))?
            .queue(Print("│                             │".to_string()))?
            .queue(MoveTo(0, ord * 10 + 2))?
            .queue(Print("│                             │".to_string()))?
            .queue(MoveTo(0, ord * 10 + 3))?
            .queue(Print("│                             │".to_string()))?
            .queue(MoveTo(0, ord * 10 + 4))?
            .queue(Print("│                             │".to_string()))?
            .queue(MoveTo(0, ord * 10 + 5))?
            .queue(Print("│                             │".to_string()))?
            .queue(MoveTo(0, ord * 10 + 6))?
            .queue(Print("│                             │".to_string()))?
            .queue(MoveTo(0, ord * 10 + 7))?
            .queue(Print("│                             │".to_string()))?
            .queue(MoveTo(0, ord * 10 + 8))?
            .queue(Print("└─────────────────────────────┘".to_string()))?
            .queue(MoveTo(1, ord * 10 + 1))?
            .queue(Print(format!("{} ({} punti)", player.name, player.points)))?;

        player.hand.iter().enumerate().for_each(|(i, card)| {
            self.card(
                card,
                (10 + i as u16 * 6, ord * 10 + 2),
                active || player.hand_visible,
            )
            .unwrap();
        });

        self.stdout
            .queue(MoveTo(1, ord * 10 + 6))?
            .queue(Print(format!("Carte: {}", player.catched.len())))?
            .queue(MoveTo(1, ord * 10 + 7))?
            .queue(Print(format!("Scope: {}", player.brooms)))?
            .queue(MoveTo(0, ord * 10 + 10))?;

        Ok(())
    }

    fn card(&mut self, card: &Card, pos: (u16, u16), show: bool) -> Result<(), Error> {
        let suit = match card {
            Card::Heart(_) => "♥",
            Card::Diamond(_) => "♦",
            Card::Club(_) => "♣",
            Card::Spade(_) => "♠",
        };

        self.stdout
            .queue(MoveTo(pos.0, pos.1))?
            .queue(Print("┌───┐".to_string()))?
            .queue(MoveTo(pos.0, pos.1 + 1))?
            .queue(Print(format!("│ {} │", if show { suit } else { " " })))?
            .queue(MoveTo(pos.0, pos.1 + 2))?
            .queue(Print(format!(
                "│ {} │",
                if show { card.name() } else { " ".to_string() }
            )))?
            .queue(MoveTo(pos.0, pos.1 + 3))?
            .queue(Print("└───┘".to_string()))?;

        Ok(())
    }
}
