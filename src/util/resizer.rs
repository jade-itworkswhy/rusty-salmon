
use axum::extract::Json;
use reqwest::Client;
use serde::Deserialize;
use std::{fs, path::Path};
use uuid::Uuid;

#[path = "./downloader.rs"]
mod downloader;

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

    let original_filepath = ["./temp/",&id.to_string(), "_original.jpg"].join("");
    let downloaded = downloader::download_file(&client, &payload.url, &original_filepath)
        .await
        .unwrap();

    println!("Downloaded: {}", &downloaded);


    println!("Original filename: {}", &original_filepath);
    // Read the downloaded image
    let image = image::open(&original_filepath).unwrap();

    // Resize the image
    let resized_image = image.resize_exact(payload.width as u32, payload.height as u32, image::imageops::FilterType::Lanczos3);
    
    // Save the resized image
    let resized_filepath = ["./temp/",&id.to_string(), "_resized.jpg"].join("");
    let resized_image_path = Path::new(&resized_filepath);
    resized_image.save(resized_image_path);

    // Continue with your code using the resized image


// Delete the original image
if let Err(e) = fs::remove_file(&original_filepath) {
    eprintln!("Failed to remove original file: {}", e);
}

// Delete the resized image
if let Err(e) = fs::remove_file(&resized_filepath) {
    eprintln!("Failed to remove resized file: {}", e);
}

    ()
}
