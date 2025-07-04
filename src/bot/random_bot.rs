use crate::{
    bot::Bot,
    map::{Command, Map},
};

pub struct RandomBot {}

impl Bot for RandomBot {
    fn name(&self) -> &str {
        "RandomBot"
    }

    fn get_move(&mut self, _map: &Map, _player_index: usize) -> Command {
        // Randomly choose a command for the bot
        use rand::Rng;
        let mut rng = rand::rng();
        let commands = vec![
            Command::Up,
            Command::Down,
            Command::Left,
            Command::Right,
            Command::Wait,
            Command::PlaceBomb,
        ];
        commands[rng.random_range(0..commands.len())].clone()
    }
}
