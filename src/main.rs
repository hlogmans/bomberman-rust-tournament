use std::time::Instant;

use crate::{bot::available_bots, tournament::run_tournament};

mod bot;
mod coord;
mod game;
mod map;
mod shrink;
mod tournament;

fn main() {
    let bot_constructors = available_bots();

    let bot1 = bot_constructors.get(1).unwrap()("Bot1-Easy");
    let bot2 = bot_constructors.get(1).unwrap()("Bot-Easy2");
    let bot3 = bot_constructors.get(0).unwrap()("Bot3-Random");
    let bot4 = bot_constructors.get(0).unwrap()("Bot4-Random");

    let bots = vec![bot1, bot2, bot3, bot4];

    run_tournament(&bots);

    // // determine how long the game will last in milliseconds
    // //
    // // Start timer
    // let start_time = Instant::now();

    // let gameresult = game::Game::build(11, 11, vec![bot1, bot2]).run();

    // print!(
    //     "Game Over! Winner: {:?} in {:?} rounds",
    //     gameresult.winner, gameresult.rounds
    // );

    // let duration = start_time.elapsed();
    // println!("\nGame duration: {:.2?}", duration);

    // // Replay the game
    // let rbot1 = bot_constructors.get(1).unwrap()("Bot1");
    // let rbot2 = bot_constructors.get(1).unwrap()("Bot2");
    // let commands = gameresult.replay_data;
    // let replay_result = game::Game::build(11, 11, vec![rbot1, rbot2]).replay(&commands);

    // print!(
    //     "Replay Over! Winner: {:?} in {:?} rounds",
    //     replay_result.winner, replay_result.rounds
    // );

    // let duration = start_time.elapsed();
    // println!("\nGame duration: {:.2?}", duration);
}
