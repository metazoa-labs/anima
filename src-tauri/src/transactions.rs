//! build transactions

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

use crate::api::{URL, execute_request};

// Note: examples come from: https://github.com/aptos-labs/aptos-core/blob/70f68edcd1c0056cb082172065320e5ea1e54d15/api/src/tests/transactions_test.rs


pub fn create_user_account_by(
    factory: TransactionFactory,
    creator: &mut LocalAccount,
    public_key: &Ed25519PublicKey,
) -> SignedTransaction {
    creator.sign_with_transaction_builder(
        factory
            .create_user_account(public_key)
            .expiration_timestamp_secs(u64::MAX),
    )
}

async fn test_post_bcs_format_transaction() -> anyhow::Result<serde_json::Value>  {
    // let mut context = new_test_context(current_function_name!());
    // let account = context.gen_account();
    // TODO: Get correct chain ID
    let factory = TransactionFactory::new(ChainId::new(1));

    // create random
    let mut random_sending_account = LocalAccount::generate(&mut OsRng);
    let random_new_account = LocalAccount::generate(&mut OsRng);

    let txn = create_user_account_by(factory, &mut random_sending_account, &random_new_account.public_key());

    let body = bcs::to_bytes(&txn).unwrap();
    post_bcs_txn("/transactions", body)
    .await

}

pub async fn post_bcs_txn(path: &str, body: Vec<u8>) -> anyhow::Result<serde_json::Value> {
    let base: Url = URL.parse().unwrap();
    let url = base.join(path).unwrap();

    let client = reqwest::Client::new();
    let req = client
        .post(url)
        .header(CONTENT_TYPE, mime_types::BCS_SIGNED_TRANSACTION)
        .body(body);

    execute_request(req).await
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
