use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangelogItem {
    pub version: String,
    pub date: String,
    pub changes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionInfo {
    pub version: String,
    #[serde(rename = "releaseDate")]
    pub release_date: String,
    pub changelog: Vec<ChangelogItem>,
}

#[tauri::command]
pub async fn check_update(url: String) -> Result<VersionInfo, String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let response = client.get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let version_info = response.json::<VersionInfo>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(version_info)
}
