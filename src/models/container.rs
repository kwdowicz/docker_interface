use crate::models::host_config::HostConfig;
use crate::models::labels::Labels;
use crate::models::mount::Mount;
use crate::models::network_settings::NetworkSettings;
use crate::models::port::Port;
use serde::{Deserialize, Serialize};

pub type Containers = Vec<Container>;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
