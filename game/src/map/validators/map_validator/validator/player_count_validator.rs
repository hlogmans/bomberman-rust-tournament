
use crate::map::structs::map_config::MapConfig;
use crate::map::validators::map_validator::traits::map_validator::MapValidator;

pub struct PlayerCountValidator {
    next: Option<Box<dyn MapValidator>>,
}

impl PlayerCountValidator {
    pub fn new() -> Self {
        Self { next: None }
    }
}

impl MapValidator for PlayerCountValidator {
    fn set_next(mut self: Box<Self>, next: Box<dyn MapValidator>) -> Box<dyn MapValidator> {
        self.next = Some(next);
        self
    }

    fn validate(&self, config: &MapConfig) -> Result<(), String> {
        let n = config.player_names.len();
        if n < 2 || n > 4 {
            return Err(format!("Invalid number of players: {}. Must be 2-4", n));
        }
        if let Some(ref next_validator) = self.next {
            next_validator.validate(config)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::structs::map_config::MapConfig;
    use crate::map::validators::map_validator::traits::map_validator::MapValidator;

    #[test]
    fn test_valid_player_counts() {
        let validator = PlayerCountValidator::new();

        let mut config = MapConfig {
            player_names: vec!["Alice".to_string(), "Bob".to_string()],
            ..MapConfig::default()
        };
        assert!(validator.validate(&config).is_ok());

        config.player_names = vec![
            "Alice".to_string(),
            "Bob".to_string(),
            "Charlie".to_string(),
        ];
        assert!(validator.validate(&config).is_ok());

        config.player_names = vec![
            "Alice".to_string(),
            "Bob".to_string(),
            "Charlie".to_string(),
            "Dave".to_string(),
        ];
        assert!(validator.validate(&config).is_ok());
    }

    #[test]
    fn test_invalid_player_counts() {
        let validator = PlayerCountValidator::new();

        let mut config = MapConfig {
            player_names: vec!["Alice".to_string()],
            ..MapConfig::default()
        };
        assert!(validator.validate(&config).is_err());

        config.player_names = vec![
            "Alice".to_string(),
            "Bob".to_string(),
            "Charlie".to_string(),
            "Dave".to_string(),
            "Eve".to_string(),
        ];
        assert!(validator.validate(&config).is_err());
    }
}

