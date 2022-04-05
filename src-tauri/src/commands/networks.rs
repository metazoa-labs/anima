//! networks to connect to

use crate::{
  configs_network::{
    override_upstream_node, set_network_configs, set_waypoint, set_waypoint_from_upstream,
    NetworkProfile, Networks,
  },
  wallet_error::WalletError,
};
use url::Url;

#[tauri::command]
pub fn toggle_network(network: Networks) -> Result<NetworkProfile, WalletError> {
  set_network_configs(network)
}

#[tauri::command(async)]
pub fn get_networks() -> Result<NetworkProfile, WalletError> {
  NetworkProfile::new()
}

#[tauri::command]
pub fn override_playlist(playlist_url: Url) -> Result<NetworkProfile, WalletError> {
  set_network_configs(Networks::Custom { playlist_url })
}

#[tauri::command]
pub fn force_upstream(url: Url) -> Result<NetworkProfile, WalletError> {
  override_upstream_node(url).map_err(|e| WalletError::misc(&e.to_string()))?;
  NetworkProfile::new()
}

#[tauri::command]
// TODO: use proper type for WP
pub fn force_waypoint(wp: String) -> Result<NetworkProfile, WalletError> {
  set_waypoint(wp).map_err(|e| WalletError::misc(&e.to_string()))?;
  NetworkProfile::new()
}

#[tauri::command]
pub fn refresh_waypoint() -> Result<NetworkProfile, WalletError> {
  set_waypoint_from_upstream()?;
  NetworkProfile::new()
}
