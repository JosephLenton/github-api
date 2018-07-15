use extern::burgundy;

pub mod contents;

pub struct GithubGetReposOwnerRepo {
    crate path: burgundy::Path,
}

impl GithubGetReposOwnerRepo {
    pub fn contents(self) -> contents::GithubGetReposOwnerRepoContents {
        contents::GithubGetReposOwnerRepoContents {
            path: self.path.push(&"contents"),
        }
    }
}
