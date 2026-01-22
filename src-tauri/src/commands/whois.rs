//! Whois lookup commands
//!
//! Provides standalone IP lookup functionality.

use crate::core::whois::WhoisClient;
use crate::models::WhoisResult;

/// Query IP information from external API
#[tauri::command]
pub async fn query_whois(ip: String) -> Result<WhoisResult, String> {
    let mut client = WhoisClient::new();
    let result = client.query(&ip).await;
    Ok(result)
}

/// Query multiple IPs at once
#[tauri::command]
pub async fn query_whois_batch(ips: Vec<String>) -> Result<Vec<WhoisResult>, String> {
    let mut client = WhoisClient::new();
    let results = client.query_batch(ips).await;
    Ok(results)
}
