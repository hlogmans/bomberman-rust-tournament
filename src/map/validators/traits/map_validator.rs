use crate::map::structs::map_config::MapConfig;

pub trait MapValidator {
    fn set_next(self: Box<Self>, next: Box<dyn MapValidator>) -> Box<dyn MapValidator>;
    fn validate(&self, config: &MapConfig) -> Result<(), String>;
}
