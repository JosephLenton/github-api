use extern::burgundy;

pub mod owner;

pub struct GithubGetOrgs {
    crate path: burgundy::Path,
}

impl GithubGetOrgs {
    pub fn owner(
        self,
        owner: &str,
    ) -> owner::GithubGetOrgsOwner {
        owner::GithubGetOrgsOwner {
            path: self.path.push(&owner),
        }
    }
}
