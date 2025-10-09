/// Represents a single game configuration
#[derive(Debug, Clone)]
pub struct GameConfig {
    pub num_players: usize,
    pub size: usize,
}

/// Utility to generate odd numbers in a range
fn odd_numbers_in_range(start: usize, end: usize) -> Vec<usize> {
    (start..=end)
        .filter(|x| x % 2 == 1)
        .collect()
}

/// Factory for creating tournament configurations
pub struct ConfigFactory;

impl ConfigFactory {
    /// Generates all tournament configs with given player counts and map sizes
    pub fn generate_tournament_configs() -> Vec<GameConfig> {
        let player_counts = [2, 3, 4];
        let map_sizes = odd_numbers_in_range(5, 19);

        let mut configs = Vec::new();

        for size in map_sizes {
            for &players in &player_counts {
                configs.push(GameConfig {
                    num_players: players,
                    size,
                });
            }
        }

        configs
    }
}
