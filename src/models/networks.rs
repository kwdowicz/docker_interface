use crate::models::bridge::Bridge;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Networks {
    pub bridge: Option<Bridge>,
}
