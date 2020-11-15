use anyhow::{Result};
use dotenv::dotenv;

use github::Github;
use log::info;
use repo_names_store::RepoNamesStore;



mod config;
mod fuzzy;
mod github;
mod repo_names_store;

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
    let repo_names_store = RepoNamesStore::new(config.file().clone());

    let all_repo_names = repo_names_store
        .fetch(config.force_refresh(), async {
            let github = Github::new(config.github().clone());
            github
                .get_repo_names()
                .await
                .expect("couldn't fetch repo games from GitHub")
        })
        .await?;

    let selected_repo_names = fuzzy::fuzzy(all_repo_names)?;
    info!("Got repo names: {:?}", selected_repo_names);



    Ok(())
}
