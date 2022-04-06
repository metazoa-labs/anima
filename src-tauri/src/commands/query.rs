//! query the chain
use move_core_types::account_address::AccountAddress;

use crate::{wallet_error::WalletError, query, api::*};

#[tauri::command(async)]
pub fn query_balance(account: AccountAddress) -> Result<u64, WalletError> {
  get_balance(&account)
}

// NOTE: change String to Type
pub fn get_balance(_account: &AccountAddress) -> Result<u64, WalletError> {
  // Mock balance
  Ok(100)
}

// NOTE: Create own event view
/// Event view
#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct EventView {
  pub key: String,
  pub sequence_number: u64,
  pub transaction_version: u64,
  pub data: EventData,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct EventData {
  e_type: String,
  amount: u64,
  sender: String,
  receiver: String,
}

fn dummy_event() -> EventView {
  let data = EventData {
    e_type: "receivedpayment".to_string(),
    amount: 10,
    sender: "alice".to_string(),
    receiver: "bob".to_string(),
  };
  EventView {
    key: "".to_string(),
    sequence_number: 100,
    transaction_version: 100,
    data,
  }
}
pub fn get_events(_account: String) -> Result<Vec<EventView>, WalletError> {
  Ok(vec![dummy_event()])
}

#[tauri::command(async)]
pub async fn get_root_account() -> Result<String, String> {
  match query::get_association_state().await {
    Ok(r) => Ok(r.to_string()),
    Err(e) => Err(e.to_string()),
}
  // Ok(r.to_string())
}

// get_root_account