use extern::burgundy;

pub mod compare;
pub mod contents;
pub mod commits;

pub struct GithubGetReposOwnerRepo {
    crate path: burgundy::Path,
}

impl GithubGetReposOwnerRepo {
    pub fn contents(self) -> contents::GithubGetReposOwnerRepoContents {
        contents::GithubGetReposOwnerRepoContents {
            path: self.path.push(&"contents"),
        }
    }

    pub fn compare(self) -> compare::GithubGetReposOwnerRepoCompare {
        compare::GithubGetReposOwnerRepoCompare {
            path: self.path.push(&"compare"),
        }
    }

    pub fn commits(self) -> commits::GithubGetReposOwnerRepoCommits {
        commits::GithubGetReposOwnerRepoCommits {
            path: self.path.push(&"commits"),
        }
    }
}
