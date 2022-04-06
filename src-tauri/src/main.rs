#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate url;

pub mod app_cfg;
pub mod commands;
pub mod configs;
pub mod configs_network;
pub mod configs_profile;
pub mod key_manager;
pub mod rpc_playlist;
pub mod wallet_error;
pub mod mnemonic;
pub mod keys;
pub mod transactions;

use crate::commands::*;
use tauri::{Menu, MenuItem, Submenu};

fn main() {
  //  println!("{}", version::version());
  // example menu https://github.com/probablykasper/mr-tagger/blob/b40fa319055d83b57f8ce59e82a14c0863f256ac/src-tauri/src/main.rs#L28-L78

  //////// TO FORCE TEST SETTINGS ON START ////////////////////
  // uncomment below to explicitly set "test" env
  // Tauri builder does not take env variable from terminal
  // set_env("test".to_owned()).unwrap();
  //////////////////////////////////////////////////////////

  let menu = Menu::new()
    .add_submenu(Submenu::new(
      "TauriWallet",
      Menu::new()
        // .add_native_item(MenuItem::About("TauriWallet".to_string()))
        .add_native_item(MenuItem::Quit),
    ))
    .add_submenu(Submenu::new("Edit", {
      let mut menu = Menu::new();
      menu = menu.add_native_item(MenuItem::Undo);
      menu = menu.add_native_item(MenuItem::Redo);
      menu = menu.add_native_item(MenuItem::Separator);
      menu = menu.add_native_item(MenuItem::Cut);
      menu = menu.add_native_item(MenuItem::Copy);
      menu = menu.add_native_item(MenuItem::Paste);
      #[cfg(not(target_os = "macos"))]
      {
        menu = menu.add_native_item(MenuItem::Separator);
      }
      menu = menu.add_native_item(MenuItem::SelectAll);
      menu
    }));

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      // Accounts
      is_init,
      refresh_accounts,
      get_all_accounts,
      get_account_events,
      add_account,
      keygen,
      init_from_mnem,
      remove_accounts,
      switch_profile,
      // Networks
      force_upstream,
      force_waypoint,
      override_playlist,
      get_networks,
      refresh_waypoint,
      toggle_network,
      // Queries
      query_balance,
      // Transactions

      get_env,
      set_env,
      // Version
      get_app_version,
      // Preferences
      get_preferences,
      set_preferences_locale
    ])
    .menu(menu)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
