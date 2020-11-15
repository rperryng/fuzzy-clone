use anyhow::{Context, Result};
use dotenv::dotenv;
use github::Github;
use log::info;
use std::io::Write;
use std::process::{Command, Stdio};

mod config;
mod file;
mod github;
mod fuzzy;

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

    let all_repo_names: Vec<String> = if *config.force_refresh() || f.cache_is_stale()? {
        info!("loading repo names from API");
        refresh_cache(&config).await?
    } else {
        info!("loading repo names from cache");
        f.read_repo_names()?
    };

    let selected_repo_names = fuzzy::fuzzy(all_repo_names)?;
    info!("Got repo names: {:?}", selected_repo_names);

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
