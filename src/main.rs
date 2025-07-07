use std::time::Instant;

use crate::bot::available_bots;

mod bot;
mod coord;
mod game;
mod map;
mod shrink;

fn main() {
    let bot_constructors = available_bots();

    let bot1 = bot_constructors.get(0).unwrap()("Bot1");
    let bot2 = bot_constructors.get(1).unwrap()("Bot2");

    // determine how long the game will last in milliseconds
    //
    // Start timer
    let start_time = Instant::now();

    let mut game = game::Game::build(11, 11, vec![bot1, bot2]);

    game.display();

    // loop until a winner is set
    while game.winner.is_none() {
        game.run_round(None);
    }
    game.display();
    print!("Game Over! Winner: {:?}", game.winner_name().unwrap());

    let duration = start_time.elapsed();
    println!("\nGame duration: {:.2?}", duration);
}

// I want to create a bomberman server in Rust.
// I want to put all building blocks in separate files, but also in threads.
// So, there must be pipelines handling the game logic, rendering, and networking.

// A bomberman map consists of a 2D vector of characters, where:
// ' ' is empty space where players can move,
// 'B' is a bomb that is placed by a players, it will last for 3 turns before exploding, it cannot be moved to
// 'W' is a wall that cannot be destroyed and not moved to, and
// 'P' is a player that cannot be moved to, but if the current player is already on a bomb he can stay there.
// '.' is a place that can be destroyed by a bomb, and it will turn into ' ' after the bomb explodes.
// The outer edges of the map are walls, so they cannot be moved to.

// players are put on the map and they can move around, turn by turn. Every turn the next player can make a move.
