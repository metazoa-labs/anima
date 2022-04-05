//! key management tools, leveraging OS keyrings.

extern crate keyring;
use anyhow::{anyhow, bail};
use bip32::secp256k1::ecdsa::{SigningKey, VerifyingKey};
use keyring::KeyringError;

const KEYRING_APP_NAME: &str = "tauriWallet";

/// send the encoded private key to OS keyring
pub fn set_private_key(ol_address: &str, key: String) -> Result<(), KeyringError> {
  let kr = keyring::Keyring::new(KEYRING_APP_NAME, &ol_address);

  let bytes: &[u8] = &(key.as_bytes());
  let encoded = hex::encode(bytes);

  kr.set_password(&encoded)
}

/// retrieve a private key from OS keyring
pub fn get_private_key(ol_address: &str) -> Result<SigningKey, anyhow::Error> {
  let kr = keyring::Keyring::new(KEYRING_APP_NAME, &ol_address);

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
pub fn get_keypair(ol_address: &str) -> Result<(SigningKey, VerifyingKey), anyhow::Error> {
  match get_private_key(&ol_address) {
    Ok(prv) => {
      let pbl = prv.verifying_key();
      Ok((prv, pbl))
    }
    Err(e) => bail!(e),
  }
}
