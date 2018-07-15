#[allow(missing_docs)]
#[derive(Deserialize, Debug)]
pub struct Contents {
    pub encoding: String,
    pub size: u32,
    pub name: String,
    pub path: String,
    pub content: String,
    pub sha: String,
    pub url: String,
    pub git_url: String,
    pub html_url: String,
    pub download_url: String,
}
