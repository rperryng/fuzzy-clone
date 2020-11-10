use anyhow::{Context, Result};
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Deserialize;

const API_URL: &str = "https://api.github.com/user/repos";

#[derive(Deserialize, Debug)]
struct Repo {
    full_name: String,
}

pub fn get_repo_names() -> Result<Vec<String>> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(API_URL)
        .header(USER_AGENT, "rperryng")
        .header(ACCEPT, "application/vnd.github.v3+json")
        .basic_auth("rperryng", Some(dotenv!("GH_ACCESS_TOKEN")))
        .query(&[
            ("page", "7"),
            ("per_page", "100"), // 100
        ])
        .send()
        .with_context(|| format!("Failed to send request to GitHub"))?;

    if let Some(link) = response.headers().get("Link") {
        println!("got headers: {:#?}", link);
    } else {
        println!("wat, no 'Link' header");
    }

    let repo_names = response
        .json::<Vec<Repo>>()
        .with_context(|| "Failed to deserialize github response")?
        .iter()
        .map(|repo| repo.full_name.to_string())
        .collect();
    Ok(repo_names)
}
