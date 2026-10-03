use reqwest::Client;
use std::fs::File;
use std::io::Write;
use crate::models::DownloadMetadata;

pub struct DownloadEngine {
    client: Client,
}

impl DownloadEngine {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Starts a single connection download to a local file
    pub async fn start_download(&self, url: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut response = self.client.get(url).send().await?;
        
        if !response.status().is_success() {
            return Err(format!("Download failed with status: {}", response.status()).into());
        }

        let mut file = File::create(output_path)?;
        
        while let Some(chunk) = response.chunk().await? {
            file.write_all(&chunk)?;
        }

        Ok(())
    }
}
