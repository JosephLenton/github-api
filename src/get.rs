use extern::burgundy;

pub mod orgs;
pub mod repos;

pub struct GithubGet {
    crate path: burgundy::Path,
}

impl GithubGet {
    pub fn orgs(self) -> orgs::GithubGetOrgs {
        orgs::GithubGetOrgs {
            path: self.path.push(&"orgs"),
        }
    }

    pub fn repos(self) -> repos::GithubGetRepos {
        repos::GithubGetRepos {
            path: self.path.push(&"repos"),
        }
    }
}
