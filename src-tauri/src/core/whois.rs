//! Whois IP lookup module
//!
//! Queries external API for IP geolocation and ISP information.
//! Uses ip-api.com free API (no API key required).
//!
//! **OpSec Warning**: This module makes external network requests.

use crate::models::{Transaction, WhoisResult};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;

/// API endpoint for IP lookup
const IP_API_ENDPOINT: &str = "http://ip-api.com/json";

/// Rate limit: max 45 requests per minute for free tier
const RATE_LIMIT_DELAY_MS: u64 = 1500; // ~40 requests per minute

/// Response from ip-api.com
#[derive(Debug, Deserialize)]
struct IpApiResponse {
    status: String,
    country: Option<String>,
    #[serde(rename = "countryCode")]
    country_code: Option<String>,
    isp: Option<String>,
    org: Option<String>,
    #[allow(dead_code)]
    query: String,
}

/// Whois lookup client with caching
pub struct WhoisClient {
    client: Client,
    cache: HashMap<String, WhoisResult>,
}

impl WhoisClient {
    /// Create a new Whois client
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        Self {
            client,
            cache: HashMap::new(),
        }
    }

    /// Query a single IP address
    pub async fn query(&mut self, ip: &str) -> WhoisResult {
        // Check cache first
        if let Some(cached) = self.cache.get(ip) {
            return cached.clone();
        }

        // Skip private/local IPs
        if is_private_ip(ip) {
            let result = WhoisResult {
                ip: ip.to_string(),
                country: Some("Private".to_string()),
                isp: Some("Local Network".to_string()),
                query_success: true,
            };
            self.cache.insert(ip.to_string(), result.clone());
            return result;
        }

        // Make API request
        let url = format!("{}/{}?fields=status,country,countryCode,isp,org,query", IP_API_ENDPOINT, ip);

        let result = match self.client.get(&url).send().await {
            Ok(response) => {
                match response.json::<IpApiResponse>().await {
                    Ok(data) => {
                        if data.status == "success" {
                            WhoisResult {
                                ip: ip.to_string(),
                                country: data.country_code.or(data.country),
                                isp: data.isp.or(data.org),
                                query_success: true,
                            }
                        } else {
                            WhoisResult {
                                ip: ip.to_string(),
                                country: None,
                                isp: None,
                                query_success: false,
                            }
                        }
                    }
                    Err(_) => WhoisResult {
                        ip: ip.to_string(),
                        country: None,
                        isp: None,
                        query_success: false,
                    },
                }
            }
            Err(_) => WhoisResult {
                ip: ip.to_string(),
                country: None,
                isp: None,
                query_success: false,
            },
        };

        // Cache the result
        self.cache.insert(ip.to_string(), result.clone());
        result
    }

    /// Query multiple IPs with rate limiting
    pub async fn query_batch(&mut self, ips: Vec<String>) -> Vec<WhoisResult> {
        let mut results = Vec::new();

        for ip in ips {
            let result = self.query(&ip).await;
            results.push(result);

            // Rate limiting delay
            tokio::time::sleep(Duration::from_millis(RATE_LIMIT_DELAY_MS)).await;
        }

        results
    }

    /// Enrich transactions with Whois data
    pub async fn enrich_transactions(&mut self, transactions: &mut [Transaction]) {
        // Collect unique IPs from matched_ip field
        let mut unique_ips: Vec<String> = Vec::new();

        for tx in transactions.iter() {
            if let Some(ref matched) = tx.matched_ip {
                if matched != "N/A" && !matched.starts_with("N/A") {
                    // Extract IPs from formatted string like "-1s:1.1.1.1 | +2s:2.2.2.2"
                    for part in matched.split(" | ") {
                        let ip = if part.contains(':') {
                            part.split(':').last().unwrap_or(part)
                        } else {
                            part
                        };
                        if !unique_ips.contains(&ip.to_string()) {
                            unique_ips.push(ip.to_string());
                        }
                    }
                }
            }
        }

        // Query all unique IPs
        let results = self.query_batch(unique_ips).await;

        // Build lookup map
        let ip_info: HashMap<String, &WhoisResult> = results.iter().map(|r| (r.ip.clone(), r)).collect();

        // Enrich transactions
        for tx in transactions.iter_mut() {
            if let Some(ref matched) = tx.matched_ip {
                if matched != "N/A" && !matched.starts_with("N/A") {
                    // Get first IP for country/ISP info
                    let first_ip = if matched.contains(':') {
                        matched.split(':').last().unwrap_or(matched).split(" | ").next().unwrap_or(matched)
                    } else {
                        matched.split(" | ").next().unwrap_or(matched)
                    };

                    if let Some(info) = ip_info.get(first_ip) {
                        tx.ip_country = info.country.clone();
                        tx.ip_isp = info.isp.clone();
                    }
                }
            }
        }
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        let total = self.cache.len();
        let successful = self.cache.values().filter(|r| r.query_success).count();
        (total, successful)
    }
}

impl Default for WhoisClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if an IP address is private/local
fn is_private_ip(ip: &str) -> bool {
    // Parse IP and check ranges
    let parts: Vec<u8> = ip
        .split('.')
        .filter_map(|p| p.parse().ok())
        .collect();

    if parts.len() != 4 {
        return false;
    }

    // 10.0.0.0/8
    if parts[0] == 10 {
        return true;
    }

    // 172.16.0.0/12
    if parts[0] == 172 && (16..=31).contains(&parts[1]) {
        return true;
    }

    // 192.168.0.0/16
    if parts[0] == 192 && parts[1] == 168 {
        return true;
    }

    // 127.0.0.0/8 (loopback)
    if parts[0] == 127 {
        return true;
    }

    // 169.254.0.0/16 (link-local)
    if parts[0] == 169 && parts[1] == 254 {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_private_ip() {
        assert!(is_private_ip("10.0.0.1"));
        assert!(is_private_ip("172.16.0.1"));
        assert!(is_private_ip("172.31.255.255"));
        assert!(is_private_ip("192.168.1.1"));
        assert!(is_private_ip("127.0.0.1"));
        assert!(!is_private_ip("8.8.8.8"));
        assert!(!is_private_ip("1.1.1.1"));
    }
}
