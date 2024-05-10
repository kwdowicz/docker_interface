use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Labels {
    #[serde(rename = "com.example.vendor")]
    pub com_example_vendor: Option<String>,
    #[serde(rename = "com.example.license")]
    pub com_example_license: Option<String>,
    #[serde(rename = "com.example.version")]
    pub com_example_version: Option<String>,
}
