use anyhow::Result;
use dotenv::dotenv;
use reqwest::header::USER_AGENT;
use serde::Deserialize;

#[macro_use]
extern crate dotenv_codegen;

fn main() -> Result<()> {
    println!("Hello, world!");

    dotenv().ok();

    let client = reqwest::blocking::Client::new();
    let body = client
        .get("https://api.github.com/user/repos")
        .header(USER_AGENT, "rperryng")
        .basic_auth("rperryng", Some(dotenv!("GH_ACCESS_TOKEN")))
        .query(&[
            ("page", "1"),
            ("per_page", "100")
        ])
        .send()?
        .text()?;

    println!("got response: {:#?}", body);

    Ok(())
}
