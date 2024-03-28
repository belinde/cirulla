use itertools::Itertools;

use crate::{Card, Player};

pub fn catching_logic(
    table: &mut Vec<Card>,
    player: &mut Player,
    card: Card,
    can_broom: bool,
) -> bool {
    let ace_on_table = table.iter().any(|c| c.value() == 1);

    // Scopa d'assi
    if !ace_on_table && card.value() == 1 && table.len() != 0 {
        while let Some(c) = table.pop() {
            player.catch(c);
        }
        player.catch(card);
        if can_broom {
            player.increment_brooms(1);
        }
        return true;
    }

    // Presa o ciapachinze
    for k in (1..table.len() + 1).rev() {
        // The permutation() method needs to clone the cards, so we need to work on a copy of the table
        let working_cards = table.iter().map(|c| c.clone());
        for permut in working_cards.permutations(k) {
            let mut value_total = 0;
            permut.iter().for_each(|c| value_total += c.value());
            if value_total == card.value() || value_total + card.value() == 15 {
                for c in permut {
                    if let Some(key) = table.iter().position(|x| *x == c) {
                        player.catch(table.remove(key));
                    }
                }
                player.catch(card);
                if can_broom && table.is_empty() {
                    player.increment_brooms(1);
                }
                return true;
            }
        }
    }

    table.push(card);

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(values: &[u8]) -> Vec<Card> {
        values.iter().map(|v| Card::Heart(*v)).collect()
    }

    #[test]
    fn ace_catch_all() {
        let mut table = table(&[2, 3, 4]);
        let mut player = Player::new("Test");
        let card = Card::Heart(1);
        let can_broom = true;

        assert_eq!(
            catching_logic(&mut table, &mut player, card, can_broom),
            true
        );
        assert_eq!(table.len(), 0);
        assert_eq!(player.catched.len(), 4);
        assert_eq!(player.brooms, 1);
    }

    #[test]
    fn ace_catch_all_cannot_broom() {
        let mut table = table(&[2, 3, 4]);
        let mut player = Player::new("Test");
        let card = Card::Heart(1);
        let can_broom = false;

        assert_eq!(
            catching_logic(&mut table, &mut player, card, can_broom),
            true
        );
        assert_eq!(table.len(), 0);
        assert_eq!(player.catched.len(), 4);
        assert_eq!(player.brooms, 0);
    }

    #[test]
    fn ace_catch_only_ace() {
        let mut table = table(&[1, 2]);
        let mut player = Player::new("Test");
        let card = Card::Heart(1);
        let can_broom = true;

        assert_eq!(
            catching_logic(&mut table, &mut player, card, can_broom),
            true
        );
        assert_eq!(table.len(), 1);
        assert_eq!(player.catched.len(), 2);
        assert_eq!(player.brooms, 0);
    }

    #[test]
    fn ace_catch_with_15_and_ace() {
        let mut table = table(&[1, 10, 3]);
        let mut player = Player::new("Test");
        let card = Card::Heart(1);
        let can_broom = true;

        assert_eq!(
            catching_logic(&mut table, &mut player, card, can_broom),
            true
        );
        assert_eq!(table.len(), 0);
        assert_eq!(player.catched.len(), 4);
        assert_eq!(player.brooms, 1);
    }

    #[test]
    fn any_normal_catch_all() {
        let mut table = table(&[2, 3, 4]);
        let mut player = Player::new("Test");
        let card = Card::Heart(6);
        let can_broom = true;

        assert_eq!(
            catching_logic(&mut table, &mut player, card, can_broom),
            true
        );
        assert_eq!(table.len(), 0);
        assert_eq!(player.catched.len(), 4);
        assert_eq!(player.brooms, 1);
    }

    #[test]
    fn any_normal_catch_all_no_broom() {
        let mut table = table(&[2, 3, 4]);
        let mut player = Player::new("Test");
        let card = Card::Heart(6);
        let can_broom = false;

        assert_eq!(
            catching_logic(&mut table, &mut player, card, can_broom),
            true
        );
        assert_eq!(table.len(), 0);
        assert_eq!(player.catched.len(), 4);
        assert_eq!(player.brooms, 0);
    }

    #[test]
    fn any_normal_catch_many_normally() {
        let mut table = table(&[2, 3, 4]);
        let mut player = Player::new("Test");
        let card = Card::Heart(5);
        let can_broom = true;

        assert_eq!(
            catching_logic(&mut table, &mut player, card, can_broom),
            true
        );
        assert_eq!(table.len(), 1);
        assert_eq!(player.catched.len(), 3);
        assert_eq!(player.brooms, 0);
    }

    #[test]
    fn any_normal_catch_many_with_15() {
        let mut table = table(&[2, 3, 4]);
        let mut player = Player::new("Test");
        let card = Card::Heart(8);
        let can_broom = true;

        assert_eq!(
            catching_logic(&mut table, &mut player, card, can_broom),
            true
        );
        assert_eq!(table.len(), 1);
        assert_eq!(player.catched.len(), 3);
        assert_eq!(player.brooms, 0);
    }

    #[test]
    fn any_normal_catch_single_normally() {
        let mut table = table(&[2, 3, 4]);
        let mut player = Player::new("Test");
        let card = Card::Heart(3);
        let can_broom = true;

        assert_eq!(
            catching_logic(&mut table, &mut player, card, can_broom),
            true
        );
        assert_eq!(table.len(), 2);
        assert_eq!(player.catched.len(), 2);
        assert_eq!(player.brooms, 0);
    }

    #[test]
    fn any_normal_catch_single_with_15() {
        let mut table = table(&[2, 6, 5]);
        let mut player = Player::new("Test");
        let card = Card::Heart(9);
        let can_broom = true;

        assert_eq!(
            catching_logic(&mut table, &mut player, card, can_broom),
            true
        );
        assert_eq!(table.len(), 2);
        assert_eq!(player.catched.len(), 2);
        assert_eq!(player.brooms, 0);
    }

    #[test]
    fn multiple_catches_takes_biggest() {
        let mut table = table(&[2, 2, 10, 4]);
        let mut player = Player::new("Test");
        let card = Card::Heart(4);
        let can_broom = true;

        assert_eq!(
            catching_logic(&mut table, &mut player, card, can_broom),
            true
        );
        println!("Table: {:?}", table);
        println!("Player: {:?}", player);
        assert_eq!(table.len(), 2);
        assert_eq!(player.catched.len(), 3);
        assert_eq!(player.brooms, 0);
    }

    #[test]
    fn nothing_to_catch() {
        let mut table = table(&[5, 4, 7]);
        let mut player = Player::new("Test");
        let card = Card::Heart(2);
        let can_broom = true;

        assert_eq!(
            catching_logic(&mut table, &mut player, card, can_broom),
            false
        );
        assert_eq!(table.len(), 4);
        assert_eq!(player.catched.len(), 0);
        assert_eq!(player.brooms, 0);
    }

    #[test]
    fn ace_on_empty_table() {
        let mut table = Vec::new();
        let mut player = Player::new("Test");
        let card = Card::Heart(1);
        let can_broom = true;

        assert_eq!(
            catching_logic(&mut table, &mut player, card, can_broom),
            false
        );
        assert_eq!(table.len(), 1);
        assert_eq!(player.catched.len(), 0);
        assert_eq!(player.brooms, 0);
    }
}
