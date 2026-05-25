use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, bail};

use crate::error::Result;

pub fn expand_path(input: &str) -> Result<PathBuf> {
    let expanded =
        shellexpand::full(input).with_context(|| format!("Failed to expand path: {input}"))?;
    Ok(PathBuf::from(expanded.into_owned()))
}

pub fn prepare_target_dir(target: &Path) -> Result<()> {
    if target.exists() {
        if !target.is_dir() {
            bail!("Target path exists and is not a directory.");
        }

        if fs::read_dir(target)
            .with_context(|| format!("Failed to inspect target directory: {}", target.display()))?
            .next()
            .transpose()
            .context("Failed to inspect target directory contents.")?
            .is_some()
        {
            bail!("Target directory already exists and is not empty.");
        }

        return Ok(());
    }

    fs::create_dir_all(target)
        .with_context(|| format!("Failed to create target directory: {}", target.display()))
}

pub fn copy_dir_contents(source: &Path, target: &Path) -> Result<()> {
    for entry in fs::read_dir(source)
        .with_context(|| format!("Failed to read extracted directory: {}", source.display()))?
    {
        let entry = entry.context("Failed to read extracted directory entry.")?;
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        copy_path(&source_path, &target_path)?;
    }

    Ok(())
}

fn copy_path(source: &Path, target: &Path) -> Result<()> {
    let metadata = fs::symlink_metadata(source)
        .with_context(|| format!("Failed to inspect path: {}", source.display()))?;

    if metadata.is_dir() {
        fs::create_dir_all(target)
            .with_context(|| format!("Failed to create directory: {}", target.display()))?;
        copy_dir_contents(source, target)?;
    } else if metadata.is_file() {
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }
        fs::copy(source, target).with_context(|| {
            format!(
                "Failed to copy file from {} to {}",
                source.display(),
                target.display()
            )
        })?;
    }

    Ok(())
}
