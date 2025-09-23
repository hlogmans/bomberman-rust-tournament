use crate::map::structs::map_config::MapConfig;
use crate::map::validators::map_validator::traits::map_validator::MapValidator;
use crate::map::validators::map_validator::validator::map_size_validator::MapSizeValidator;
use crate::map::validators::map_validator::validator::player_count_validator::PlayerCountValidator;

pub struct MapValidatorChainFactory;

impl MapValidatorChainFactory {
    pub fn create() -> Box<dyn MapValidator> {
        // Start with the first validator
        let chain: Box<dyn MapValidator> = Box::new(PlayerCountValidator::new())
            .set_next(Box::new(MapSizeValidator::new())); // add more validators as needed

        chain
    }

    pub fn validate(config: &MapConfig) -> Result<(), String> {
        let chain = Self::create();
        chain.validate(config)
    }
}
