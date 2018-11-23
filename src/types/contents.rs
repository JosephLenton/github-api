#[allow(missing_docs)]
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Contents {
    File(ContentsFile),
    Dir(Vec<ContentsDirFile>),
}

#[allow(missing_docs)]
#[derive(Deserialize, Debug)]
pub struct ContentsFile {
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

#[allow(missing_docs)]
#[derive(Deserialize, Debug)]
pub struct ContentsDirFile {
    #[serde(rename = "type")]
    pub content_type: FileOrDir,
    pub size: u32,
    pub name: String,
    pub path: String,
    pub sha: String,
    pub url: String,
    pub git_url: String,
    pub html_url: String,
    pub download_url: Option<String>,
    pub _links: Links,
}

#[allow(missing_docs)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FileOrDir {
    File,
    Dir,
}

#[allow(missing_docs)]
#[derive(Deserialize, Debug)]
pub struct Links {
    #[serde(rename = "self")]
    pub this: String,
    pub git: String,
    pub html: String,
}
