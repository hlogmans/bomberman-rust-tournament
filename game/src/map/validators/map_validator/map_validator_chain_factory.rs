use crate::map::structs::map_config::MapConfig;
use crate::map::validators::map_validator::traits::map_validator::MapValidator;
use crate::map::validators::map_validator::validator::map_size_validator::MapSizeValidator;

pub struct MapValidatorChainFactory;

impl MapValidatorChainFactory {
    pub fn create() -> Box<dyn MapValidator> {
        // Start with the first validator
        let chain: Box<dyn MapValidator> = Box::new(MapSizeValidator::new());

        chain
    }

    pub fn validate(config: &MapConfig) -> Result<(), String> {
        let chain = Self::create();
        chain.validate(config)
    }
}
