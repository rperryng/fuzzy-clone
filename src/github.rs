use anyhow::{anyhow, bail, Context, Result};
use log::info;
use regex::Regex;
use reqwest::blocking::{Client, Response};
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Deserialize;
use url::Url;

const API_URL: &str = "https://api.github.com/user/repos";
const ACCEPT_VALUE: &str = "application/vnd.github.v3+json";
const GH_ACCESS_TOKEN: &str = dotenv!("GH_ACCESS_TOKEN");
const GH_USER_NAME: &str = dotenv!("GH_USER_NAME");

lazy_static! {
    static ref CLIENT: Client = Client::new();
    static ref LINK_PATTERN: Regex = Regex::new(r#"<.+\bpage\b=(\d+).+>; rel="last""#).unwrap();
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

pub fn get_repo_names() -> Result<Vec<String>> {
    let response = request()?;

    let num_pages = response
        .headers()
        .get("link")
        .context("failed to read link")
        .and_then(|header| header.to_str().map_err(|e| anyhow!(e)))
        .and_then(|text| parse_num_pages_from_link_header(text))?;

    info!("num_pages: {}", num_pages);

    parse_response(response)
}

fn parse_num_pages_from_link_header(link: &str) -> Result<i32> {
    LINK_PATTERN
        .captures(link)
        .context(format!("Failed to match link: {}", link))?
        .get(1)
        .context(format!(
            "Failed to match 'page' element from link: {}",
            link
        ))?
        .as_str()
        .parse::<i32>()
        .map_err(|e| anyhow!(e))
}

fn parse_response(response: Response) -> Result<Vec<String>> {
    let repo_names = response
        .json::<Vec<Repo>>()
        .context("Failed to deserialize github response")?
        .iter()
        .map(|repo| repo.full_name.to_string())
        .collect();

    Ok(repo_names)
}

fn request() -> Result<Response> {
    CLIENT
        .get(API_URL)
        .header(USER_AGENT, GH_USER_NAME)
        .header(ACCEPT, ACCEPT_VALUE)
        .basic_auth(GH_USER_NAME, Some(GH_ACCESS_TOKEN))
        .query(&[
            ("page", "1"),
            ("per_page", "100"), // 100
        ])
        .send()
        .context(format!("Failed to send request to GitHub"))
}
