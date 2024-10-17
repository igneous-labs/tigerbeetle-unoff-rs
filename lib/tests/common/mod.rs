#[cfg(feature = "live-test")]
pub fn live_test_client() -> tigerbeetle_unoff::Client {
    tigerbeetle_unoff::Client::init(0, "127.0.0.1:3000").unwrap()
}
