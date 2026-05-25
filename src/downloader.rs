use std::path::Path;

use anyhow::{Context, bail};
use futures_util::StreamExt;
use reqwest::{Client, StatusCode};
use tokio::io::AsyncWriteExt;

use crate::error::{COMMIT_NOT_FOUND, Result};

pub async fn download_archive(client: &Client, url: &str, destination: &Path) -> Result<()> {
    let response = client
        .get(url)
        .send()
        .await
        .context("Download request failed.")?;

    match response.status() {
        StatusCode::OK => {}
        StatusCode::NOT_FOUND => bail!(COMMIT_NOT_FOUND),
        status => bail!("Download failed: HTTP {status}."),
    }

    let mut file = tokio::fs::File::create(destination)
        .await
        .with_context(|| format!("Failed to create archive file: {}", destination.display()))?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.context("Failed while downloading archive.")?;
        file.write_all(&chunk)
            .await
            .context("Failed to write archive file.")?;
    }

    file.flush()
        .await
        .context("Failed to flush archive file.")?;

    Ok(())
}
