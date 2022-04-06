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