use crate::map::structs::map_config::MapConfig;
use crate::map::validators::map_validator::traits::map_validator::MapValidator;

pub struct MapValidatorNode {
    next: Option<Box<dyn MapValidator>>,
    validate_fn: fn(&MapConfig) -> Result<(), String>,
}

impl MapValidatorNode {
    pub fn new(validate_fn: fn(&MapConfig) -> Result<(), String>) -> Self {
        MapValidatorNode { next: None, validate_fn }
    }
}

impl MapValidator for MapValidatorNode {
    fn set_next(mut self: Box<Self>, next: Box<dyn MapValidator>) -> Box<dyn MapValidator> {
        self.next = Some(next);
        self
    }

    fn validate(&self, config: &MapConfig) -> Result<(), String> {
        (self.validate_fn)(config)?;
        if let Some(ref next_validator) = self.next {
            next_validator.validate(config)
        } else {
            Ok(())
        }
    }
}
