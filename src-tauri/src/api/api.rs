//! interfaces for aptos API
use anyhow::bail;
use aptos_api_types::*;
use reqwest::Url;
use reqwest::{header::CONTENT_TYPE};
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::configs;
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ApiSchema {
  r#type: String,
  data: Schemas
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]

pub enum Schemas {
  Coin{value: U64}
}
pub async fn execute_request(req: reqwest::RequestBuilder) -> anyhow::Result<serde_json::Value> {
    let resp = req.send().await?;

    let headers = resp.headers();
    assert_eq!(headers[CONTENT_TYPE], mime_types::JSON);

    let s = resp.text().await?;
    serde_json::from_str(&s).map_err(|e| anyhow::anyhow!(e.to_string()))
}


/// get a working endpoint from the list of upstream peers.
// TODO: endpoints need to be TESTED to check for ledger info once selected.

pub fn pick_endpoint() -> anyhow::Result<Url> {
  let cfg = configs::get_cfg()?;
  let mut endpoints = cfg.profile.upstream_nodes;
  endpoints.shuffle(&mut thread_rng());
  match endpoints.first() {
    Some(u) => Ok(u.to_owned()),
    None => bail!("could not find endpoint url in list")
  }
}
