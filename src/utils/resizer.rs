use axum::extract::Json;
use reqwest::Client;
use serde::Deserialize;
use std::{fs, path::Path};
use uuid::Uuid;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::{cmp::min, fs::File, io::Write};

#[derive(Deserialize)]
pub struct ResizeImage {
    url: String,
    width: i32,
    height: i32,
}

pub async fn resize_image(Json(payload): Json<ResizeImage>) {
    println!("{} {} {}", payload.url, payload.width, payload.height);

    let client: Client = Client::new();
    let id = Uuid::new_v4();

    let original_filepath = ["./temp/", &id.to_string(), "_original.jpg"].join("");
    let downloaded = download_file(&client, &payload.url, &original_filepath)
        .await
        .unwrap();

    println!("Downloaded: {}", &downloaded);

    println!("Original filename: {}", &original_filepath);
    // Read the downloaded image
    let image = image::open(&original_filepath).unwrap();

    // Resize the image
    let resized_image = image.resize_exact(
        payload.width as u32,
        payload.height as u32,
        image::imageops::FilterType::Lanczos3,
    );

    // Save the resized image
    let resized_filepath = ["./temp/", &id.to_string(), "_resized.jpg"].join("");
    let resized_image_path = Path::new(&resized_filepath);

    if let Err(e) = resized_image.save(resized_image_path) {
        eprintln!("Failed to resize image: {}", e);
    }

    // Continue with your code using the resized image

    // Delete the original image
    if let Err(e) = fs::remove_file(&original_filepath) {
        eprintln!("Failed to remove original file: {}", e);
    }

    // Delete the resized image
    if let Err(e) = fs::remove_file(&resized_filepath) {
        eprintln!("Failed to remove resized file: {}", e);
    }
}


pub async fn download_file(client: &Client, url: &str, path: &str) -> Result<String, String> {
    // Reqwest setup
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = match res.content_length() {
        Some(length) => length,
        None => 100,
    };

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(&format!("Downloading {}", url));

    // download chunks
    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write_all(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(&format!("Downloaded {} to {}", url, path));
    return Ok(path.to_string());
}