use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
pub enum Card {
    Heart(u8),
    Diamond(u8),
    Club(u8),
    Spade(u8),
}

impl Card {
    pub fn value(&self) -> u8 {
        match self {
            Self::Heart(v) => *v,
            Self::Diamond(v) => *v,
            Self::Club(v) => *v,
            Self::Spade(v) => *v,
        }
    }

    pub fn primiera_value(&self) -> u8 {
        match self.value() {
            1 => 13,
            8 | 9 | 10 => 1,
            v => v * 2,
        }
    }

    pub fn name(&self) -> String {
        match self.value() {
            1 => "A".to_string(),
            8 => "J".to_string(),
            9 => "Q".to_string(),
            10 => "K".to_string(),
            v => v.to_string(),
        }
    }

    pub fn suit(&self) -> String {
        match self {
            Self::Heart(_) => "h".to_string(),
            Self::Diamond(_) => "d".to_string(),
            Self::Club(_) => "c".to_string(),
            Self::Spade(_) => "s".to_string(),
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.name(), self.suit())
    }
}
