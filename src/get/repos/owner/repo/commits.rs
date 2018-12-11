use extern::burgundy;
pub mod path;
pub mod sha;
use types;

pub struct GithubGetReposOwnerRepoCommits {
    crate path: burgundy::Path,
}

impl GithubGetReposOwnerRepoCommits {
    pub fn run(self) -> burgundy::Result<Vec<types::Commit>> {
        self.path.execute_as_json::<(), Vec<types::Commit>>(None)
    }

    pub fn sha(self, commit_sha:&str) -> sha::GithubGetReposOwnerRepoCommitsSha {
        sha::GithubGetReposOwnerRepoCommitsSha {
            path: self.path.push(&commit_sha),
        }
    }

    pub fn path(self, file:&str) -> path::GithubGetReposOwnerRepoCommitsPath {
        path::GithubGetReposOwnerRepoCommitsPath {
            path: self.path.query(&"path", &file),
        }
    }
}
