use anyhow::{Context, Result};
use dotenv::dotenv;
use regex::Regex;
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Deserialize;

use std::process::{Command, Stdio};

mod github;

#[macro_use]
extern crate dotenv_codegen;

fn main() -> Result<()> {
    dotenv().ok();

    // let link_pattern = Regex::new(r#"<(.+)>; rel=\\"(.+)\\""#).unwrap();
    // let caps = link_pattern.captures(input)
    //     .with_context(|| format!("couldn't parse page from input {}", input))?;

    let repos = github::get_repos()?;
    // call_fzf();

    println!("repos: {}", repos);

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
