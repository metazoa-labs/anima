//! interfaces for aptos API
use aptos_api_types::*;


use reqwest::{header::CONTENT_TYPE};

pub const API_ENDPOINT: &str = "http:/0.0.0.0:8080";


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
    // let resp = self.reply(req).await;
    // req.reply(f).a
    // resp = reqwest::blocking::RequestBuilder::
    let resp = req.send().await.unwrap();

    let headers = resp.headers();
    assert_eq!(headers[CONTENT_TYPE], mime_types::JSON);

    let s = resp.text().await.unwrap();
    serde_json::from_str(&s).map_err(|e| anyhow::anyhow!(e.to_string()))
}


// We want to get the JSON object back into a type.
fn _todo_deserialize_response(val: serde_json::Value) {
  let s: Vec<ApiSchema> = serde_json::from_value(val).unwrap();
  dbg!(&s);
}