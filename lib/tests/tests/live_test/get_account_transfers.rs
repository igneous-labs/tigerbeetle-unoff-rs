use tigerbeetle_unoff::{data_model::account_filter::AccountFilter, u128_id::U128Id};

use crate::common::live_test_client;

/// Expects running cluster to have:
/// - an account with ID 1 that has some transfers ran
#[tokio::test]
async fn get_account_transfers_sanity() {
    let c = live_test_client();
    let res = c
        .get_account_transfers(
            &AccountFilter::new_default(U128Id::new(1).unwrap()).to_account_filter_t(),
        )
        .await
        .unwrap();
    eprintln!("{:#?}", res.as_slice());
}
