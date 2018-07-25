use extern::burgundy;

pub mod revision;

pub struct GithubGetReposOwnerRepoCompare {
    crate path: burgundy::Path,
}

impl GithubGetReposOwnerRepoCompare {
    pub fn master(
        self,
        base: &str,
    ) -> revision::GithubGetReposOwnerRepoCompareRevision {
        self.diff(base, &"master")
    }

    pub fn diff(
        self,
        base: &str,
        upper: &str,
    ) -> revision::GithubGetReposOwnerRepoCompareRevision {
        revision::GithubGetReposOwnerRepoCompareRevision {
            path: self.path.push(&base).push_partial(&"...").push_partial(&upper),
        }
    }
}
