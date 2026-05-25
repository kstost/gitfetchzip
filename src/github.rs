use std::time::Duration;

use anyhow::{Context, bail};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use url::Url;

use crate::error::{COMMIT_INDEX_OUT_OF_RANGE, INVALID_GITHUB_REPO_URL, Result};

const GITHUB_HOST: &str = "github.com";
const API_BASE_URL: &str = "https://api.github.com";
const USER_AGENT: &str = concat!("gitfetchzip/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Repository {
    pub owner: String,
    pub name: String,
}

impl Repository {
    pub fn full_name(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}

#[derive(Debug, Deserialize)]
struct CommitSummary {
    sha: String,
}

pub fn http_client() -> Result<Client> {
    Client::builder()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(60))
        .build()
        .context("Failed to create HTTP client.")
}

pub fn parse_repository_url(input: &str) -> Result<Repository> {
    if let Some(repo) = parse_git_ssh_url(input) {
        return Ok(repo);
    }

    let parsed = Url::parse(input).map_err(|_| anyhow::anyhow!(INVALID_GITHUB_REPO_URL))?;
    if parsed.host_str() != Some(GITHUB_HOST) {
        bail!(INVALID_GITHUB_REPO_URL);
    }

    let mut segments = parsed
        .path_segments()
        .ok_or_else(|| anyhow::anyhow!(INVALID_GITHUB_REPO_URL))?
        .filter(|segment| !segment.is_empty());

    let owner = segments
        .next()
        .ok_or_else(|| anyhow::anyhow!(INVALID_GITHUB_REPO_URL))?;
    let repo = segments
        .next()
        .ok_or_else(|| anyhow::anyhow!(INVALID_GITHUB_REPO_URL))?;

    if segments.next().is_some() || owner.is_empty() || repo.is_empty() {
        bail!(INVALID_GITHUB_REPO_URL);
    }

    Ok(Repository {
        owner: owner.to_owned(),
        name: strip_git_suffix(repo),
    })
}

pub async fn commit_sha_for_index(
    client: &Client,
    repo: &Repository,
    index: usize,
) -> Result<String> {
    let page = (index / 100) + 1;
    let page_index = index % 100;
    let url = format!(
        "{API_BASE_URL}/repos/{}/{}/commits?per_page=100&page={page}",
        repo.owner, repo.name
    );

    let response = client
        .get(url)
        .send()
        .await
        .context("GitHub API request failed.")?;

    match response.status() {
        StatusCode::OK => {}
        StatusCode::NOT_FOUND => bail!("Repository not found."),
        status => bail!("GitHub API request failed: HTTP {status}."),
    }

    let commits: Vec<CommitSummary> = response
        .json()
        .await
        .context("Failed to parse GitHub commits response.")?;

    commits
        .get(page_index)
        .map(|commit| commit.sha.clone())
        .ok_or_else(|| anyhow::anyhow!(COMMIT_INDEX_OUT_OF_RANGE))
}

pub fn archive_url(repo: &Repository, commit: &str) -> String {
    format!(
        "https://github.com/{}/{}/archive/{commit}.zip",
        repo.owner, repo.name
    )
}

pub fn short_sha(commit: &str) -> &str {
    commit.get(..7).unwrap_or(commit)
}

fn parse_git_ssh_url(input: &str) -> Option<Repository> {
    let rest = input.strip_prefix("git@github.com:")?;
    let (owner, repo) = rest.split_once('/')?;
    if owner.is_empty() || repo.is_empty() || repo.contains('/') {
        return None;
    }

    Some(Repository {
        owner: owner.to_owned(),
        name: strip_git_suffix(repo),
    })
}

fn strip_git_suffix(repo: &str) -> String {
    repo.strip_suffix(".git").unwrap_or(repo).to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_https_repository_url() {
        let repo = parse_repository_url("https://github.com/kstost/cokacdir.git").unwrap();
        assert_eq!(repo.owner, "kstost");
        assert_eq!(repo.name, "cokacdir");
    }

    #[test]
    fn parses_git_ssh_repository_url() {
        let repo = parse_repository_url("git@github.com:kstost/cokacdir.git").unwrap();
        assert_eq!(repo.full_name(), "kstost/cokacdir");
    }

    #[test]
    fn rejects_non_github_url() {
        assert!(parse_repository_url("https://example.com/kstost/cokacdir").is_err());
    }
}
