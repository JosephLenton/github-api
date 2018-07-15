use extern::burgundy;

pub mod repos;

pub struct GithubGetOrgsOwner {
    crate path: burgundy::Path,
}

impl GithubGetOrgsOwner {
    pub fn repos(self) -> repos::GithubGetOrgsOwnerRepos {
        repos::GithubGetOrgsOwnerRepos {
            path: self.path.push(&"repos"),
        }
    }
}
