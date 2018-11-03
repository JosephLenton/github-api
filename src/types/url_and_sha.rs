#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
pub struct UrlAndSha {
    pub sha: String,
    pub url: String,
    pub html_url: Option<String>,
}
