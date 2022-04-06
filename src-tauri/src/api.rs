//! interfaces for aptos API
use aptos_api_types::*;
use aptos_sdk::{
    crypto::ed25519::Ed25519PublicKey,
    transaction_builder::TransactionFactory,
    types::LocalAccount,
};
use aptos_types::{transaction::SignedTransaction};
use reqwest::{header::CONTENT_TYPE, Url};
use rand_core::OsRng;
use aptos_types::chain_id::ChainId;

pub const URL: &str = "http:/0.0.0.0:8080";


// URL

// request handler

// query chain state

// query account state

// submit transaction

// sign a transaction

pub async fn execute_request(req: reqwest::RequestBuilder) -> anyhow::Result<serde_json::Value> {
    // let resp = self.reply(req).await;
    // req.reply(f).a
    // resp = reqwest::blocking::RequestBuilder::
    let resp = req.send().await.unwrap();

    let headers = resp.headers();
    assert_eq!(headers[CONTENT_TYPE], mime_types::JSON);

    let s = resp.text().await.unwrap();
    serde_json::from_str(&s).map_err(|e| anyhow::anyhow!(e.to_string()))
}
