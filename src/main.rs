use std::time::Instant;

use crate::bot::available_bots;

mod bot;
mod coord;
mod game;
mod map;
mod shrink;

fn main() {
    let bot_constructors = available_bots();

    let bot1 = bot_constructors.get(1).unwrap()("Bot1");
    let bot2 = bot_constructors.get(1).unwrap()("Bot2");

    // determine how long the game will last in milliseconds
    //
    // Start timer
    let start_time = Instant::now();

    let gameresult = game::Game::build(11, 11, vec![bot1, bot2]).run();

    print!("Game Over! Winner: {:?}", gameresult.winner);

    let duration = start_time.elapsed();
    println!("\nGame duration: {:.2?}", duration);
}
