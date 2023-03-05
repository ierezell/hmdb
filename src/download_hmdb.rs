use futures_util::StreamExt;
use indicatif;
use reqwest;
use std::cmp::min;
use std::fs;
use std::io::Write;
use std::path::Path;
use tempfile;
use zip;
pub async fn download_hmdb_files(path: &Path) {
    let metabolites_url: &str = "https://hmdb.ca/system/downloads/current/hmdb_metabolites.zip";

    let client = reqwest::Client::new();

    let metabolite_data = client
        .get(metabolites_url)
        .send()
        .await
        .expect("Failed to download metabolite data");

    let total_payload_size = metabolite_data
        .content_length()
        .expect("Failed to get content length");

    let progress_bar = indicatif::ProgressBar::new(total_payload_size);
    let bar_style = indicatif::ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .expect("Cannot set template bar style")
        .progress_chars("#>-");

    progress_bar.set_style(bar_style);
    progress_bar.set_message(format!("Downloading {}", metabolites_url));

    let mut temporary_file = tempfile::tempfile().expect("Failed to create temporary file");
    // File::create(path).expect(&format!("Failed to create file '{}'", path.display()));

    let mut downloaded_bytes = 0;
    let mut byte_stream = metabolite_data.bytes_stream();

    while let Some(item) = byte_stream.next().await {
        let chunk = item.expect(&format!("Error while downloading file"));
        temporary_file
            .write_all(&chunk)
            .expect(&format!("Error while writing to file"));
        let new_downloaded_bytes = min(downloaded_bytes + (chunk.len() as u64), total_payload_size);
        downloaded_bytes = new_downloaded_bytes;
        progress_bar.set_position(new_downloaded_bytes);
    }

    let temporary_dir = tempfile::tempdir().expect("Failed to create temporary dir");

    zip::ZipArchive::new(temporary_file)
        .expect("Failed to create zip archive")
        .extract(&temporary_dir)
        .expect("Failed to extract zip archive");

    fs::rename(temporary_dir.path().join("hmdb_metabolites.xml"), path)
        .expect("Failed to rename file");
}
