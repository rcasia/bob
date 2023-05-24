use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::fs;

use super::types::{LocalNightly, UpstreamVersion};
use crate::{config::Config, helpers::directories};

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoCommit {
    pub commit: Commit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    pub author: CommitAuthor,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitAuthor {
    pub name: String,
}


pub async fn get_upstream_stable(client: &Client) -> Result<UpstreamVersion> {
    let response = client
        .get("https://api.github.com/repos/neovim/neovim/releases/tags/stable")
        .header("user-agent", "bob")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?
        .text()
        .await?;

    serde_json::from_str(&response)
        .map_err(|_| anyhow!("Failed to get upstream nightly version"))
}

pub async fn get_local_stable(config: &Config) -> Result<UpstreamVersion> {
    let downloads_dir = directories::get_downloads_directory(config).await?;
    if let Ok(file) =
        fs::read_to_string(format!("{}/stable/bob.json", downloads_dir.display())).await
    {
        let file_json: UpstreamVersion = serde_json::from_str(&file)?;
        Ok(file_json)
    } else {
        Err(anyhow!("Couldn't find bob.json"))
    }
}
