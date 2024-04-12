use cirulla_lib::{Card, Effect, Game, HandResult, Player};
use crossterm::style::Stylize;
use crossterm::ExecutableCommand;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode},
    style::{Color, Print, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    QueueableCommand,
};
use std::cmp;
use std::collections::HashMap;
use std::{
    io::{stdout, Error, Stdout, Write},
    process,
};

const PLAYER_HEIGHT: u16 = 11;

pub struct UI {
    stdout: Stdout,
}

impl Drop for UI {
    fn drop(&mut self) {
        self.reset(false);
    }
}

impl UI {
    pub fn new() -> UI {
        let mut stdout = stdout();
        enable_raw_mode().unwrap();
        stdout.queue(Hide).unwrap().flush().unwrap();

        UI { stdout }
    }

    pub fn ask_for_input(
        &mut self,
        message: &str,
        error: &Option<String>,
    ) -> Result<String, Error> {
        let width = 3 + match error {
            Some(e) => cmp::max(e.len(), message.len()) as u16,
            None => message.len() as u16,
        };
        self.clear()?;
        self.draw_box(4, 2, width, if error.is_some() { 4 } else { 3 }, true)?;

        if let Some(err) = error {
            self.stdout.queue(MoveTo(6, 5))?.queue(Print(err))?;
        }

        self.stdout
            .queue(MoveTo(6, 3))?
            .queue(Print(message))?
            .queue(MoveTo(6, 4))?
            .queue(Print(">>> "))?;
        self.apply()?;

        let mut input = String::new();
        loop {
            match read()? {
                Event::Key(evt) => match evt.code {
                    KeyCode::Enter => {
                        break;
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    KeyCode::Esc => {
                        self.reset(true);
                        process::exit(0);
                    }
                    _ => {}
                },
                _ => {}
            }

            self.stdout
                .queue(MoveTo(6, 4))?
                .queue(Print("    "))?
                .queue(MoveTo(6, 4))?
                .queue(Print(input.as_str()))?
                .flush()?;
        }

        Ok(input)
    }

    pub fn reset(&mut self, clear: bool) {
        disable_raw_mode().unwrap();
        self.stdout.queue(Show).unwrap();
        if clear {
            self.clear().unwrap();
        }
        self.stdout.flush().unwrap();
    }

    pub fn show_hand_result(
        &mut self,
        result: &HandResult,
        players_list: &Vec<Player>,
    ) -> Result<(), Error> {
        self.clear()?;

        let mut people = HashMap::new();
        for p in players_list.iter() {
            people.insert(p.id.to_owned(), p.name.to_owned());
        }

        self.stdout
            .queue(MoveTo(2, 2))?
            .queue(Print(
                "Carte:         Denari:        Primiera:      Settebello:      Alta:          Bassa:"
                    .bold(),
            ))?
            .queue(MoveTo(2, 3))?
            .queue(Print(
                match result.cards {
                    Some(ref cards) => &people[cards],
                    None => "-",
                }))?
            .queue(MoveTo(17, 3))?
            .queue(Print(
                match result.diamonds {
                    Some(ref diamond) => &people[diamond],
                    None => "-",
                }))?
            .queue(MoveTo(32, 3))?
            .queue(Print(
                match result.primiera {
                    Some(ref primiera) => &people[primiera],
                    None => "-",
                }))?
            .queue(MoveTo(47, 3))?
            .queue(Print(&people[&result.pretty_seven]))?
            .queue(MoveTo(64, 3))?
            .queue(Print(
                match result.high_ladder {
                    Some(ref hilad) => &people[hilad],
                    None => "-",
                }))?
                .queue(MoveTo(79, 3))?
                .queue(Print(match result.low_ladder {
                    Some(ref low_ladder) => &people[low_ladder],
                    None => "-",
                }))?;

        if result.low_ladder_value > 0 {
            self.stdout
                .queue(MoveTo(86, 2))?
                .queue(Print(result.low_ladder_value.to_string()))?;
        }

        for (i, a) in result.points.iter().enumerate() {
            self.stdout
                .queue(MoveTo(2, 7 + (i as u16) * 6))?
                .queue(Print(&people[&a.player_id]))?;
            for (j, c) in a.cards_taken.iter().enumerate() {
                self.card(c, 2 + (j as u16) * 3, 8 + (i as u16) * 6, true)?;
            }
        }

        self.draw_box(0, 0, 90, result.points.len() as u16 * 6 + 8, true)?;

        self.wait_for_key('c', 40, result.points.len() as u16 * 6 + 10);
        Ok(())
    }

    pub fn draw_winner(&mut self, game: &Game) -> Result<(), Error> {
        self.clear()?;
        self.apply()?;

        let mut points: Vec<(String, u8)> = game
            .players
            .iter()
            .map(|p| (p.name.to_owned(), p.points))
            .collect();
        points.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        let last_line: u16 = game.players.len() as u16 + 7;
        self.draw_box(0, 0, 28, last_line, true)?;
        self.stdout
            .queue(MoveTo(5, 2))?
            .queue(Print("CLASSIFICA FINALE".bold()))?;

        for (i, (name, points)) in points.iter().enumerate() {
            self.stdout
                .queue(MoveTo(3, i as u16 + 4))?
                .queue(Print(format!("{}° {} - {}", i + 1, name, points)))?;
        }

        self.wait_for_key('q', 40, 22);
        self.reset(true);
        process::exit(0);
    }

    pub fn wait_for_key(&mut self, wanted: char, column: u16, row: u16) {
        self.draw_box(column, row, 30, 2, false).unwrap();
        self.stdout
            .queue(MoveTo(column + 2, row + 1))
            .unwrap()
            .queue(Print(format!("Premi `{}` per continuare...", wanted)))
            .unwrap();
        self.apply().unwrap();
        loop {
            match read().unwrap() {
                Event::Key(evt) => match evt.code {
                    KeyCode::Char(character) => {
                        if character == wanted {
                            break;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    pub fn draw_table(&mut self, game: &Game) {
        self.clear().unwrap();

        self.table(&game.table, game.deck.len(), game.win_at)
            .unwrap();

        for (i, player) in game.players.iter().enumerate() {
            self.player(player, i as u16, i == 0, i == game.current_player_index)
                .unwrap();
        }

        self.apply().unwrap();
    }

    pub fn ask_for_card(&mut self, game: &Game) -> Result<String, Error> {
        let player = game.current_player();
        let mut pointer: usize = 0;
        loop {
            self.stdout
                .queue(MoveTo(
                    16,
                    game.current_player_index as u16 * PLAYER_HEIGHT + 7,
                ))?
                .queue(Print("                 "))?
                .queue(MoveTo(
                    (16 + pointer * 6) as u16,
                    game.current_player_index as u16 * PLAYER_HEIGHT + 7,
                ))?
                .queue(Print("▀▀▀▀▀"))?
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
                        self.reset(true);
                        process::exit(0);
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
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
            .queue(Clear(ClearType::All))?
            .queue(MoveTo(0, 0))?;
        Ok(())
    }

    fn apply(&mut self) -> Result<(), Error> {
        self.stdout.flush()
    }

    fn draw_box(
        &mut self,
        column: u16,
        row: u16,
        width: u16,
        height: u16,
        thick: bool,
    ) -> Result<(), Error> {
        self.stdout
            .queue(MoveTo(column, row))?
            .queue(Print(if thick { "╔" } else { "┌" }))?
            .queue(MoveTo(column + width, row))?
            .queue(Print(if thick { "╗" } else { "┐" }))?
            .queue(MoveTo(column, row + height))?
            .queue(Print(if thick { "╚" } else { "└" }))?
            .queue(MoveTo(column + width, row + height))?
            .queue(Print(if thick { "╝" } else { "┘" }))?;

        let horizontal = if thick { "═" } else { "─" };
        for i in 1..width {
            self.stdout
                .queue(MoveTo(column + i, row))?
                .queue(Print(horizontal))?
                .queue(MoveTo(column + i, row + height))?
                .queue(Print(horizontal))?;
        }

        let vertical = if thick { "║" } else { "│" };
        for i in 1..height {
            self.stdout
                .queue(MoveTo(column, row + i))?
                .queue(Print(vertical))?
                .queue(MoveTo(column + width, row + i))?
                .queue(Print(vertical))?;
        }

        Ok(())
    }

    fn table(&mut self, table: &Vec<Card>, deck: usize, win_at: u8) -> Result<(), Error> {
        self.draw_box(40, 0, 30, 21, false)?;
        self.stdout
            .queue(MoveTo(46, 1))?
            .queue(Print(format!("Carte nel mazzo: {}", deck)))?
            .queue(MoveTo(46, 20))?
            .queue(Print(format!("Si vince a {} punti", win_at)))?;

        table.iter().enumerate().for_each(|(i, card)| {
            self.card(
                card,
                44 + (i % 4) as u16 * 6,
                (5 + (i / 4) * 4) as u16,
                true,
            )
            .unwrap();
        });

        Ok(())
    }

    fn player(
        &mut self,
        player: &Player,
        ord: u16,
        dealer: bool,
        active: bool,
    ) -> Result<(), Error> {
        self.draw_box(0, ord * PLAYER_HEIGHT, 35, PLAYER_HEIGHT - 1, active)?;

        self.stdout
            .queue(MoveTo(2, ord * PLAYER_HEIGHT + 1))?
            .queue(Print(player.name.as_str().bold()))?;

        if dealer {
            self.stdout
                .queue(MoveTo(29, ord * PLAYER_HEIGHT + 1))?
                .queue(Print("MAZZO"))?;
        }

        player.hand.iter().enumerate().for_each(|(i, card)| {
            self.card(
                card,
                16 + i as u16 * 6,
                ord * PLAYER_HEIGHT + 3,
                active || player.hand_visible,
            )
            .unwrap();
        });

        player.effect.iter().enumerate().for_each(|(pos, effect)| {
            self.stdout
                .queue(MoveTo(2, ord * PLAYER_HEIGHT + 3 + pos as u16))
                .unwrap()
                .queue(Print(match &effect {
                    Effect::DeckHandlerBroom(value) => format!("Banco a {}", value),
                    Effect::Knocked(value) => format!("Bussa da {}", value),
                }))
                .unwrap();
        });

        self.stdout
            .queue(MoveTo(2, ord * PLAYER_HEIGHT + 9))?
            .queue(Print(format!(
                "Carte: {}   Scope: {}   Punti: {}",
                player.catched.len(),
                player.brooms,
                player.points
            )))?;

        Ok(())
    }

    fn card(&mut self, card: &Card, column: u16, row: u16, show: bool) -> Result<(), Error> {
        self.draw_box(column, row, 4, 3, false)?;

        if show {
            let suit = match card {
                Card::Heart(_) => " ♥ ",
                Card::Diamond(_) => " ♦ ",
                Card::Club(_) => " ♣ ",
                Card::Spade(_) => " ♠ ",
            };

            self.stdout
                .queue(SetForegroundColor(match card {
                    Card::Heart(_) | Card::Diamond(_) => Color::Red,
                    _ => Color::Blue,
                }))?
                .queue(MoveTo(column + 1, row + 1))?
                .queue(Print(suit))?
                .queue(MoveTo(column + 1, row + 2))?
                .queue(Print("   "))?
                .queue(MoveTo(column + 2, row + 2))?
                .queue(Print(card.name()))?
                .queue(SetForegroundColor(Color::Reset))?;
        }

        Ok(())
    }
}
