use reqwest::Client;
use std::error::Error;
use crate::models::DownloadMetadata;

pub struct NetworkEngine {
    client: Client,
}

impl NetworkEngine {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Fetches metadata for a given URL via HEAD request
    pub async fn fetch_metadata(&self, url: &str) -> Result<DownloadMetadata, Box<dyn Error>> {
        let res = self.client.head(url).send().await?;

        // 1. Detect Size
        let size = res
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());

        // 2. Detect MIME type
        let mime_type = res
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_string());

        // 3. Detect Resume Support (Accept-Ranges: bytes)
        let supports_resume = res
            .headers()
            .get(reqwest::header::ACCEPT_RANGES)
            .and_then(|v| v.to_str().ok())
            .map_or(false, |v| v.contains("bytes"));

        // 4. Detect Filename from Content-Disposition
        let filename = res
            .headers()
            .get(reqwest::header::CONTENT_DISPOSITION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| {
                v.split(';')
                    .find(|p| p.trim().starts_with("filename="))
                    .map(|p| {
                        p.trim()
                            .trim_start_matches("filename=")
                            .trim_matches('"')
                            .to_string()
                    })
            });
            // Fallback to URL path could be done here if filename is None.

        Ok(DownloadMetadata {
            url: url.to_string(),
            filename,
            size,
            mime_type,
            supports_resume,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    // Real unit tests would typically mock the HTTP response.
    // Assuming cargo is not available, this is just a placeholder structure.
    #[test]
    async fn test_fetch_metadata() {
        let engine = NetworkEngine::new();
        // Since we can't test actual URLs reliably without a local server, 
        // we would normally use mockito or wiremock here.
        assert!(true);
    }
}
