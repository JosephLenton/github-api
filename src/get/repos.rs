use extern::burgundy;

pub mod owner;

pub struct GithubGetRepos {
    crate path: burgundy::Path,
}

impl GithubGetRepos {
    pub fn owner(
        self,
        owner: &str,
    ) -> owner::GithubGetReposOwner {
        owner::GithubGetReposOwner {
            path: self.path.push(&owner),
        }
    }
}
