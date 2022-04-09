//! Configs for wallet app

use anyhow::Error;
use aptos_types::{transaction::authenticator::AuthenticationKey};
use dirs;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use move_core_types::account_address::AccountAddress;

use std::{
  fs::{self, File},
  io::{Read, Write},
  path::PathBuf,
};

use crate::configs_network::NetworkID;


pub static APP_HOME: &str = ".anima_canary";
pub static CONFIG_FILE: &str = "anima_canary.toml";

/// App Configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppCfg {
  /// Workspace config
  pub workspace: Workspace,
  /// User Profile
  pub profile: Profile,
  /// Chain Info for all users
  pub chain_info: ChainInfo,
}

fn default_path() -> PathBuf {
  dirs::home_dir().unwrap().join(APP_HOME).join(CONFIG_FILE)
}
/// Get a AppCfg object from toml file
pub fn parse_toml(path: Option<PathBuf>) -> Result<AppCfg, Error> {
  let cfg_path = path.unwrap_or(default_path());
  let mut toml_buf = "".to_string();
  let mut file = File::open(&cfg_path)?;
  file.read_to_string(&mut toml_buf)?;
  let cfg: AppCfg = toml::from_str(&toml_buf)?;
  Ok(cfg)
}

/// Get a AppCfg object from toml file
pub fn fix_missing_fields(path: PathBuf) -> Result<(), Error> {
  let cfg: AppCfg = parse_toml(Some(path))?;
  cfg.save_file()?;
  Ok(())
}
/// Default configuration settings.

impl Default for AppCfg {
  fn default() -> Self {
    Self {
      workspace: Workspace::default(),
      profile: Profile::default(),
      chain_info: ChainInfo::default(),
    }
  }
}

impl AppCfg {
  /// save the config file to the workspace home path
  pub fn save_file(&self) -> Result<(), Error> {
    let toml = toml::to_string(&self)?;
    let home_path = &self.workspace.node_home.clone();
    // create home path if doesn't exist, usually only in dev/ci environments.
    fs::create_dir_all(&home_path)?;
    let toml_path = home_path.join(CONFIG_FILE);
    let mut file = fs::File::create(&toml_path)?;
    file.write(&toml.as_bytes())?;

    println!("\nwallet configs saved to: {:?}", &toml_path);
    Ok(())
  }
}

/// Information about the chain
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Workspace {
  /// home directory of the app
  pub node_home: PathBuf,
}

impl Default for Workspace {
  fn default() -> Self {
    Self {
      node_home: dirs::home_dir().unwrap().join(APP_HOME),
    }
  }
}

/// Information about the chain
#[derive(Clone, Debug, Deserialize, Serialize)]
// #[serde(deny_unknown_fields)]
pub struct ChainInfo {
  /// Chain that this work is being committed to
  pub chain_id: NetworkID,

  /// Epoch from which the node started syncing
  pub base_epoch: Option<u64>,

  /// Waypoint from which the node started syncing
  pub base_waypoint: Option<String>,
}

// TODO: These defaults serving as test fixtures.
impl Default for ChainInfo {
  fn default() -> Self {
    Self {
      chain_id: NetworkID::Devnet,
      base_epoch: Some(0),
      // Mock Waypoint.
      base_waypoint: Some("0:0".to_owned()),
    }
  }
}
/// App account profile
#[derive(Clone, Debug, Deserialize, Serialize)]
// #[serde(deny_unknown_fields)]
// NOTE: all account and auth_key should use the actual type, not String.
pub struct Profile {
  /// The wallet account
  pub account: AccountAddress,

  /// The account's authkey.
  pub auth_key: AuthenticationKey,

  /// Other nodes to connect for fallback connections
  pub upstream_nodes: Vec<Url>,
}

impl Default for Profile {
  fn default() -> Self {
    Self {
      account: AccountAddress::ZERO,
      auth_key: AuthenticationKey::zero(),
      upstream_nodes: vec!["http://localhost:8080".parse().expect("parse url")],
    }
  }
}
