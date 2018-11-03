#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
pub struct File {
    /// sha from GitHub may be null when file mode changed without contents
    /// changing.
    pub sha: Option<String>,
    pub filename: String,
    pub status: String,
    pub additions: u32,
    pub deletions: u32,
    pub changes: u32,
    /// Missing for submodule changes.
    pub blob_url: Option<String>,
    /// Missing for submodule changes.
    pub raw_url: Option<String>,
    pub contents_url: String,
    /// Patch is typically None for binary files.
    pub patch: Option<String>,
}
