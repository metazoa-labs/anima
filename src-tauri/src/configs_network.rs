//! network configs

use anyhow::{bail, Error};
use std::fmt;
use url::Url;

use crate::{
  app_cfg::AppCfg,
  configs::{self},
  rpc_playlist::{self, FullnodePlaylist},
  wallet_error::WalletError,
};

static DEFAULT_GIT: &str = "https://raw.githubusercontent.com/metazoa-labs/anima/api/seed_peers/";

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NetworkProfile {
  pub chain_id: NetworkID, // Todo, use the Network Enum
  pub urls: Vec<Url>,
  pub waypoint: String, // NOTE: Use the Actual Waypoint Type
  pub profile: String,  // tbd, to use default node, or to use upstream, or a custom url.
}

impl NetworkProfile {
  pub fn new() -> Result<Self, WalletError> {
    let cfg = configs::get_cfg()?;
    Ok(NetworkProfile {
      chain_id: cfg.chain_info.chain_id,
      urls: cfg.profile.upstream_nodes,
      waypoint: cfg.chain_info.base_waypoint.unwrap_or_default(),
      profile: "default".to_string(),
    })
  }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum NetworkID {
  Mainnet,
  Testnet,
  Devnet,
  Local,
  Custom { playlist_url: Url },
}

impl fmt::Display for NetworkID {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
    // or, alternatively:
    // fmt::Debug::fmt(self, f)
  }
}

pub fn set_network_configs(network: NetworkID) -> Result<NetworkProfile, WalletError> {
  dbg!("toggle network", &network);
  let playlist = match &network {
    NetworkID::Testnet => FullnodePlaylist::http_fetch_playlist(rpc_playlist::make_url(
      DEFAULT_GIT,
      "fullnode_seed_playlist_testnet",
    )?)?,

    // fullnode_seed_playlist_testnet
    NetworkID::Devnet => FullnodePlaylist::http_fetch_playlist(rpc_playlist::make_url(
      DEFAULT_GIT,
      "fullnode_seed_playlist_devnet",
    )?)?,

    NetworkID::Local => FullnodePlaylist::http_fetch_playlist(rpc_playlist::make_url(
      DEFAULT_GIT,
      "fullnode_seed_playlist_local",
    )?)?,

    NetworkID::Custom { playlist_url } => {
      FullnodePlaylist::http_fetch_playlist(playlist_url.to_owned())?
    }

    NetworkID::Mainnet => FullnodePlaylist::http_fetch_playlist(rpc_playlist::make_url(
      DEFAULT_GIT,
      "fullnode_seed_playlist",
    )?)?,
  };

  playlist.update_config_file(None)?; // None uses default path of anima_canary.toml

  // TODO: I don't think chain ID needs to change.
  set_chain_id(network).map_err(|e| {
    let err_msg = format!("could not set chain id, message: {}", &e.to_string());
    WalletError::misc(&err_msg)
  })?;

  set_waypoint_from_upstream()?;

  NetworkProfile::new()
}

pub fn set_waypoint_from_upstream() -> Result<AppCfg, Error> {

  // try getting waypoint from upstream nodes
  // no waypoint is necessary in advance.

  //////////////////////////////
  // NOTE:  do a request here.
  let cfg = set_waypoint("0".to_string())?;
  /////////////////////

  Ok(cfg)
}

// TODO: Use proper Type for Waypoint
/// Set the base_waypoint used for client connections.
pub fn set_waypoint(wp: String) -> Result<AppCfg, Error> {
  let mut cfg = configs::get_cfg()?;
  cfg.chain_info.base_waypoint = Some(wp);
  println!("set_waypoint");
  cfg.save_file()?;
  Ok(cfg)
}

/// Get all the anima_canary configs. For tx sending and upstream nodes
/// Note: The default_node key in anima_canary is not used by anima_canary. anima_canary randomly tests
/// all the endpoints in upstream_peers on every TX.
pub fn override_upstream_node(url: Url) -> Result<AppCfg, Error> {
  let mut cfg = configs::get_cfg()?;
  cfg.profile.upstream_nodes = vec![url];
  cfg.save_file()?;
  Ok(cfg)
}

// the anima_canary configs. For tx sending and upstream nodes
pub fn set_chain_id(chain_id: NetworkID) -> Result<AppCfg, Error> {
  let mut cfg = configs::get_cfg()?;
  cfg.chain_info.chain_id = chain_id;
  cfg.save_file()?;
  Ok(cfg)
}

/// Set the list of upstream nodes
pub fn set_upstream_nodes(vec_url: Vec<Url>) -> Result<AppCfg, Error> {
  let mut cfg = configs::get_cfg()?;
  cfg.profile.upstream_nodes = vec_url;
  cfg.save_file()?;
  Ok(cfg)
}

/// Removes current node from upstream nodes
/// To be used when DB is corrupted for instance.
pub fn remove_node(host: String) -> Result<(), Error> {
  match configs::get_cfg() {
    Ok(mut cfg) => {
      let nodes = cfg.profile.upstream_nodes;
      match nodes.len() {
        1 => bail!("Cannot remove last node"),
        _ => {
          cfg.profile.upstream_nodes = nodes
            .into_iter()
            .filter(|each| !each.to_string().contains(&host))
            .collect();
          cfg.save_file()
        }
      }
    }
    Err(_) => Ok(()),
  }
}
