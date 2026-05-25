use anyhow::bail;
use clap::Parser;

use crate::error::{INVALID_COMMIT_SHA, Result};

#[derive(Debug, Parser)]
#[command(
    name = "gitfetchzip",
    version,
    about = "Download and extract a GitHub repository snapshot at a commit"
)]
pub struct Args {
    /// GitHub repository URL, for example https://github.com/owner/repo
    pub repo_url: String,

    /// Commit SHA or zero-based relative index from the latest commit
    pub commit_or_index: String,

    /// Directory to receive the extracted repository contents
    pub target_dir: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommitSelector {
    Index(usize),
    Sha(String),
}

impl Args {
    pub fn commit_selector(&self) -> Result<CommitSelector> {
        parse_commit_selector(&self.commit_or_index)
    }
}

fn parse_commit_selector(input: &str) -> Result<CommitSelector> {
    if input.chars().all(|ch| ch.is_ascii_digit()) {
        return Ok(CommitSelector::Index(input.parse()?));
    }

    if is_commit_sha(input) {
        return Ok(CommitSelector::Sha(input.to_ascii_lowercase()));
    }

    bail!(INVALID_COMMIT_SHA)
}

fn is_commit_sha(input: &str) -> bool {
    (7..=40).contains(&input.len()) && input.chars().all(|ch| ch.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_index() {
        assert_eq!(
            parse_commit_selector("3").unwrap(),
            CommitSelector::Index(3)
        );
    }

    #[test]
    fn parses_sha() {
        assert_eq!(
            parse_commit_selector("4B592F250F784E259A9A41DC18BB4FCBC2074DBC").unwrap(),
            CommitSelector::Sha("4b592f250f784e259a9a41dc18bb4fcbc2074dbc".to_owned())
        );
    }

    #[test]
    fn rejects_branch_names() {
        assert!(parse_commit_selector("main").is_err());
    }
}
