use types;

#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
pub struct Commit {
    pub url: String,
    pub sha: String,
    pub node_id: String,
    pub html_url: String,
    pub comments_url: String,
    pub commit: types::CommitInner,
    pub author: types::User,
    pub committer: types::User,
    pub parents: Vec<types::UrlAndSha>,
}
