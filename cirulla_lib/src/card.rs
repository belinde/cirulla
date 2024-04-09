use serde::{Deserialize, Deserializer, Serialize, Serializer};
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

impl Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

fn value_from_name(name: char) -> u8 {
    match name {
        'A' => 1,
        'J' => 8,
        'Q' => 9,
        'K' => 10,
        v => v.to_digit(10).unwrap() as u8,
    }
}

impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Card, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut chars = s.chars();
        let name = chars.next().unwrap().to_ascii_uppercase();
        let suit = chars.next().unwrap().to_ascii_uppercase();

        match suit {
            'H' =>  Ok(Card::Heart(value_from_name(name))),
            'D' =>  Ok(Card::Diamond(value_from_name(name))),
            'C' =>  Ok(Card::Club(value_from_name(name))),
            'S' =>  Ok(Card::Spade(value_from_name(name))),
            _ => Err(serde::de::Error::custom("Invalid suit")),
        }
    }
}
