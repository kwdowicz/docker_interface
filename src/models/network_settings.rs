use serde::{Serialize, Deserialize};
use crate::models::networks::Networks;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkSettings {
    pub networks: Option<Networks>,
}