use extern::burgundy;

pub mod path;

pub struct GithubGetReposOwnerRepoContents {
    crate path: burgundy::Path,
}

impl GithubGetReposOwnerRepoContents {
    pub fn path(
        self,
        path: &str,
    ) -> path::GithubGetReposOwnerRepoContentsPath {
        path::GithubGetReposOwnerRepoContentsPath {
            path: self.path.push(&path),
        }
    }
}
