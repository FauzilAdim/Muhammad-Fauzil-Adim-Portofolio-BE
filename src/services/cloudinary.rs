use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudinaryResponse {
    pub secure_url: String,
    pub public_id: String,
}

pub struct CloudinaryService {
    cloud_name: String,
    upload_preset: String,
}

impl CloudinaryService {
    pub fn new() -> Self {
        Self {
            cloud_name: env::var("CLOUDINARY_CLOUD_NAME").expect("CLOUDINARY_CLOUD_NAME must be set"),
            upload_preset: env::var("CLOUDINARY_UPLOAD_PRESET").unwrap_or_else(|_| "portfolio_uploads".to_string()),
        }
    }

    pub async fn upload_image(&self, image_data: Vec<u8>, filename: String) -> Result<String, String> {
        let url = format!(
            "https://api.cloudinary.com/v1_1/{}/image/upload",
            self.cloud_name
        );

        let part = Part::bytes(image_data)
            .file_name(filename)
            .mime_str("image/png")
            .map_err(|e| e.to_string())?;

        let form = Form::new()
            .part("file", part)
            .text("upload_preset", self.upload_preset.clone())
            .text("folder", "portfolio");

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| format!("Failed to upload to Cloudinary: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Cloudinary upload failed: {}", error_text));
        }

        let cloudinary_response: CloudinaryResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Cloudinary response: {}", e))?;

        Ok(cloudinary_response.secure_url)
    }
}
