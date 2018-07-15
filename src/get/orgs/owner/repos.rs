use extern::burgundy;
use types;

// The number of items that should appear in a Github listings.
const NUM_GITHUB_REPOS_PER_PAGE: usize = 100;

pub struct GithubGetOrgsOwnerRepos {
    crate path: burgundy::Path,
}

impl GithubGetOrgsOwnerRepos {
    pub fn run(self) -> burgundy::Result<Vec<types::Repo>> {
        self.path
            .query(&"per_page", &NUM_GITHUB_REPOS_PER_PAGE)
            .execute_as_json::<Vec<types::Repo>>()
    }
}
