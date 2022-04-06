//! build transactions

use aptos_sdk::{
    crypto::ed25519::Ed25519PublicKey, transaction_builder::TransactionFactory, types::LocalAccount,
};
use aptos_types::transaction::SignedTransaction;
use reqwest::{header::CONTENT_TYPE, Url};
use aptos_api_types::*;


pub const URL: &str = "http:/0.0.0.0:8080";

pub fn create_user_account_by(
    factory: TransactionFactory,
    creator: &mut LocalAccount,
    public_key: &Ed25519PublicKey,
) -> SignedTransaction {
    // let factory = self.transaction_factory();
    creator.sign_with_transaction_builder(
        factory
            .create_user_account(public_key)
            .expiration_timestamp_secs(u64::MAX),
    )
}

// async fn test_post_bcs_format_transaction() {
//     // let mut context = new_test_context(current_function_name!());
//     // let account = context.gen_account();
//     let txn = create_user_account(factory, &sending_account, &new_account);
//     let body = bcs::to_bytes(&txn).unwrap();
//     let resp = context
//         .expect_status_code(202)
//         .post_bcs_txn("/transactions", body)
//         .await;
//     context.check_golden_output(resp.clone());

//     // ensure ed25519 sig txn can be submitted into mempool by JSON format
//     context
//         .expect_status_code(202)
//         .post("/transactions", resp)
//         .await;
// }

pub async fn post_bcs_txn(path: &str, body: Vec<u8>) -> anyhow::Result<serde_json::Value> {
    // let r = reqwest::blocking::Request::new("POST".into(), URL);
    let base: Url = URL.parse().unwrap();
    let url = base.join(path).unwrap();
    // let r = reqwest::Request::new(Method::POST, url )

    let client = reqwest::Client::new();
    let req = client.post(url)
    .header(CONTENT_TYPE, mime_types::BCS_SIGNED_TRANSACTION)
    .body(body);

    execute_request(req).await
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

        // if !(resp.status() < 300) {
        //     let ledger_info = self.get_latest_ledger_info();
        //     assert_eq!(headers[X_APTOS_CHAIN_ID], "4");
        //     assert_eq!(
        //         headers[X_APTOS_LEDGER_VERSION],
        //         ledger_info.version().to_string()
        //     );
        //     assert_eq!(
        //         headers[X_APTOS_LEDGER_TIMESTAMP],
        //         ledger_info.timestamp().to_string()
        //     );
        // }

        // assert_eq!(
        //     202,
        //     resp.status(),
        //     "\nresponse: {}",
        //     pretty(&body)
        // );

        // if self.expect_status_code < 300 {
        //     let ledger_info = self.get_latest_ledger_info();
        //     assert_eq!(headers[X_APTOS_CHAIN_ID], "4");
        //     assert_eq!(
        //         headers[X_APTOS_LEDGER_VERSION],
        //         ledger_info.version().to_string()
        //     );
        //     assert_eq!(
        //         headers[X_APTOS_LEDGER_TIMESTAMP],
        //         ledger_info.timestamp().to_string()
        //     );
        // }

        // body
    }

// async fn test_signing_message_with_payload(
//     mut context: TestContext,
//     txn: SignedTransaction,
//     payload: serde_json::Value,
// ) {
//     let sender = context.root_account();
//     let mut body = json!({
//         "sender": sender.address().to_hex_literal(),
//         "sequence_number": sender.sequence_number().to_string(),
//         "gas_unit_price": txn.gas_unit_price().to_string(),
//         "max_gas_amount": txn.max_gas_amount().to_string(),
//         "gas_currency_code": txn.gas_currency_code(),
//         "expiration_timestamp_secs": txn.expiration_timestamp_secs().to_string(),
//         "payload": payload,
//     });

//     let resp = context
//         .post("/transactions/signing_message", body.clone())
//         .await;

//     let signing_msg = resp["message"].as_str().unwrap();
//     assert_eq!(
//         signing_msg,
//         format!(
//             "0x{}",
//             hex::encode(&txn.clone().into_raw_transaction().signing_message())
//         )
//     );

//     let hex_bytes: HexEncodedBytes = signing_msg.parse().unwrap();
//     let sig = context
//         .root_account()
//         .private_key()
//         .sign_arbitrary_message(hex_bytes.inner());
//     let expected_sig = match txn.authenticator() {
//         TransactionAuthenticator::Ed25519 {
//             public_key: _,
//             signature,
//         } => signature,
//         _ => panic!("expect TransactionAuthenticator::Ed25519"),
//     };
//     assert_eq!(sig, expected_sig);

//     // assert transaction can be submitted into mempool and execute.
//     body["signature"] = json!({
//         "type": "ed25519_signature",
//         "public_key": format!("0x{}", hex::encode(sender.public_key().to_bytes())),
//         "signature": format!("0x{}", hex::encode(sig.to_bytes())),
//     });

//     context
//         .expect_status_code(202)
//         .post("/transactions", body)
//         .await;

//     context.commit_mempool_txns(10).await;

//     let ledger = context.get("/").await;
//     assert_eq!(ledger["ledger_version"].as_str().unwrap(), "2"); // one metadata + one txn
// }

// #[tokio::test]
// async fn test_signing_message_with_script_function_payload() {
//     let mut context = new_test_context(current_function_name!());
//     let account = context.gen_account();
//     let txn = context.create_user_account(&account);

//     let payload = json!({
//         "type": "script_function_payload",
//         "function": "0x1::AptosAccount::create_account",
//         "type_arguments": [
//         ],
//         "arguments": [
//             account.address().to_hex_literal(), // new_account_address
//         ]
//     });
//     test_signing_message_with_payload(context, txn, payload).await;
// }
