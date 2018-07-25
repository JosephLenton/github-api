use extern::burgundy;
use types;

pub struct GithubGetReposOwnerRepoCompareRevision {
    crate path: burgundy::Path,
}

impl GithubGetReposOwnerRepoCompareRevision {
    pub fn run(self) -> burgundy::Result<types::Diff> {
        self.path.execute_as_json()
    }
}
