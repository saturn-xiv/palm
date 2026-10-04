use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClusterStatus {
    #[serde(rename = "IsLeader")]
    pub is_leader: bool,
    #[serde(rename = "Leader")]
    pub leader: String,
    #[serde(rename = "MaxVolumeId")]
    pub max_volume_id: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assign {
    pub fid: String,
    pub url: String,
    pub public_url: String,
    pub count: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LookupVolumeById {
    pub volume_or_file_id: String,
    pub locations: Vec<VolumeLocation>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeLocation {
    pub url: String,
    pub public_url: String,
    pub data_center: String,
    pub grpc_port: u16,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFile {
    pub name: String,
    pub size: usize,
    pub e_tag: String,
    pub mime: String,
}
