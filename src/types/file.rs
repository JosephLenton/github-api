#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
pub struct File {
    pub sha: String,
    pub filename: String,
    pub status: String,
    pub additions: u32,
    pub deletions: u32,
    pub changes: u32,
    pub blob_url: String,
    pub raw_url: String,
    pub contents_url: String,
    pub patch: String,
}
