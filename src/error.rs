pub type Result<T> = anyhow::Result<T>;

pub const INVALID_GITHUB_REPO_URL: &str = "Invalid GitHub repository URL.";
pub const INVALID_COMMIT_SHA: &str = "Invalid commit SHA.";
pub const COMMIT_INDEX_OUT_OF_RANGE: &str = "Commit index out of range.";
pub const COMMIT_NOT_FOUND: &str = "Commit not found.";
