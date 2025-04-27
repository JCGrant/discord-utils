use std::fs;

use reqwest::{
    multipart::{Form, Part},
    Response,
};

pub async fn upload(images_folder: String, art_webhook_url: String, animation_webhook_url: String) {
    // Collect files
    let files: Vec<(String, String)> = fs::read_dir(&images_folder)
        .unwrap()
        .filter_map(|f| {
            let Ok(dir_entry) = f else {
                return None;
            };
            let path = dir_entry.path();
            // Check path is a file
            if !path.is_file() {
                return None;
            }
            // Check extension is valid
            let extension = path.extension()?;
            // convert to String
            let path_str = path.to_str()?;
            let extension_str = extension.to_str()?;
            Some((path_str.to_string(), extension_str.to_string()))
        })
        .collect();

    // Display extensions
    let mut unique_extensions = std::collections::HashSet::new();
    for (_, extension) in &files {
        unique_extensions.insert(extension.clone());
    }
    println!("Unique extensions: {:?}", unique_extensions);

    // Upload files
    for (index, (file_path, extension)) in files.iter().enumerate() {
        // Decide which webhook to use based on the file extension
        let webhook_url = match extension.to_lowercase().as_str() {
            "png" | "jpg" | "jpeg" => &art_webhook_url,
            "mp4" | "gif" => &animation_webhook_url,
            "webp" => {
                let is_animated = {
                    let file = fs::read(file_path).unwrap();
                    let webp =
                        image::codecs::webp::WebPDecoder::new(std::io::Cursor::new(file)).unwrap();
                    webp.has_animation()
                };
                if is_animated {
                    &animation_webhook_url
                } else {
                    &art_webhook_url
                }
            }
            _ => {
                println!("Unsupported file type: {:?}", extension);
                continue;
            }
        };

        // Attempt to upload the file a few times
        let num_attempts = 5;
        let mut uploaded = false;
        let mut attempt = 1;
        while attempt <= num_attempts && !uploaded {
            let response = send_file(file_path, webhook_url).await;
            uploaded = response.status().is_success();
            if uploaded {
                println!(
                    "Uploaded ({} / {}): {:?}",
                    index + 1,
                    files.len(),
                    file_path
                );
            } else {
                println!(
                    "Failed to upload ({} / {}) - attempt {} / {}: {:?}",
                    index + 1,
                    files.len(),
                    attempt,
                    num_attempts,
                    file_path
                );
                println!("  Response: {:?}", response.text().await.unwrap());
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(attempt)).await;
            attempt += 1;
        }
    }
}

async fn send_file(filepath: &str, webhook_url: &str) -> Response {
    let file = fs::read(filepath).unwrap();
    let form = Form::new().part("file", Part::bytes(file).file_name(filepath.to_string()));
    reqwest::Client::new()
        .post(webhook_url)
        .header("Expect", "application/json")
        .multipart(form)
        .send()
        .await
        .unwrap()
}
