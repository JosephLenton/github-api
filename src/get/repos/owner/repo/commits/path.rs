use extern::burgundy;
use types;

pub struct GithubGetReposOwnerRepoCommitsPath {
    crate path: burgundy::Path,
}

impl GithubGetReposOwnerRepoCommitsPath {
    pub fn run(self) -> burgundy::Result<Vec<types::Commit>> {
        self.path.execute_as_json::<(), Vec<types::Commit>>(None)
    }
}
