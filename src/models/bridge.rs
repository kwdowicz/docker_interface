use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Bridge {
    #[serde(rename = "NetworkID")]
    pub network_id: Option<String>,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: Option<String>,
    pub gateway: Option<String>,
    #[serde(rename = "IPAddress")]
    pub ip_address: Option<String>,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: Option<i64>,
    pub i_pv6_gateway: Option<String>,
    pub global_i_pv6_address: Option<String>,
    pub global_i_pv6_prefix_len: Option<i64>,
    pub mac_address: Option<String>,
}