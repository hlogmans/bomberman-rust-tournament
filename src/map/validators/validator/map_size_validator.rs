
use crate::map::structs::map_config::MapConfig;
use crate::map::validators::traits::map_validator::MapValidator;

pub struct MapSizeValidator {
    next: Option<Box<dyn MapValidator>>,
}

impl MapSizeValidator {
    pub fn new() -> Self {
        Self { next: None }
    }
}

impl MapValidator for MapSizeValidator {
    fn set_next(mut self: Box<Self>, next: Box<dyn MapValidator>) -> Box<dyn MapValidator> {
        self.next = Some(next);
        self
    }

    fn validate(&self, config: &MapConfig) -> Result<(), String> {
        let w = config.width;
        let h = config.height;
        if w < 5 || h < 5 || w > 20 || h > 20 || w % 2 == 0 || h % 2 == 0 {
            return Err(format!("Invalid map size: {}x{}. Must be odd and 5-20", w, h));
        }
        if let Some(ref next_validator) = self.next {
            next_validator.validate(config)
        } else {
            Ok(())
        }
    }
}
