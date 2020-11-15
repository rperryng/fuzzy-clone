use anyhow::{Context, Result};
use dirs::home_dir;
use std::future::Future;
use std::path::PathBuf;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

lazy_static! {
    pub static ref DEFAULT_PATH: String =
        format!("{}/.fzf_repos", home_dir().unwrap().to_str().unwrap());
}

const DAY_IN_SECONDS: u64 = 60 * 60 * 24;

#[derive(Debug, Clone)]
pub struct Config {
    path: String,
}

impl Config {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

pub struct RepoNamesStore {
    path: PathBuf,
    repo_names: Option<Vec<String>>,
}

impl RepoNamesStore {
    pub fn new(config: Config) -> Self {
        let mut path = PathBuf::new();
        path.push(config.path);
        Self {
            path,
            repo_names: None,
        }
    }

    pub async fn fetch(
        &self,
        bust_cache: &bool,
        f: impl Future<Output = Vec<String>>,
    ) -> Result<Vec<String>> {
        let repo_names: Vec<String> = if *bust_cache || self.cache_is_stale().await? {
            f.await
        } else {
            self.read_repo_names().await?
        };

        self.write_repo_names(&repo_names).await?;
        Ok(repo_names)
    }

    async fn cache_is_stale(&self) -> Result<bool> {
        let modified = fs::metadata(self.path.to_owned()).await?.modified()?;
        let stale = modified.elapsed()? > std::time::Duration::from_secs(DAY_IN_SECONDS);
        Ok(stale)
    }

    async fn write_repo_names(&self, repo_names: &Vec<String>) -> Result<()> {
        let mut f = File::create(self.path.to_owned())
            .await
            .context(format!("couldn't create file {:?}", self.path))?;
        f.write_all(repo_names.join("\n").as_bytes())
            .await
            .context("couldn't write file")?;
        Ok(())
    }

    async fn read_repo_names(&self) -> Result<Vec<String>> {
        // TODO: memoize
        match self.repo_names.to_owned() {
            Some(names) => Ok(names),
            None => self.read_file().await,
        }
    }

    async fn read_file(&self) -> Result<Vec<String>> {
        let repo_names: Vec<String> = fs::read_to_string(self.path.clone())
            .await?
            .split("\n")
            .map(|name| name.to_string())
            .collect::<Vec<String>>();

        Ok(repo_names)
    }
}
