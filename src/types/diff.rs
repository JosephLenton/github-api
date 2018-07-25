use types;

#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
pub struct Diff {
    pub url: String,
    pub html_url: String,
    pub permalink_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub base_commit: types::Commit,
    pub merge_base_commit: types::Commit,

    pub status: String,
    pub ahead_by: u32,
    pub behind_by: u32,
    pub total_commits: u32,

    pub commits: Vec<types::Commit>,
    pub files: Vec<types::File>,
}
