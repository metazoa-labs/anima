//! keygen

use aptos_types::transaction::authenticator::AuthenticationKey;
use bip32::{Mnemonic, Prefix, XPrv};
use rand_core::OsRng;
use crate::wallet_error::WalletError;
use crate::configs::default_accounts_db_path;
use crate::{configs, configs_network, configs_profile, key_manager};
use anyhow::{bail, Error};
use move_core_types::account_address::AccountAddress;

use std::fs::{create_dir_all, File};
use std::io::prelude::*;


#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Accounts {
  pub accounts: Vec<AccountEntry>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct AccountEntry {
  pub account: AccountAddress,
  pub authkey: AuthenticationKey,
  pub nickname: String,
  pub on_chain: bool,
  pub balance: Option<u64>,
}

impl AccountEntry {
  // NOTE: You don't neeed to use String here if your keys have Serde serialization. This is a placeholder.
  pub fn new(address: AccountAddress, authkey: AuthenticationKey) -> Self {
    AccountEntry {
      account: address,
      authkey,
      nickname: get_short(&address),
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


pub fn new_random() -> Result<NewKeygen, WalletError> {

    // Generate random Mnemonic using the default language (English)
  let mnemonic = Mnemonic::random(&mut OsRng, Default::default());

  // Derive a BIP39 seed value using the given password

  let seed = mnemonic.to_seed("password");
  
  let bytes: [u8; 32] = seed.as_bytes()[0..32].try_into()
    .map_err(|_|{ WalletError::misc("keygen")})?;
  
  // TODO: Aptos uses a different size seed.
  let mut keys = aptos_keygen::KeyGen::from_seed(bytes);

  let pair = keys.generate_keypair();
  let authkey = AuthenticationKey::ed25519(&pair.1);

  let creds = keys.generate_credentials_for_account_creation();


  let res = NewKeygen {
    entry: AccountEntry::new(
      creds.2,
      authkey
    ),
    mnem: mnemonic.phrase().to_owned(),
  };

  Ok(res)
}


pub fn danger_init_from_mnem(mnem: String) -> Result<AccountEntry, WalletError> {
  dbg!("init from mnem");
  // Generate random Mnemonic using the default language (English)
  let mnemonic = Mnemonic::new(mnem, Default::default())
  .map_err(|_|{ WalletError::misc("cannot derive keys from mnemonic")})?;

  // Derive a BIP39 seed value using the given password
  let seed = mnemonic.to_seed("password");
  
  let bytes: [u8; 32] = seed.as_bytes()[0..32].try_into()
    .map_err(|_|{ WalletError::misc("keygen")})?;
  
  // TODO: Aptos uses a different size seed.
  let mut keys = aptos_keygen::KeyGen::from_seed(bytes);

  let pair = keys.generate_keypair();
  let authkey = AuthenticationKey::ed25519(&pair.1);

  let creds = keys.generate_credentials_for_account_creation();


  // let res = NewKeygen {
  //   entry: AccountEntry::new(
  //     creds.2,
  //     authkey
  //   ),
  //   mnem: mnemonic.phrase().to_owned(),
  // };

  Ok(AccountEntry::new(creds.2, authkey))
}

/// insert into accounts file
pub fn insert_account_db(
  nickname: String,
  address: AccountAddress, // TODO: Change this to the actual type
  authkey: AuthenticationKey, // TODO: Change this to the actual type
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

/// update accounts file
pub fn update_accounts_db(accounts: &Accounts) -> Result<(), WalletError> {
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

/// read accounts file
pub fn read_accounts() -> Result<Accounts, Error> {
  let db_path = default_accounts_db_path();
  if db_path.exists() {
    let file = File::open(db_path)?;
    Ok(serde_json::from_reader(file)?)
  } else {
    Ok(Accounts { accounts: vec![] })
  }
}

fn get_short(acc: &AccountAddress) -> String {
  acc.to_string()[..3].to_owned()
}