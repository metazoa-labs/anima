//! account configurations

use crate::app_cfg::AppCfg;
use crate::configs::{self};
use anyhow::Error;
use aptos_types::transaction::authenticator::AuthenticationKey;
use move_core_types::account_address::AccountAddress;
use std::fs;

/// For switching between profiles in the Account DB.
// NOTE: account and authkey are placeholders!
pub fn set_account_profile(account: &AccountAddress, authkey: AuthenticationKey) -> Result<AppCfg, Error> {
  let mut cfg = match configs::is_initialized() {
    true => configs::get_cfg()?,
    false => AppCfg::default(),
  };

  cfg.workspace.node_home = configs::default_config_path().parent().unwrap().to_owned();

  cfg.profile.account = account.to_owned();
  cfg.profile.auth_key = authkey;

  if !cfg.workspace.node_home.exists() {
    fs::create_dir_all(&cfg.workspace.node_home)?;
  }

  println!("set_account_profile");

  cfg.save_file()?;
  Ok(cfg)
}
