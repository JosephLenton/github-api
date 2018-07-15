use extern::burgundy;

pub mod repo;

pub struct GithubGetReposOwner {
    crate path: burgundy::Path,
}

impl GithubGetReposOwner {
    pub fn repo(
        self,
        repo: &str,
    ) -> repo::GithubGetReposOwnerRepo {
        repo::GithubGetReposOwnerRepo {
            path: self.path.push(&repo),
        }
    }
}
