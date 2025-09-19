use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct PluginsResult {
    pub plugins: Vec<PluginEntry>,
    pub dir: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PluginEntry {
    pub name: String,
    #[serde(rename = "sizeKB")]
    pub size_kb: u64,
    pub modified: u64,
    pub display_name: Option<String>,
    pub version: Option<String>,
    pub environment: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub id: Option<String>,
    pub description: Option<String>,
    pub dependencies: Option<HashMap<String, String>>,
    pub image: Option<String>,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    pub internal: Option<bool>,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMetadata {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub environment: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default, alias = "imageUrl")]
    pub image_url: Option<String>,
    #[serde(default)]
    pub internal: Option<bool>,
    #[serde(default)]
    pub parent: Option<String>,
}

#[derive(Serialize)]
pub struct InstallFromUrlResult {
    pub message: String,
    pub size: u64,
    pub sha256: String,
    pub name: Option<String>,
    pub version: Option<String>,
    pub environment: Option<String>,
}

#[derive(Serialize)]
pub struct ValidationMetadata {
    pub valid: bool,
    pub name: Option<String>,
    pub version: Option<String>,
    pub environment: Option<String>,
    pub size: u64,
    pub sha256: Option<String>,
    pub message: String,
}

#[derive(Serialize)]
pub struct WorkshopScanResult {
    pub found: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryInfo {
    pub total_mb: u64,
    pub available_mb: u64,
}

// Versions management
#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct VersionEntry {
    pub id: String, // stable identifier (often equals version)
    pub version: Option<String>,
    pub display_name: Option<String>,
    pub dir: String, // absolute path on disk
    pub has_jre: bool,
    pub modified: u64,
    pub size_kb: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionsResult {
    pub versions: Vec<VersionEntry>,
    pub root: String,
    pub selected_id: Option<String>,
}

// Available Avrix versions from GitHub releases
#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AvailableVersion {
    pub tag: String,
    pub version: String,
    pub core_url: String,
    pub jre_url: Option<String>,
    pub published_at: Option<String>,
}
