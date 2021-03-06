use crate::git;
use crate::github;
use crate::repo_names_store;
use anyhow::{Context, Result};
use clap::{Arg, ArgMatches};
use std::path::PathBuf;
use std::env;

const ARG_USERNAME: &str = "username";
const ARG_TOKEN: &str = "token";
const ARG_OUTPUT_FILE: &str = "output_file";
const ARG_DIRECTORY: &str = "directory";
const ARG_FORCE: &str = "force";

pub struct Config {
    git: git::Config,
    github: github::Config,
    repo_names_store: repo_names_store::Config,
    force_refresh: bool,
}

impl Config {
    pub fn git(&self) -> &git::Config {
        &self.git
    }

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
        git: git::Config::new(args.value_of(ARG_DIRECTORY).map(|dir| PathBuf::from(dir))),
        github: github::Config::new(
            args.value_of(ARG_USERNAME)
                .context("username arg required.")?
                .to_owned(),
            args.value_of(ARG_TOKEN)
                .map(|t| t.to_owned())
                .or(env::var("GH_ACCESS_TOKEN").ok())
                .context("--token, -t, or GH_ACCESS_TOKEN must be set to a valid GitHub personal access token.")?,
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
    clap::App::new("fuzzy-clone")
        .version("1.0")
        .author("rperryng")
        .about("Fuzzy search your authorized GitHub repositories and clone selected entries")
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
        .arg(
            Arg::new(ARG_DIRECTORY)
                .short('d')
                .long("--directory")
                .takes_value(true)
                .value_name("CLONING_DIRECTORY")
                .required(false)
                .about("Directory where newly cloned projects should go into"),
        )
        .get_matches()
}
