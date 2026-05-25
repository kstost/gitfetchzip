use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{Context, bail};
use zip::ZipArchive;

use crate::error::Result;

pub fn extract_archive(archive_path: &Path, destination: &Path) -> Result<PathBuf> {
    let archive_file = File::open(archive_path)
        .with_context(|| format!("Failed to open archive: {}", archive_path.display()))?;
    let mut archive =
        ZipArchive::new(archive_file).context("Failed to read ZIP archive metadata.")?;

    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .with_context(|| format!("Failed to read ZIP entry #{index}."))?;
        let enclosed_name = match entry.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        let output_path = destination.join(enclosed_name);

        if entry.is_dir() {
            fs::create_dir_all(&output_path).with_context(|| {
                format!("Failed to create directory: {}", output_path.display())
            })?;
            continue;
        }

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        let mut output_file = File::create(&output_path)
            .with_context(|| format!("Failed to create file: {}", output_path.display()))?;
        io::copy(&mut entry, &mut output_file)
            .with_context(|| format!("Failed to extract file: {}", output_path.display()))?;

        #[cfg(unix)]
        if let Some(mode) = entry.unix_mode() {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&output_path, fs::Permissions::from_mode(mode))
                .with_context(|| format!("Failed to set permissions: {}", output_path.display()))?;
        }
    }

    find_archive_root(destination)
}

fn find_archive_root(destination: &Path) -> Result<PathBuf> {
    let entries = fs::read_dir(destination)
        .with_context(|| {
            format!(
                "Failed to inspect extracted directory: {}",
                destination.display()
            )
        })?
        .collect::<std::result::Result<Vec<_>, _>>()
        .context("Failed to inspect extracted archive contents.")?;

    if entries.len() != 1 {
        bail!("Archive did not contain the expected top-level directory.");
    }

    let root = entries[0].path();
    if !root.is_dir() {
        bail!("Archive did not contain the expected top-level directory.");
    }

    Ok(root)
}
