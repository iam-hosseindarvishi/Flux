use reqwest::Client;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

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
    pub async fn start_download(
        &self,
        url: &str,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let final_path = Path::new(output_path);

        // Prevent silent overwrite of the final file
        if final_path.exists() {
            return Err(format!("File already exists: {}", output_path).into());
        }

        let mut response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(format!(
                "Network error: Download failed with status: {}",
                response.status()
            )
            .into());
        }

        // Write to a temporary .part file
        let part_filename = format!("{}.{}.part", output_path, Uuid::new_v4());
        let part_path = Path::new(&part_filename);

        // Fail if the .part file somehow exists to avoid overwriting
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&part_path)?;

        while let Some(chunk) = response.chunk().await.map_err(|e| {
            // Attempt to clean up part file if network fails during streaming
            let _ = std::fs::remove_file(&part_path);
            format!("Network error during download: {}", e)
        })? {
            if let Err(e) = file.write_all(&chunk) {
                let _ = std::fs::remove_file(&part_path);
                return Err(format!("File system error writing partial chunk: {}", e).into());
            }
        }

        // Ensure all bytes are written
        if let Err(e) = file.sync_all() {
            let _ = std::fs::remove_file(&part_path);
            return Err(format!("File system error syncing data: {}", e).into());
        }

        // Finalize atomically
        if let Err(e) = std::fs::rename(&part_path, final_path) {
            return Err(format!("Failed to finalize download (rename failed): {}", e).into());
        }

        Ok(())
    }
}
