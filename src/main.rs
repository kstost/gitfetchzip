mod cli;
mod downloader;
mod error;
mod extractor;
mod fs;
mod github;

use clap::Parser;
use tempfile::tempdir;

use crate::cli::{Args, CommitSelector};
use crate::error::Result;

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("Error:\n{error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let args = Args::parse();
    let repo = github::parse_repository_url(&args.repo_url)?;
    let selector = args.commit_selector()?;
    let target_dir = fs::expand_path(&args.target_dir)?;
    let client = github::http_client()?;

    let commit = match selector {
        CommitSelector::Index(index) => github::commit_sha_for_index(&client, &repo, index).await?,
        CommitSelector::Sha(sha) => sha,
    };

    fs::prepare_target_dir(&target_dir)?;

    println!("[gitfetchzip]\n");
    println!("Repository : {}", repo.full_name());
    println!("Commit     : {}", github::short_sha(&commit));
    println!();
    println!("Downloading archive...");

    let workspace = tempdir()?;
    let archive_path = workspace.path().join("archive.zip");
    let archive_url = github::archive_url(&repo, &commit);
    downloader::download_archive(&client, &archive_url, &archive_path).await?;

    println!("Extracting files...");
    let extract_dir = workspace.path().join("extract");
    std::fs::create_dir_all(&extract_dir)?;
    let source_root = extractor::extract_archive(&archive_path, &extract_dir)?;
    fs::copy_dir_contents(&source_root, &target_dir)?;

    println!("Done.\n");
    println!("Output:");
    println!("{}", target_dir.display());

    Ok(())
}
