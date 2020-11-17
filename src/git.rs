use anyhow::{Context, Result};
use console::{style, Term};
use futures::future::try_join_all;
use futures::Future;

use log::info;
use std::path::PathBuf;

use std::process::Output;
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
        let clone_commands = repos.iter().map(|repo| self.build_clone_command(repo));
        let term = Term::stdout();

        term.write_str(&format!("Cloning repos {:?}", repos))?;
        let results = try_join_all(clone_commands).await?;
        term.clear_line()?;

        self.print_results(&term, results)?;

        Ok(())
    }

    fn print_results(&self, term: &Term, results: Vec<(String, Output)>) -> Result<()> {
        for (repo, output) in results {
            if output.status.success() {
                term.write_str(&format!(
                    "Successfully cloned {}",
                    style(repo.to_owned()).green().to_string()
                ))?;
            } else {
                term.write_line(
                    &style(format!(
                        "Failed to clone \"{}\":\n  {}",
                        repo,
                        String::from_utf8(output.stderr)?
                    ))
                    .red()
                    .to_string(),
                )?;
            }
        }

        Ok(())
    }

    fn build_clone_command(&self, repo: &str) -> impl Future<Output = Result<(String, Output)>> {
        let repo_pointer = format!("git@github.com:{}.git", repo);
        let mut cmd = Command::new("git");

        if let Some(parent_path) = &self.config.parent_path {
            info!("\ncloning to directory: {:?}", parent_path);
            cmd.arg("-C");
            cmd.arg(parent_path);
        }

        cmd.arg("clone");
        cmd.arg(repo_pointer);

        let repo = repo.to_owned();
        async move {
            cmd.output()
                .await
                .context(format!("Failed to run clone command for {}", repo))
                .map(|output| (repo, output))
        }
    }
}
