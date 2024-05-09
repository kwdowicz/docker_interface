use serde::{Serialize, Deserialize};

pub type Containers = Vec<Container>;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Container {
    pub id: Option<String>,
    pub names: Option<Vec<String>>,
    pub image: Option<String>,
    #[serde(rename = "ImageID")]
    pub image_id: Option<String>,
    pub command: Option<String>,
    pub created: Option<i64>,
    pub state: Option<String>,
    pub status: Option<String>,
    pub ports: Option<Vec<Port>>,
    pub labels: Option<Labels>,
    pub size_rw: Option<i64>,
    pub size_root_fs: Option<i64>,
    pub host_config: Option<HostConfig>,
    pub network_settings: Option<NetworkSettings>,
    pub mounts: Option<Vec<Mount>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerBasic {
    pub id: String,
    pub command: String,
}

impl Container {
    pub fn basic_list(&self) -> ContainerBasic {
        // Create longer-lived variables for default values
        let default_command = "NO_COMMAND".to_string();
        let default_id = "NO_ID".to_string();
        let mut short_id = String::new();

        // Use these variables in `unwrap_or`
        let command = self.command.as_ref().unwrap_or(&default_command);
        let _id = self.id.as_ref().map_or(&default_id, |i| {
            short_id = i.clone();
            short_id.truncate(8);
            &short_id
        });

        ContainerBasic { id: short_id.to_string(), command: command.to_string() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostConfig {
    pub network_mode: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Labels {
    #[serde(rename = "com.example.vendor")]
    pub com_example_vendor: Option<String>,
    #[serde(rename = "com.example.license")]
    pub com_example_license: Option<String>,
    #[serde(rename = "com.example.version")]
    pub com_example_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkSettings {
    pub networks: Option<Networks>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Networks {
    pub bridge: Option<Bridge>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Port {
    pub private_port: Option<i64>,
    pub public_port: Option<i64>,
    #[serde(rename = "Type")]
    pub port_type: Option<String>,
}
