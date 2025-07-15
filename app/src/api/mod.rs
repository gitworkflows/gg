use anyhow::{Result, anyhow};
use reqwest::Client;
use log::{info, error};

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        ApiClient {
            client: Client::new(),
            base_url: "https://jsonplaceholder.typicode.com/".to_string(), // Example public API
        }
    }

    pub async fn fetch_data(&self, endpoint: &str) -> Result<String> {
        let url = format!("{}{}", self.base_url, endpoint);
        info!("Fetching data from: {}", url);
        match self.client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    let text = response.text().await?;
                    info!("Successfully fetched data from {}", url);
                    Ok(text)
                } else {
                    let status = response.status();
                    let error_text = response.text().await?;
                    error!("API request failed with status {}: {}", status, error_text);
                    Err(anyhow!("API request failed: {} - {}", status, error_text))
                }
            },
            Err(e) => {
                error!("Network error fetching from {}: {:?}", url, e);
                Err(anyhow!("Network error: {:?}", e))
            }
        }
    }

    pub async fn post_data(&self, endpoint: &str, body: serde_json::Value) -> Result<String> {
        let url = format!("{}{}", self.base_url, endpoint);
        info!("Posting data to: {}", url);
        match self.client.post(&url).json(&body).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    let text = response.text().await?;
                    info!("Successfully posted data to {}", url);
                    Ok(text)
                } else {
                    let status = response.status();
                    let error_text = response.text().await?;
                    error!("API POST request failed with status {}: {}", status, error_text);
                    Err(anyhow!("API POST request failed: {} - {}", status, error_text))
                }
            },
            Err(e) => {
                error!("Network error posting to {}: {:?}", url, e);
                Err(anyhow!("Network error: {:?}", e))
            }
        }
    }
}
