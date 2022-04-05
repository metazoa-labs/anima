//! wallets

use crate::configs::default_accounts_db_path;
use crate::wallet_error::WalletError;
use crate::{configs, configs_network, configs_profile, key_manager};
use anyhow::{bail, Error};
use super::get_events;
use super::{get_balance, EventView};
use bip32::{Mnemonic, Prefix, XPrv};
use rand_core::OsRng;
use std::fs::{self, create_dir_all, File};
use std::io::prelude::*;
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Accounts {
  pub accounts: Vec<AccountEntry>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct AccountEntry {
  pub account: String,
  pub authkey: String,
  pub nickname: String,
  pub on_chain: bool,
  pub balance: Option<u64>,
}

impl AccountEntry {
  // NOTE: You don't neeed to use String here if your keys have Serde serialization. This is a placeholder.
  pub fn new(address: String, authkey: String) -> Self {
    AccountEntry {
      account: address.clone(),
      authkey,
      nickname: get_short(address),
      on_chain: false,
      balance: None,
    }
  }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct NewKeygen {
  entry: AccountEntry,
  mnem: String,
}

#[tauri::command]
pub fn keygen() -> Result<NewKeygen, WalletError> {
  dbg!("keygen");

  // Generate random Mnemonic using the default language (English)
  let mnemonic = Mnemonic::random(&mut OsRng, Default::default());

  // Derive a BIP39 seed value using the given password
  let seed = mnemonic.to_seed("password");

  // Derive a child `XPrv` using the provided BIP32 derivation path
  let child_path = "m/0/2147483647'/1/2147483646'";
  let child_xprv = XPrv::derive_from_path(&seed, &child_path.parse().unwrap()).unwrap();

  // Get the `XPub` associated with `child_xprv`.
  let child_xpub = child_xprv.public_key();

  let res = NewKeygen {
    entry: AccountEntry::new(
      child_xpub.to_string(Prefix::XPUB),
      child_xpub.to_string(Prefix::XPUB),
    ),
    mnem: mnemonic.phrase().to_owned(),
  };

  Ok(res)
}

/// default way accounts get initialized in TauriWallet
#[tauri::command(async)]
pub fn is_init() -> Result<bool, WalletError> {
  Ok(configs::is_initialized())
}

/// default way accounts get initialized in TauriWallet
#[tauri::command]
pub fn init_from_mnem(mnem: String) -> Result<AccountEntry, WalletError> {
  danger_init_from_mnem(mnem).map_err(|_| WalletError::config("could not initialize from mnemonic"))
}

pub fn danger_init_from_mnem(_mnem: String) -> Result<AccountEntry, WalletError> {
  dbg!("init from mnem");
  let init = configs::is_initialized();

  // Generate random Mnemonic using the default language (English)
  let mnemonic = Mnemonic::random(&mut OsRng, Default::default());

  // Derive a BIP39 seed value using the given password
  let seed = mnemonic.to_seed("password");

  ///// DEMO ONLY!  /////
  // Derive the root `XPrv` from the `seed` value
  let _root_xprv = XPrv::new(&seed).unwrap();
  // Derive a child `XPrv` using the provided BIP32 derivation path
  let child_path = "m/0/2147483647'/1/2147483646'";
  let child_xprv = XPrv::derive_from_path(&seed, &child_path.parse().unwrap()).unwrap();

  // // Get the `XPub` associated with `child_xprv`.
  let child_xpub = child_xprv.public_key();

  // NOTE: DEMO ONLY!
  let address = child_xpub.to_string(Prefix::XPUB);
  let priv_key = child_xprv.to_string(Prefix::XPUB).as_str().to_owned();

  let authkey = child_xpub.to_string(Prefix::XPUB);

  // first try to insert into DB.
  // it will error if the account already exists.
  insert_account_db(get_short(address.clone()), address.clone(), authkey.clone())?;

  key_manager::set_private_key(&address.clone().to_string(), priv_key)
    .map_err(|e| WalletError::config(&e.to_string()))?;

  configs_profile::set_account_profile(address.clone(), authkey.clone())?;

  // this may be the first account and may not yet be initialized.
  if !init {
    configs_network::set_network_configs(configs_network::Networks::Mainnet)?;
  }

  Ok(AccountEntry::new(address, authkey))
}

/// read all accounts from ACCOUNTS_DB_FILE
#[tauri::command(async)]
pub fn get_all_accounts() -> Result<Accounts, WalletError> {
  let all = read_accounts()?;
  Ok(all)
}

#[tauri::command(async)]
pub fn get_account_events(account: String) -> Result<Vec<EventView>, WalletError> {
  let events = get_events(account)?;
  Ok(events)
}

#[tauri::command(async)]
pub fn refresh_accounts() -> Result<Accounts, WalletError> {
  let all = read_accounts()?;
  let updated = map_get_balance(all)?;
  update_accounts_db(&updated)?;
  Ok(updated)
}

fn map_get_balance(mut all_accounts: Accounts) -> Result<Accounts, WalletError> {
  all_accounts.accounts = all_accounts
    .accounts
    .into_iter()
    .map(|mut e| {
      e.balance = get_balance(&e.account).ok();
      e.on_chain = e.balance.is_some();
      e
    })
    .collect();
  Ok(all_accounts)
}

/// TODO: use actual type for account
fn find_account_data(account: &str) -> Result<AccountEntry, WalletError> {
  let all = read_accounts()?;
  match all.accounts.into_iter().find(|a| a.account == account) {
    Some(entry) => Ok(entry),
    None => Err(WalletError::misc("could not find an account")),
  }
}

/// Add an account (for tracking only).
#[tauri::command]
pub fn add_account(
  nickname: String,
  authkey: String,
  address: String,
) -> Result<Accounts, WalletError> {

  insert_account_db(nickname, address, authkey).map_err(|e| {
    WalletError::misc(&format!(
      "could not add account, message {:?}",
      e.to_string()
    ))
  })
}

/// Switch tx profiles, change tauriWallet.toml to use selected account
#[tauri::command(async)]
pub fn switch_profile(account: String) -> Result<AccountEntry, WalletError> {
  match find_account_data(&account) {
    Ok(entry) => {
      configs_profile::set_account_profile(account.clone(), entry.authkey.clone())
        .map_err(|_| WalletError::misc("could not switch profile"))?;
      Ok(AccountEntry::new(account, entry.authkey))
    }
    Err(_) => Err(WalletError::misc("could not switch profile")),
  }
}

fn insert_account_db(
  nickname: String,
  address: String, // TODO: Change this to the actual type
  authkey: String, // TODO: Change this to the actual type
) -> Result<Accounts, Error> {
  let app_dir = default_accounts_db_path();
  // get all accounts
  let mut all = read_accounts()?;

  // push new account
  let new_account = AccountEntry {
    account: address,
    authkey: authkey,
    nickname: nickname,
    on_chain: false,
    balance: None,
  };

  if !all.accounts.contains(&new_account) {
    all.accounts.push(new_account);

    // write to db file
    // in case it doesn't exist
    //TODO: remove this.
    create_dir_all(&app_dir.parent().unwrap()).unwrap();
    let serialized = serde_json::to_vec(&all).expect("Struct Accounts should be converted!");
    let mut file = File::create(app_dir).expect("DB_FILE should be created!");
    file
      .write_all(&serialized)
      .expect("DB_FILE should be writen!");

    Ok(all)
  } else {
    bail!("account already exists")
  }
}

fn update_accounts_db(accounts: &Accounts) -> Result<(), WalletError> {
  let app_dir = default_accounts_db_path();
  let serialized = serde_json::to_vec(accounts)
    .map_err(|e| WalletError::config(&format!("json account db should serialize, {:?}", &e)))?;

  File::create(app_dir)
    .map_err(|e| WalletError::config(&format!("tauriWallet DB_FILE should be created!, {:?}", &e)))?
    .write_all(&serialized)
    .map_err(|e| {
      WalletError::config(&format!("tauriWallet DB_FILE should be written!, {:?}", &e))
    })?;
  Ok(())
}

// remove all accounts which are being tracked.
#[tauri::command]
pub fn remove_accounts() -> Result<String, WalletError> {
  // Note: this only removes the account tracking, doesn't delete account on chain.

  let db_path = default_accounts_db_path();
  dbg!(&db_path);
  if db_path.exists() {
    match fs::remove_file(&db_path) {
      Ok(_) => return Ok("removed all accounts".to_owned()),
      _ => {
        return Err(WalletError::misc(&format!(
          "unable to delete account file found at {:?}",
          &db_path
        )))
      }
    }
  }
  return Err(WalletError::misc(
    &format!(
      "No accounts to remove. No account file found at {:?}",
      &db_path
    )
    .to_owned(),
  ));
}

fn read_accounts() -> Result<Accounts, Error> {
  let db_path = default_accounts_db_path();
  if db_path.exists() {
    let file = File::open(db_path)?;
    Ok(serde_json::from_reader(file)?)
  } else {
    Ok(Accounts { accounts: vec![] })
  }
}

fn get_short(acc: String) -> String {
  acc[..3].to_owned()
}
