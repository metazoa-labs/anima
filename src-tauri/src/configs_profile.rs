//! account configurations

use crate::app_cfg::AppCfg;
use crate::configs::{self};
use crate::configs_network::{NetworkID, set_network_configs};
use crate::wallet_error::WalletError;
use aptos_types::transaction::authenticator::AuthenticationKey;
use move_core_types::account_address::AccountAddress;
use std::fs;

/// For switching between profiles in the Account DB.
// NOTE: account and authkey are placeholders!
pub fn set_account_profile(account: &AccountAddress, authkey: AuthenticationKey) -> Result<AppCfg, WalletError> {
  let mut cfg = match configs::is_initialized() {
    true => configs::get_cfg()?,
    false => AppCfg::default(),
  };

  cfg.workspace.node_home = configs::default_config_path().parent().unwrap().to_owned();

  cfg.profile.account = account.to_owned();
  cfg.profile.auth_key = authkey;

  if !cfg.workspace.node_home.exists() {
    fs::create_dir_all(&cfg.workspace.node_home)
    .map_err(|_| WalletError::config("can't create directory"))?;
  }

  cfg.save_file()?;
  // TODO: this is for Canary profile. Make switch to Devnet when appropriate.
  set_network_configs(NetworkID::Devnet)?;

  println!("account profile saved");

  Ok(cfg)
}
