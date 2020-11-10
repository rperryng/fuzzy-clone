use anyhow::{Context, Result};
use log::info;
use reqwest::blocking::{Client, Response};
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Deserialize;

const API_URL: &str = "https://api.github.com/user/repos";
const ACCEPT_VALUE: &str = "application/vnd.github.v3+json";
const GH_ACCESS_TOKEN: &str = dotenv!("GH_ACCESS_TOKEN");
const GH_USER_NAME: &str = dotenv!("GH_USER_NAME");

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

#[derive(Deserialize, Debug)]
struct Repo {
    full_name: String,
}

pub fn get_repo_names() -> Result<Vec<String>> {
    let response = request()?;

    if let Some(link) = response.headers().get("Link") {
        info!("got headers: {:#?}", link);
    } else {
        info!("wat, no 'Link' header");
    }

    parse_response(response)
}

fn parse_response(response: Response) -> Result<Vec<String>> {
    let repos: Vec<Repo> = response
        .json::<Vec<Repo>>()
        .context("Failed to deserialize github response")?;

    let repo_names: Vec<String> = repos
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
            ("page", "7"),
            ("per_page", "100"), // 100
        ])
        .send()
        .context(format!("Failed to send request to GitHub"))
}
