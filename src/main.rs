use anyhow::Result;
use dotenv::dotenv;
use log::info;
use github::Github;

mod file;
mod github;
mod config;

#[macro_use]
extern crate dotenv_codegen;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = config::config()?;
    let github = Github::new(config.github().clone());
    let repo_names = github.get_repo_names().await?;

    info!("repo_names: {:?} entries", repo_names.len());
    info!("repo_names: {:?} entries", repo_names);

    file::write_repo_names(repo_names)?;

    Ok(())
}
