
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
