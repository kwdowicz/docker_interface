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

impl Container {
    pub fn id_short(&self) -> String {
        let mut short_id = self.id.clone().unwrap_or("NO_ID".to_string());
        short_id.truncate(12);
        short_id
    }

    pub fn image(&self) -> String {
        let img = self.image.clone().unwrap_or("NO_IMAGE".to_string());
        img
    }

    pub fn command(&self) -> String {
        let cmd = self.command.clone().unwrap_or("NO_COMMAND".to_string());
        cmd
    }

    pub fn first_name(&self) -> String {
        self.names
            .as_ref()
            .and_then(|names| names.get(0))
            .map_or("NO_NAME".to_string(), |first_name| first_name.to_owned())
    }
}
