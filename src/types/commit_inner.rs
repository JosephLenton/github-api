use types;

#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
pub struct CommitInner {
    pub url: String,
    pub author: types::UserCommitDetail,
    pub committer: types::UserCommitDetail,
    pub message: String,
    pub tree: types::UrlAndSha,
    pub comment_count: u32,
    pub verification: types::Verification,
}
