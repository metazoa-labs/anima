//! query
use reqwest::Url;
use aptos_api_types::{LedgerInfo, MoveResource};
use crate::api::api::execute_request;

use super::api::pick_endpoint;

pub async fn get_ledger_info() -> anyhow::Result<LedgerInfo> {

  let c = reqwest::Client::new();
  let req = c.get(pick_endpoint()?);
  let res = execute_request(req).await?;
  Ok(serde_json::from_value(res)?)

}

pub async fn get_association_state() -> anyhow::Result<Vec<MoveResource>> {
    let address = "0xA550C18";
    get_account_resources(address).await
}



pub async fn get_account_resources(address: &str) -> anyhow::Result<Vec<MoveResource>> {
  let base: Url = pick_endpoint()?;
  let url = base.join(&format!("/accounts/{}/resources", address))?;
  let r = reqwest::Client::new();

  let req = r.get(url);

  let vals = execute_request(req).await?;
  let r: Vec<MoveResource> = serde_json::from_value(vals)?;
  Ok(r)
}

#[test]
fn test () {
  let b = tauri::async_runtime::block_on(get_account_resources("0x2924db8ef7c619a5c586325193ed1517e0a72b2f68a0b6cd675bb08c821033bb")).unwrap();

  dbg!(&b);
}

// async fn test_get_transactions_returns_last_page_when_start_version_is_not_specified() {
//     let mut context = new_test_context(current_function_name!());

//     let mut root_account = context.root_account();
//     for _i in 0..20 {
//         let account = context.gen_account();
//         let txn = context.create_user_account_by(&mut root_account, &account);
//         context.commit_block(&vec![txn.clone()]).await;
//     }

//     let resp = context.get("/transactions").await;
//     context.check_golden_output(resp);
// }
