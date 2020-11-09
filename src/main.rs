use anyhow::{Context, Result};
use dotenv::dotenv;
use regex::Regex;
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Deserialize;

use std::process::{Command, Stdio};

#[macro_use]
extern crate dotenv_codegen;

fn main() -> Result<()> {
    dotenv().ok();

    let input =
        r#"
"<https://api.github.com/user/repos?page=1&per_page=100>; rel=\"prev\", <https://api.github.com/user/repos?page=1&per_page=100>; rel=\"last\", <https://api.github.com/user/repos?page=1&per_page=100>; rel=\"first\""
        "#
        .trim();

    let link_pattern = Regex::new(r#"<(.+)>; rel=\\"(.+)\\""#).unwrap();
    let caps = link_pattern.captures(input)
        .with_context(|| format!("couldn't parse page from input {}", input))?;

    get_repos();
    // call_fzf();

    Ok(())
}

fn parse_links(link_raw: &str) -> Result<()> {
    // <https://api.github.com/user/repos?page=1&per_page=100>;
    // rel=\"prev\", <https://api.github.com/user/repos?page=1&per_page=100>; rel=\"last\", <https://api.github.com/user/repos?page=1&per_page=100>; rel=\"first\"
    Ok(())
}

fn call_fzf() -> Result<()> {
    // Command::new("fzf").stdin(Stdio::piped)?;

    Ok(())
}

fn get_repos() -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.github.com/user/repos")
        .header(USER_AGENT, "rperryng")
        .header(ACCEPT, "application/vnd.github.v3+json")
        .basic_auth("rperryng", Some(dotenv!("GH_ACCESS_TOKEN")))
        .query(&[
            ("page", "7"),
            ("per_page", "100") // 100
        ])
        .send()?;

    if let Some(link) = response.headers().get("Link") {
        println!("got headers: {:#?}", link);
    } else {
        println!("wat, no 'Link' header");
    }

    Ok(())
}
