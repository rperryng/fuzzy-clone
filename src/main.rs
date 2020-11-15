use anyhow::{Context, Result};
use dotenv::dotenv;
use github::Github;
use log::info;
use std::io::Write;
use std::process::{Command, Stdio};

mod config;
mod file;
mod github;

extern crate skim;

#[macro_use]
extern crate dotenv_codegen;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = config::config()?;

    let f = file::FileUtils::new(config.file().clone());

    let repo_names: Vec<String> = if *config.force_refresh() || f.cache_is_stale()? {
        info!("loading repo names from API");
        refresh_cache(&config).await?
    } else {
        info!("loading repo names from cache");
        f.read_repo_names()?
    };

    run_fzf(repo_names)?;

    Ok(())
}

fn run_fzf(input: Vec<String>) -> Result<()> {
    info!("starting fzf");

    let input = input.join("\n");
    let input_bytes = input.as_bytes();

    let mut fzf = Command::new("fzf")
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Couldn't start fzf")?;

    let fzf_stdin = fzf
        .stdin
        .as_mut()
        .context("Couldn't get fzf stdin handle")?;
    fzf_stdin.write_all(input_bytes)?;

    info!("Waiting for FZF to finish.");
    let fzf_result = fzf.wait_with_output()?.stdout;

    info!("got FZF output: {}", String::from_utf8(fzf_result)?);

    Ok(())
}

async fn refresh_cache(config: &config::Config) -> Result<Vec<String>> {
    let github = Github::new(config.github().clone());
    let repo_names = github.get_repo_names().await?;

    //     info!("repo_names: {:?} entries", repo_names.len());
    //     info!("repo_names: {:?} entries", repo_names);

    let f = file::FileUtils::new(config.file().clone());
    f.write_repo_names(&repo_names)?;

    Ok(repo_names)
}
