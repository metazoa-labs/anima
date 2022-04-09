//! WalletError error type for client

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum ErrorCat {
  Client,
  Tx,
  Configs,
  Misc,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct WalletError {
  ///
  pub category: ErrorCat,
  ///
  pub uid: u64,
  ///
  pub msg: String,
  ///
  pub trace: String,
}

impl From<anyhow::Error> for WalletError {
  fn from(e: anyhow::Error) -> Self {
    WalletError::misc(&format!("misc error, message: {:?}", e.to_string()))
  }
}

// TODO: have an opinion about error codes.
pub const E_UNKNOWN: u64 = 100;

pub const E_APP_CONFIG: u64 = 103;

// Client Errors
pub const E_CLIENT_CX: u64 = 404;

impl WalletError {
  pub fn new(category: ErrorCat, uid: u64, msg: String, trace: String) -> Self {
    WalletError {
      category,
      uid,
      msg,
      trace,
    }
  }

  pub fn config(msg: &str) -> Self {
    WalletError {
      category: ErrorCat::Configs,
      uid: E_APP_CONFIG,
      msg: msg.to_owned(),
      trace: msg.to_owned(),
    }
  }

  pub fn tx(msg: &str) -> Self {
    WalletError {
      category: ErrorCat::Tx,
      uid: E_UNKNOWN,
      msg: msg.to_owned(),
      trace: msg.to_owned(),
    }
  }

  pub fn client(msg: &str) -> Self {
    WalletError {
      category: ErrorCat::Client,
      uid: E_UNKNOWN,
      msg: msg.to_owned(),
      trace: msg.to_owned(),
    }
  }

  pub fn rpc_fail(msg: &str) -> Self {
    WalletError {
      category: ErrorCat::Client,
      uid: E_CLIENT_CX,
      msg: "Network Unreacheable".to_owned(),
      trace: msg.to_owned(),
    }
  }

  pub fn misc(msg: &str) -> Self {
    WalletError {
      category: ErrorCat::Misc,
      uid: E_UNKNOWN,
      msg: msg.to_owned(),
      trace: msg.to_owned(),
    }
  }
}
