//! seed peers for connecting to various networks.
use crate::app_cfg;
use anyhow::Error;
use rand::{seq::SliceRandom, thread_rng};
use reqwest;
use serde::Deserialize;
use std::path::PathBuf;
use url::Url;

#[derive(Deserialize)]
/// A list of host information for upstream fullnodes serving RPC servers
pub struct FullnodePlaylist {
  ///
  pub nodes: Vec<HostInfo>,
}

#[derive(Deserialize)]
/// infor for the RPC peers connection.
pub struct HostInfo {
  ///
  pub note: String,
  ///
  pub url: Url,
}

pub fn make_url(git_path: &str, filename: &str) -> Result<Url, Error> {
  let f = format!("{}/{}", git_path, filename);
  Ok(f.parse()?)
}

/// try to fetch current fullnodes from a URL, or default to a seed peer list
pub fn get_known_fullnodes(seed_url: Url) -> Result<FullnodePlaylist, Error> {
  FullnodePlaylist::http_fetch_playlist(seed_url)
}

impl FullnodePlaylist {
  /// use a URL to load a fullnode playlist
  pub fn http_fetch_playlist(url: Url) -> Result<FullnodePlaylist, Error> {
    let res = reqwest::blocking::get(url)?;
    let play: FullnodePlaylist = serde_json::from_str(&res.text()?)?; //res.text()?.parse()?;
    Ok(play)
  }

  /// extract the urls from the playlist struct
  pub fn get_urls(&self) -> Vec<Url> {
    self
      .nodes
      .iter()
      .filter_map(|a| Some(a.url.to_owned()))
      .collect()
  }

  /// update the app configs file
  pub fn update_config_file(&self, path: Option<PathBuf>) -> Result<(), Error> {
    let mut new_cfg = app_cfg::parse_toml(path)?;
    let mut peers = self.get_urls();
    let mut rng = thread_rng();
    peers.shuffle(&mut rng);

    new_cfg.profile.upstream_nodes = peers;

    new_cfg.save_file()
  }
}
