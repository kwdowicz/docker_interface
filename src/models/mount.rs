use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Mount {
    pub name: Option<String>,
    pub source: Option<String>,
    pub destination: Option<String>,
    pub driver: Option<String>,
    pub mode: Option<String>,
    #[serde(rename = "RW")]
    pub rw: Option<bool>,
    pub propagation: Option<String>,
}
