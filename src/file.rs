use anyhow::{Context, Result};
use dirs::home_dir;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

lazy_static! {
    static ref FILENAME: String = format!("{}/.fzf_repos", home_dir().unwrap().to_str().unwrap());
}

pub fn write_repo_names(repo_names: Vec<String>) -> Result<()> {
    let mut f = File::create(Path::new(&FILENAME.to_string())).context("couldn't create file")?;
    f.write_all(repo_names.join("\n").as_bytes())
        .context("couldn't write file")?;
    Ok(())
}
