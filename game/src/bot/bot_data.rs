use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct BotData {
    pub name: String,
    pub id: usize,
}
