use crate::common::live_test_client;

/// Expects running cluster to have:
/// - an account with ID 1
/// - an account with ID 2
#[tokio::test]
async fn lookup_accounts_sanity() {
    let c = live_test_client();
    let accs = c.lookup_accounts(&[1]).await.unwrap();
    eprintln!("{:#?}", accs.as_slice());
    let accs = c.lookup_accounts(&[1, 2]).await.unwrap();
    eprintln!("{:#?}", accs.as_slice());
}
