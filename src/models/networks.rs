use serde::{Serialize, Deserialize};
use crate::models::bridge::Bridge;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Networks {
    pub bridge: Option<Bridge>,
}