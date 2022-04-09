//! key management tools, leveraging OS keyrings.

extern crate keyring;
use anyhow::{anyhow, bail};
use aptos_sdk::crypto::ed25519::Ed25519PrivateKey;
use aptos_types::account_address::AccountAddress;
use bip32::secp256k1::ecdsa::{SigningKey, VerifyingKey};
use keyring::KeyringError;

const KEYRING_APP_NAME: &str = "anima_canary";

/// send the encoded private key to OS keyring
pub fn set_private_key(user_address: &AccountAddress, key: Ed25519PrivateKey) -> Result<(), KeyringError> {
  let name = user_address.to_string();
  let kr = keyring::Keyring::new(KEYRING_APP_NAME, &name);

  let bytes: &[u8] = &(key.to_bytes());
  let encoded = hex::encode(bytes);

  kr.set_password(&encoded)
}

/// retrieve a private key from OS keyring
pub fn get_private_key(user_address: &str) -> Result<SigningKey, anyhow::Error> {
  let kr = keyring::Keyring::new(KEYRING_APP_NAME, &user_address);

  match kr.get_password() {
    Ok(s) => {
      let bytes = hex::decode(s)?;
      match SigningKey::from_bytes(bytes.as_slice()) {
        Ok(k) => Ok(k),
        Err(e) => bail!(e),
      }
    }
    Err(e) => Err(anyhow!("{:?}", e)),
  }
}

// retrieve a keypair from OS keyring
pub fn get_keypair(user_address: &str) -> Result<(SigningKey, VerifyingKey), anyhow::Error> {
  match get_private_key(&user_address) {
    Ok(prv) => {
      let pbl = prv.verifying_key();
      Ok((prv, pbl))
    }
    Err(e) => bail!(e),
  }
}
