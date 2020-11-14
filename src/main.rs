use anyhow::Result;
use dotenv::dotenv;
use github::Github;
use log::info;

mod config;
mod file;
mod github;

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

    if *config.force_refresh() || f.cache_is_stale()? {
        refresh_cache(config).await?;
    }

    Ok(())
}

async fn refresh_cache(config: config::Config) -> Result<()> {
    let github = Github::new(config.github().clone());
    let repo_names = github.get_repo_names().await?;

    info!("repo_names: {:?} entries", repo_names.len());
    info!("repo_names: {:?} entries", repo_names);

    let f = file::FileUtils::new(config.file().clone());
    f.write_repo_names(repo_names)?;

    Ok(())
}
