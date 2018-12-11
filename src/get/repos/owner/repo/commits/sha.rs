use extern::burgundy;
use types;

pub struct GithubGetReposOwnerRepoCommitsSha {
    crate path: burgundy::Path,
}

impl GithubGetReposOwnerRepoCommitsSha {
    pub fn run(self) -> burgundy::Result<types::CommitWithFiles> {
        self.path.execute_as_json::<(), types::CommitWithFiles>(None)
    }
}
