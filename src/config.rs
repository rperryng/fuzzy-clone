use crate::github;
use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches};

pub struct Config {
    github: github::Config,
}

impl Config {
    pub fn github(&self) -> &github::Config {
        &self.github
    }
}

pub fn config() -> Result<Config> {
    let args = read_args();

    Ok(Config {
        github: github::Config::new(
            args.value_of("username")
                .context("username arg required.")?
                .to_owned(),
            args.value_of("token")
                .or_else(|| Some(dotenv!("GH_ACCESS_TOKEN")))
                .map(|t| t.to_owned())
                .context("personal access token arg or GH_ACCESS_TOKEN must be set.")?,
        ),
    })
}

fn read_args() -> ArgMatches {
    App::new("fzf-repo-clone")
        .version("1.0")
        .author("rperryng")
        .about("Use FZF to clone github projects")
        .arg(
            Arg::new("username")
                .short('u')
                .long("username")
                .takes_value(true)
                .value_name("USERNAME")
                .required(true)
                .about("GitHub username")
        )
        .arg(
            Arg::new("personal_access_token")
                .short('t')
                .long("--token")
                .takes_value(true)
                .value_name("USERNAME")
                .required(false)
                .about("GitHub username")
        )
        .get_matches()
}
