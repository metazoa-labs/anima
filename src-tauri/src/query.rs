//! query
use reqwest::Url;
use crate::api::{API_ENDPOINT, execute_request};

pub async fn get_association_state() -> anyhow::Result<serde_json::Value> {
    let address = "0xA550C18";
    get_account_resources(address).await
}


fn account_resources_path(address: &str) -> String {
    format!("/accounts/{}/resources", address)
}

pub async fn get_account_resources(address: &str) -> anyhow::Result<serde_json::Value> {
  let base: Url = API_ENDPOINT.parse().unwrap();
  let url = base.join(&account_resources_path(address))?;
  let r = reqwest::Client::new();

  let req = r.get(url);

  execute_request(req).await
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
