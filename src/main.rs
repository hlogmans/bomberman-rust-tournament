mod bot;
mod coord;
mod game;
mod map;
mod shrink;

fn main() {
    let bot1 = Box::new(bot::random_bot::RandomBot {});
    let bot2 = Box::new(bot::random_bot::RandomBot {});

    let mut game = game::Game::build(11, 11, vec![bot1, bot2]);

    let mut turn = 0;

    // loop until a winner is set
    while game.winner.is_none() {
        turn += 1;
        // Run a round of the game, where each bot gets to make a move

        game.run_round();
        // now we could render the game state, but showing just the turn number for simplicity
        println!("Turn: {}", turn);
    }

    print!("Game Over! Winner: {:?}", game.winner_name());
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
