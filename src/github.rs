use anyhow::{anyhow, Context, Result};
use futures::future::try_join_all;
use log::info;
use regex::Regex;
use reqwest::header::{ACCEPT, USER_AGENT};
use reqwest::{Client, Response};
use serde::Deserialize;
use url::Url;

const API_REPO_URL_STR: &str = "https://api.github.com/user/repos";
const ACCEPT_VALUE: &str = "application/vnd.github.v3+json";

lazy_static! {
    static ref CLIENT: Client = Client::new();
    static ref LINK_PATTERN: Regex = Regex::new(r#"<.+\bpage\b=(\d+).+>;\s*rel="last""#).unwrap();
    static ref API_REPO_URL: Url = Url::parse(API_REPO_URL_STR).unwrap();
    static ref CONCURRENT_REQUESTS: usize =
        dotenv!("CONCURRENT_REQUESTS").parse::<usize>().unwrap();
}

#[derive(Debug, Clone)]
pub struct Config {
    username: String,
    personal_access_token: String,
}

impl Config {
    pub fn new(username: String, personal_access_token: String) -> Self {
        Self {
            username,
            personal_access_token,
        }
    }
}

pub struct Github {
    config: Config,
}

#[derive(Deserialize, Debug)]
struct Repo {
    full_name: String,
}

#[derive(Debug)]
struct Link {
    url: String,
    link_type: String,
}

impl Github {
    pub fn new(config: Config) -> Github {
        Github { config }
    }

    pub async fn get_repo_names(&self) -> Result<Vec<String>> {
        let response = self.request(&1).await?;
        let link_header = response
            .headers()
            .get("link")
            .context("couldn't find link header in response")?;
        let num_pages = self.parse_num_pages_from_link_header(link_header.to_str()?)?;

        info!("num_pages: {}", num_pages);

        let mut repo_names = self.parse_response(response).await?;
        let mut reminaing_repo_names = self.get_remaining_repos(num_pages).await?;

        repo_names.append(&mut reminaing_repo_names);
        repo_names.sort();

        Ok(repo_names)
    }

    async fn get_remaining_repos(&self, num_pages: u32) -> Result<Vec<String>> {
        let pages: Vec<u32> = (2..=num_pages).collect();
        let requests = pages.iter().map(|page_num| {
            CLIENT
                .get(API_REPO_URL_STR)
                .header(USER_AGENT, self.config.username.clone())
                .header(ACCEPT, ACCEPT_VALUE)
                .basic_auth(
                    self.config.username.clone(),
                    Some(self.config.personal_access_token.clone()),
                )
                .query(&[
                    ("page", page_num.to_string()),
                    ("per_page", "100".to_string()), // 100
                ])
                .send()
        });

        let responses: Vec<Response> = try_join_all(requests).await?;
        let repo_names = responses
            .into_iter()
            .map(|response| self.parse_response(response));
        let repo_names: Vec<_> = try_join_all(repo_names)
            .await?
            .iter()
            .flatten()
            .map(|s| s.to_string())
            .collect();

        Ok(repo_names)
    }

    fn parse_num_pages_from_link_header(&self, link: &str) -> Result<u32> {
        info!("link: {}", link);
        LINK_PATTERN
            .captures(link)
            .context(format!("Failed to match link: {}", link))?
            .get(1)
            .context(format!(
                "Failed to match 'page' element from link: {}",
                link
            ))?
            .as_str()
            .parse::<u32>()
            .map_err(|e| anyhow!(e))
    }

    async fn parse_response(&self, response: Response) -> Result<Vec<String>> {
        let repo_names: Vec<String> = response
            .json::<Vec<Repo>>()
            .await
            .context("Failed to deserialize github response")?
            .iter()
            .map(|repo| repo.full_name.to_string())
            .collect();

        Ok(repo_names)
    }

    async fn request(&self, page: &u32) -> Result<Response> {
        CLIENT
            .get(API_REPO_URL_STR)
            .header(USER_AGENT, self.config.username.clone())
            .header(ACCEPT, ACCEPT_VALUE)
            .basic_auth(
                self.config.username.clone(),
                Some(self.config.personal_access_token.clone()),
            )
            .query(&[
                ("page", page),
                ("per_page", &100), // 100
            ])
            .send()
            .await
            .map_err(|e| anyhow!(e))
    }
}
