use crate::app_cfg::APP_HOME;
use crate::wallet_error::WalletError;
use anyhow::Error;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

const PREFERENCES_DB_FILE: &str = "preferences.json";

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Preferences {
  pub locale: Option<String>,
}

/*
  get preferences
*/
#[tauri::command]
pub fn get_preferences() -> Result<Preferences, WalletError> {
  let preferences = read_preferences()?;
  Ok(preferences)
}

/*
  set preferences
*/
#[tauri::command(async)]
pub fn set_preferences_locale(locale: String) -> Result<(), WalletError> {
  let mut preferences = read_preferences()?;
  preferences.locale = Some(locale);
  update_preferences(&preferences)
}

fn read_preferences() -> Result<Preferences, Error> {
  let db_path = preferences_db_path();
  match db_path.exists() {
    true => {
      let file = File::open(db_path)?;
      Ok(serde_json::from_reader(file)?)
    }
    false => Ok(Preferences { locale: None }),
  }
}

fn preferences_db_path() -> PathBuf {
  dirs::home_dir()
    .unwrap()
    .join(APP_HOME)
    .join(PREFERENCES_DB_FILE)
}

fn update_preferences(preferences: &Preferences) -> Result<(), WalletError> {
  let db_path = preferences_db_path();
  let serialized = serde_json::to_vec(preferences)
    .map_err(|e| WalletError::config(&format!("json preferences db should serialize, {:?}", &e)))?;

  File::create(db_path)
    .map_err(|e| {
      WalletError::config(&format!(
        "app preferences_db_file should be created!, {:?}",
        &e
      ))
    })?
    .write_all(&serialized)
    .map_err(|e| {
      WalletError::config(&format!(
        "app preferences_db_file should be written!, {:?}",
        &e
      ))
    })?;
  Ok(())
}

#[tauri::command(async)]
pub fn set_env(env: String) -> Result<String, WalletError> {
  match env.as_ref() {
    "test" => env::set_var("NODE_ENV", "test"),
    "prod" => env::set_var("NODE_ENV", "prod"),
    _ => {}
  }

  let v = env::var("NODE_ENV")
    .map_err(|_| WalletError::misc("environment variable NODE_ENV is not set"))?;
  Ok(v)
}

#[tauri::command(async)]
pub fn get_env() -> Result<String, WalletError> {
  let v = env::var("NODE_ENV").unwrap_or("prod".to_owned());
  Ok(v)
}
