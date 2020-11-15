use anyhow::{Context, Result};
use futures::future::try_join_all;

use log::info;
use std::path::PathBuf;

use tokio::process::Command;

#[derive(Debug, Clone)]
pub struct Config {
    parent_path: Option<PathBuf>,
}

impl Config {
    pub fn new(parent_path: Option<PathBuf>) -> Self {
        Self { parent_path }
    }
}

pub struct Git {
    config: Config,
}

impl Git {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn clone_repos(&self, repos: &Vec<String>) -> Result<()> {
        let clone_commands = repos.iter().map(|repo| {
            let repo_pointer = format!("git@github.com:{}.git", repo);

            let mut cmd = Command::new("git");

            if let Some(parent_path) = &self.config.parent_path {
                info!("cloning to directory: {:?}", parent_path);
                cmd.arg("-C");
                cmd.arg(parent_path);
            }

            cmd.arg("clone");
            cmd.arg(repo_pointer);

            async move {
                cmd.output()
                    .await
                    .map(|output| (repo, output))
                    .context(format!("Failed to run clone command for {}", repo))
            }
        });

        let outputs = try_join_all(clone_commands).await?;

        for output in outputs {
            let (repo, output) = output;
            println!(
                "repo: [{}]:\n\
                status: {}\n\
                stdout: {}\n\
                stderr: {}",
                repo,
                output.status,
                String::from_utf8(output.stdout)?,
                String::from_utf8(output.stderr)?,
            );
        }

        Ok(())
    }
}
