use crate::models::DownloadMetadata;
use reqwest::{redirect::Policy, Client, Method};
use std::error::Error;
use std::path::{Component, Path};
use url::Url;

pub struct NetworkEngine {
    client: Client,
}

impl Default for NetworkEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkEngine {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .redirect(Policy::limited(10))
                .build()
                .unwrap_or_default(),
        }
    }
    /// Fetches metadata for a given URL, falling back to GET if HEAD fails
    pub async fn fetch_metadata(&self, url: &str) -> Result<DownloadMetadata, Box<dyn Error>> {
        let parsed_url = Url::parse(url)?;
        if parsed_url.scheme() != "http" && parsed_url.scheme() != "https" {
            return Err("Only HTTP and HTTPS protocols are supported".into());
        }

        // Try HEAD first
        let mut req = self.client.request(Method::HEAD, url);
        let mut res = req.send().await?;

        // Fallback to GET if HEAD is not allowed (405) or unreliable
        if !res.status().is_success() {
            req = self.client.request(Method::GET, url);
            res = req.send().await?;
        }

        if !res.status().is_success() {
            return Err(format!("Failed to fetch URL metadata: status {}", res.status()).into());
        }

        let size = res
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());

        let mime_type = res
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_string());

        let supports_resume = res
            .headers()
            .get(reqwest::header::ACCEPT_RANGES)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|v| v.contains("bytes"));

        let mut filename = res
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

        if filename.is_none() {
            if let Some(mut segments) = parsed_url.path_segments() {
                if let Some(last) = segments.next_back() {
                    if !last.is_empty() {
                        filename = Some(last.to_string());
                    }
                }
            }
        }

        let safe_filename = filename.map(|f| Self::sanitize_filename(&f));

        Ok(DownloadMetadata {
            url: url.to_string(),
            filename: safe_filename,
            size,
            mime_type,
            supports_resume,
        })
    }

    fn sanitize_filename(name: &str) -> String {
        let path = Path::new(name);
        let mut safe_name = String::new();
        for component in path.components() {
            if let Component::Normal(os_str) = component {
                let s = os_str.to_string_lossy().to_string();
                if !safe_name.is_empty() {
                    safe_name.push('_');
                }
                safe_name.push_str(&s);
            }
        }
        if safe_name.is_empty() {
            "downloaded_file".to_string()
        } else {
            safe_name.replace(
                |c: char| !c.is_ascii_alphanumeric() && c != '.' && c != '-' && c != '_',
                "_",
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_sanitize_filename() {
        assert_eq!(NetworkEngine::sanitize_filename("valid.txt"), "valid.txt");
        assert_eq!(
            NetworkEngine::sanitize_filename("../hidden/test.txt"),
            "hidden_test.txt"
        );
        assert_eq!(NetworkEngine::sanitize_filename("a/b/c.txt"), "a_b_c.txt");
        assert_eq!(
            NetworkEngine::sanitize_filename("invalid|name?.txt"),
            "invalid_name_.txt"
        );
    }

    #[tokio::test]
    async fn test_fetch_metadata_head_success() {
        let mut server = Server::new_async().await;
        let mock = server.mock("HEAD", "/")
            .with_status(200)
            .with_header("Content-Length", "1024")
            .with_header("Content-Type", "application/pdf")
            .with_header("Accept-Ranges", "bytes")
            .with_header("Content-Disposition", "attachment; filename=\"test.pdf\"")
            .create_async().await;

        let engine = NetworkEngine::new();
        let metadata = engine.fetch_metadata(&server.url()).await.unwrap();

        mock.assert_async().await;
        assert_eq!(metadata.size, Some(1024));
        assert_eq!(metadata.mime_type.as_deref(), Some("application/pdf"));
        assert_eq!(metadata.supports_resume, true);
        assert_eq!(metadata.filename.as_deref(), Some("test.pdf"));
    }

    #[tokio::test]
    async fn test_fetch_metadata_get_fallback() {
        let mut server = Server::new_async().await;
        let head_mock = server.mock("HEAD", "/")
            .with_status(405) // Method not allowed
            .create_async().await;
        
        let get_mock = server.mock("GET", "/")
            .with_status(200)
            .with_body("hello")
            .create_async().await;

        let engine = NetworkEngine::new();
        let metadata = engine.fetch_metadata(&server.url()).await.unwrap();

        head_mock.assert_async().await;
        get_mock.assert_async().await;
        assert_eq!(metadata.size, Some(5));
    }

    #[tokio::test]
    async fn test_unsupported_scheme() {
        let engine = NetworkEngine::new();
        let result = engine.fetch_metadata("file:///etc/passwd").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Only HTTP and HTTPS protocols are supported");
    }

    #[tokio::test]
    async fn test_redirect_limit() {
        let mut server = Server::new_async().await;
        let url = server.url();
        let mock = server.mock("HEAD", "/")
            .with_status(302)
            .with_header("Location", &url) // infinite loop redirect
            .expect_at_least(1)
            .create_async().await;

        let engine = NetworkEngine::new();
        let result = engine.fetch_metadata(&url).await;
        mock.assert_async().await;
        assert!(result.is_err());
    }
}
