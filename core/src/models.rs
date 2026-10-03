use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadMetadata {
    pub url: String,
    pub filename: Option<String>,
    pub size: Option<u64>,
    pub mime_type: Option<String>,
    pub supports_resume: bool,
}
