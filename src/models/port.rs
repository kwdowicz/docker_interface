use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Port {
    pub private_port: Option<i64>,
    pub public_port: Option<i64>,
    #[serde(rename = "Type")]
    pub port_type: Option<String>,
}