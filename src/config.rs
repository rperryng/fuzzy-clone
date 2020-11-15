use crate::repo_names_store;
use crate::github;
use anyhow::{Context, Result};
use clap::{Arg, ArgMatches};

const ARG_USERNAME: &str = "username";
const ARG_TOKEN: &str = "token";
const ARG_OUTPUT_FILE: &str = "output_file";
const ARG_FORCE: &str = "force";

pub struct Config {
    github: github::Config,
    repo_names_store: repo_names_store::Config,
    force_refresh: bool,
}

impl Config {
    pub fn github(&self) -> &github::Config {
        &self.github
    }

    pub fn file(&self) -> &repo_names_store::Config {
        &self.repo_names_store
    }

    pub fn force_refresh(&self) -> &bool {
        &self.force_refresh
    }
}

pub fn config() -> Result<Config> {
    let args = read_args();
    let config = Config {
        github: github::Config::new(
            args.value_of(ARG_USERNAME)
                .context("username arg required.")?
                .to_owned(),
            args.value_of(ARG_TOKEN)
                .or_else(|| Some(dotenv!("GH_ACCESS_TOKEN")))
                .map(|t| t.to_owned())
                .context("personal access token arg or GH_ACCESS_TOKEN must be set.")?,
        ),
        repo_names_store: repo_names_store::Config::new(
            args.value_of(ARG_OUTPUT_FILE)
                .or(Some(&crate::repo_names_store::DEFAULT_PATH))
                .unwrap()
                .to_string(),
        ),
        force_refresh: args.is_present(ARG_FORCE),
    };

    Ok(config)
}

fn read_args() -> ArgMatches {
    clap::App::new("fzf-repo-clone")
        .version("1.0")
        .author("rperryng")
        .about("Use FZF to clone github projects")
        .arg(
            Arg::new(ARG_USERNAME)
                .short('u')
                .long("username")
                .takes_value(true)
                .value_name("USERNAME")
                .required(true)
                .about("GitHub username"),
        )
        .arg(
            Arg::new(ARG_TOKEN)
                .short('t')
                .long("--token")
                .takes_value(true)
                .value_name("USERNAME")
                .required(false)
                .about("GitHub username"),
        )
        .arg(
            Arg::new(ARG_OUTPUT_FILE)
                .short('o')
                .long("--output-file")
                .takes_value(true)
                .value_name("OUTPUT_FILE")
                .required(false)
                .about("Location to cache repo names"),
        )
        .arg(
            Arg::new(ARG_FORCE)
                .short('f')
                .long("--force")
                .about("Wipe and re-hydrate the cache of repo names"),
        )
        .get_matches()
}
