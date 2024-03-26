use cirulla_lib::Game;

fn main() {
    let mut game = Game::new();
    let alice = game.add_player("Alice").unwrap();
    let bob = game.add_player("Bob").unwrap();
    println!("Alice: player {}\nBob: player {}", alice, bob);

    println!("Cirulla! {:?}\n", game);
    game.start_round().unwrap();

    println!("Cirulla! {:?}\n", game);
    game.player_draw(alice).unwrap();
    game.player_draw(bob).unwrap();
    
    println!("Cirulla! {:?}\n", game);

    game.end_round().unwrap();

    println!("Cirulla! {:?}\n", game);
}
