use crate::models::networks::Networks;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkSettings {
    pub networks: Option<Networks>,
}
