use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum GameError {
    GameAlreadyStarted,
    TooManyPlayers,
    NameTooShort,
    NameAlreadyTaken,
    NotEnoughPlayers,
    GameNotStarted,
    DeckNotReady,
    HandNotStarted,
    CardNotFound,
}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GameError::GameAlreadyStarted => write!(f, "Game already started"),
            GameError::TooManyPlayers => write!(f, "Too many players"),
            GameError::NameTooShort => write!(f, "Name too short"),
            GameError::NameAlreadyTaken => write!(f, "Name already taken"),
            GameError::NotEnoughPlayers => write!(f, "Not enough players"),
            GameError::GameNotStarted => write!(f, "Game not started"),
            GameError::DeckNotReady => write!(f, "Deck not ready"),
            GameError::HandNotStarted => write!(f, "Hand not started"),
            GameError::CardNotFound => write!(f, "Card not found"),
        }
    }
}
