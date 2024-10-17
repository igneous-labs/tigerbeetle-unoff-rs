use crate::common::live_test_client;

/// Expects running cluster to have:
/// - a transfer with ID 1
#[tokio::test]
async fn lookup_transfers_sanity() {
    let c = live_test_client();
    let trfs = c.lookup_transfers(&[1]).await.unwrap();
    eprintln!("{:#?}", trfs.as_slice());
}
