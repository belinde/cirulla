use std::fmt::Debug;

pub enum Card {
    Heart(u8),
    Diamond(u8),
    Club(u8),
    Spade(u8),
}

fn name_of(v: &u8) -> String {
    match v {
        1 => "A".to_string(),
        8 => "J".to_string(),
        9 => "Q".to_string(),
        10 => "K".to_string(),
        _ => v.to_string(),
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Heart(v) => write!(f, "{}♥", name_of(v)),
            Self::Diamond(v) => write!(f, "{}♦", name_of(v)),
            Self::Club(v) => write!(f, "{}♣", name_of(v)),
            Self::Spade(v) => write!(f, "{}♠", name_of(v)),
        }
    }
}
