//Stolen off Reddit
// https://www.reddit.com/r/rust/comments/9lrpru/download_file_with_progress_bar/e7e43wh?utm_source=share&utm_medium=web2x&context=3
use crate::error::LauncherError;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use reqwest::header;
use reqwest::Url;
use std::fs::create_dir_all;
use std::io::Read;
use std::io::Write;
use std::{fs, path::Path};

pub fn download(url: &str, location: &Path, what: &str) -> Result<(), LauncherError> {
    if location.exists() {
        return Ok(());
    }
    println!("Downloading {}", what);
    let x = location.parent().unwrap();
    if !x.exists() {
        create_dir_all(x)?;
    }

    let url = Url::parse(url).unwrap();
    let client = Client::new();

    let total_size = {
        let resp = client.head(url.as_str()).send()?;
        if resp.status().is_success() {
            resp.headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|ct_len| ct_len.to_str().ok())
                .and_then(|ct_len| ct_len.parse().ok())
                .unwrap_or(0)
        } else {
            return Err(LauncherError::from(resp.status()));
        }
    };

    let mut request = client.get(url.as_str());
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})").unwrap()
        .progress_chars("#>-"));

    if location.exists() {
        let size = location.metadata().unwrap().len() - 1;
        request = request.header(header::RANGE, format!("bytes={}-", size));
        pb.inc(size);
    }

    let mut dest = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(location)?;

    let mut source = request.send()?;
    let mut bytes = [0; 24];
    while let Ok(chunk) = source.read(&mut bytes) {
        if chunk == 0 {
            break;
        }
        dest.write_all(&bytes[..chunk])?;
        pb.inc(chunk as u64);
    }
    println!(
        "Download of '{}' has been completed.",
        location.to_str().unwrap()
    );

    Ok(())
}
