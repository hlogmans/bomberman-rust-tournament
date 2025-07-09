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

    print!(
        "Game Over! Winner: {:?} in {:?} rounds",
        gameresult.winner, gameresult.rounds
    );

    let duration = start_time.elapsed();
    println!("\nGame duration: {:.2?}", duration);

    // Replay the game
    let rbot1 = bot_constructors.get(1).unwrap()("Bot1");
    let rbot2 = bot_constructors.get(1).unwrap()("Bot2");
    let commands = gameresult.replay_data;
    let replay_result = game::Game::build(11, 11, vec![rbot1, rbot2]).replay(&commands);

    print!(
        "Replay Over! Winner: {:?} in {:?} rounds",
        replay_result.winner, replay_result.rounds
    );

    let duration = start_time.elapsed();
    println!("\nGame duration: {:.2?}", duration);
}
