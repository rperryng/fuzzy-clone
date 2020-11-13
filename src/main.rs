use anyhow::Result;
use dotenv::dotenv;
use log::info;

mod github;

#[macro_use]
extern crate dotenv_codegen;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let repo_names = github::get_repo_names().await?;

    info!("repo_names: {:?} entries", repo_names.len());
    info!("repo_names: {:?} entries", repo_names);

    Ok(())
}
