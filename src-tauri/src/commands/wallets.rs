//! wallets

use aptos_types::transaction::authenticator::AuthenticationKey;
use move_core_types::account_address::AccountAddress;

use crate::configs::default_accounts_db_path;
use crate::keys::{NewKeygen, AccountEntry, Accounts};
use crate::wallet_error::WalletError;
use crate::{configs, configs_profile, keys};

use super::get_events;
use super::{get_balance, EventView};
use std::fs::{self};


#[tauri::command]
pub fn keygen() -> Result<NewKeygen, WalletError> {
  keys::new_random()
}

/// default way accounts get initialized in anima_canary
#[tauri::command(async)]
pub fn is_init() -> Result<bool, WalletError> {
  Ok(configs::is_initialized())
}

/// default way accounts get initialized in anima_canary
#[tauri::command]
pub fn init_from_mnem(mnem: String) -> Result<AccountEntry, WalletError> {
  keys::danger_init_from_mnem(mnem)
}

/// read all accounts from ACCOUNTS_DB_FILE
#[tauri::command(async)]
pub fn get_all_accounts() -> Result<Accounts, WalletError> {
  let all = keys::read_accounts()?;
  Ok(all)
}

#[tauri::command(async)]
pub fn get_account_events(account: String) -> Result<Vec<EventView>, WalletError> {
  let events = get_events(account)?;
  Ok(events)
}

#[tauri::command(async)]
pub fn refresh_accounts() -> Result<Accounts, WalletError> {
  let all = keys::read_accounts()?;
  let updated = map_get_balance(all)?;
  keys::update_accounts_db(&updated)?;
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
fn find_account_data(account: &AccountAddress) -> Result<AccountEntry, WalletError> {
  let all = keys::read_accounts()?;
  match all.accounts.into_iter().find(|a| &a.account == account) {
    Some(entry) => Ok(entry),
    None => Err(WalletError::misc("could not find an account")),
  }
}

/// Add an account (for tracking only).
#[tauri::command]
pub fn add_account(
  nickname: String,
  authkey: AuthenticationKey,
  address: AccountAddress,
) -> Result<Accounts, WalletError> {

  dbg!("add account");
  let mut entry = AccountEntry::new(address, authkey);
  entry.nickname = nickname;

  keys::insert_account_db(entry).map_err(|e| {
    WalletError::misc(&format!(
      "could not add account, message {:?}",
      e.to_string()
    ))
  })
}

/// Switch tx profiles, change anima_canary.toml to use selected account
#[tauri::command(async)]
pub fn switch_profile(account: AccountAddress) -> Result<AccountEntry, WalletError> {
  match find_account_data(&account) {
    Ok(entry) => {
      configs_profile::set_account_profile(&account, entry.authkey.clone())
        .map_err(|_| WalletError::misc("could not switch profile"))?;
      Ok(AccountEntry::new(account, entry.authkey))
    }
    Err(_) => Err(WalletError::misc("could not switch profile")),
  }
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



