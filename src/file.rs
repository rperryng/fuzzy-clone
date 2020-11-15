use anyhow::{Context, Result};
use dirs::home_dir;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

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

// Convert this to a "Repo Names Datastore" instead.
pub struct FileUtils {
    path: PathBuf,
    repo_names: Option<Vec<String>>,
}

impl FileUtils {
    pub fn new(config: Config) -> Self {
        let mut path = PathBuf::new();
        path.push(config.path);
        Self {
            path,
            repo_names: None,
        }
    }

    pub fn cache_is_stale(&self) -> Result<bool> {
        let modified = fs::metadata(self.path.clone())?.modified()?;
        let stale = modified.elapsed()? > std::time::Duration::from_secs(DAY_IN_SECONDS);
        Ok(stale)
    }

    pub fn write_repo_names(&self, repo_names: &Vec<String>) -> Result<()> {
        let mut f = File::create(self.path.clone()).context("couldn't create file")?;
        f.write_all(repo_names.join("\n").as_bytes())
            .context("couldn't write file")?;
        Ok(())
    }

    pub fn read_repo_names(&self) -> Result<Vec<String>> {
        match self.repo_names.clone() {
            Some(names) => Ok(names),
            None => self.read_file(),
        }
    }

    fn read_file(&self) -> Result<Vec<String>> {
        let repo_names: Vec<String> = fs::read_to_string(self.path.clone())?
            .split("\n")
            .map(|name| name.to_string())
            .collect::<Vec<String>>();

        Ok(repo_names)
    }
}
