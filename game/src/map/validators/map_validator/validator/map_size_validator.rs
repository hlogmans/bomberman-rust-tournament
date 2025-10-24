
use crate::map::structs::map_config::MapConfig;
use crate::map::validators::map_validator::traits::map_validator::MapValidator;

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
        let w = config.size;
        let h = config.size;
        if w < 7 || h < 7 || w > 20 || h > 20 || w.is_multiple_of(2) || h.is_multiple_of(2) {
            return Err(format!("Invalid map size: {w}x{h}. Must be odd and 7-20"));
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

    #[test]
    fn test_valid_map_sizes() {
        let validator = MapSizeValidator::new();
        let config = MapConfig { size: 7, .. MapConfig::default() };
        assert!(validator.validate(&config).is_ok());

        let config = MapConfig { size: 19, .. MapConfig::default() };
        assert!(validator.validate(&config).is_ok());

        let config = MapConfig { size: 11, .. MapConfig::default() };
        assert!(validator.validate(&config).is_ok());
    }

    #[test]
    fn test_invalid_map_sizes_too_small() {
        let validator = MapSizeValidator::new();
        let config = MapConfig { size: 5, .. MapConfig::default() };
        assert!(validator.validate(&config).is_err());

        let config = MapConfig { size: 4, .. MapConfig::default() };
        assert!(validator.validate(&config).is_err());
    }

    #[test]
    fn test_invalid_map_sizes_too_large() {
        let validator = MapSizeValidator::new();
        let config = MapConfig { size: 21, .. MapConfig::default() };
        assert!(validator.validate(&config).is_err());

        let config = MapConfig { size: 22, .. MapConfig::default() };
        assert!(validator.validate(&config).is_err());
    }

    #[test]
    fn test_invalid_map_sizes_even_numbers() {
        let validator = MapSizeValidator::new();
        let config = MapConfig { size: 5, .. MapConfig::default() };

        assert!(validator.validate(&config).is_err());

        let config = MapConfig { size: 8, .. MapConfig::default() };
        assert!(validator.validate(&config).is_err());
    }
}
