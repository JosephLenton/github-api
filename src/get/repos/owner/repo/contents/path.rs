use extern::base64;
use extern::burgundy;
use types;

lazy_static! {
    /// Github returns it's base64 content in a 60 character wide block.
    static ref GITHUB_BASE64_CONFIG : base64::Config = base64::Config::new(
        base64::CharacterSet::Standard,
        true,
        true,
        base64::LineWrap::Wrap(60, base64::LineEnding::LF),
    );
}

pub struct GithubGetReposOwnerRepoContentsPath {
    crate path: burgundy::Path,
}

impl GithubGetReposOwnerRepoContentsPath {
    pub fn run(self) -> burgundy::Result<types::Contents> {
        self.path.execute_as_json()
    }

    pub fn run_and_decode(self) -> burgundy::Result<types::Contents> {
        let mut file = self.run()?;

        let decoded_content = base64::decode_config(&file.content, *GITHUB_BASE64_CONFIG).unwrap();
        let decoded_content_str = String::from_utf8(decoded_content).unwrap();
        file.content = decoded_content_str;

        Ok(file)
    }
}
