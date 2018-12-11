#![feature(crate_visibility_modifier)]
#![feature(extern_in_paths)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

use extern::burgundy;
pub use extern::burgundy::Error;
pub use extern::burgundy::Result;

pub mod get;
pub mod types;

const GITHUB_DOMAIN: &'static str = "https://api.github.com";

lazy_static! {
    static ref USER_AGENT: String = format!("github-api (Rust)/{}", env!("CARGO_PKG_VERSION"));
}

pub struct Github {
    domain: burgundy::Domain,
}

impl Github {
    pub fn new() -> Self {
        let mut domain = burgundy::Domain::new(&GITHUB_DOMAIN);
        domain.header(&"User-Agent", &USER_AGENT.as_str());

        Self {
            domain,
        }
    }

    pub fn new_with_oath(oath_token: &str) -> Self {
        let mut client = Self::new();
        client.domain.header(&"Authorization", &format!("token {}", oath_token));
        client
    }

    pub fn get(&self) -> get::GithubGet {
        get::GithubGet {
            path: self.domain.get(),
        }
    }
}
