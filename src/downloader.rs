//Stolen off Reddit
// https://www.reddit.com/r/rust/comments/9lrpru/download_file_with_progress_bar/e7e43wh?utm_source=share&utm_medium=web2x&context=3
use std::{fs,
          io::{self, copy, Read},
          path::Path,
};
use std::fs::create_dir_all;
use std::io::Write;

use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Client, header};
use reqwest::Url;

pub async fn download(url: &str, location: &Path, what: &str) -> Result<(), String> {
    if location.exists(){
        return Ok(());
    }
    println!("Downloading {}", what);
    let x = location.parent().unwrap();
    if !x.exists() {
        create_dir_all(x);
    }

    let url = Url::parse(url).unwrap();
    let client = Client::new();

    let total_size = {
        let resp = client.head(url.as_str()).send().await.unwrap();
        if resp.status().is_success() {
            resp.headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|ct_len| ct_len.to_str().ok())
                .and_then(|ct_len| ct_len.parse().ok())
                .unwrap_or(0)
        } else {
            return Err(String::from("Failed to download file"));
        }
    };

    let mut request = client.get(url.as_str());
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));


    if location.exists() {
        let size = location.metadata().unwrap().len() - 1;
        request = request.header(header::RANGE, format!("bytes={}-", size));
        pb.inc(size);
    }


    let mut dest = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(location).unwrap();


    let mut source = request.send().await.unwrap();
    while let Some(chunk) = source.chunk().await.unwrap() {
        dest.write_all(&chunk);
        pb.inc(chunk.len() as u64);
    }
    println!(
        "Download of '{}' has been completed.",
        location.to_str().unwrap()
    );

    Ok(())
}