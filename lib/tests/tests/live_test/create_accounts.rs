use std::{
    num::{NonZeroU16, NonZeroU32},
    time::SystemTime,
};

use tigerbeetle_unoff::{
    data_model::{
        account::{account_to_create, AccountFlags, HasAccountFlags},
        CreateNativeEventTimestamp, EmptyUserData, HasCode, HasId, HasLedger,
    },
    err::CreateAccountErr,
    resp::create_accounts::CreateAccountsResp,
    u128_id::U128Id,
};

use crate::common::live_test_client;

fn assert_all_create_accounts_success(res: &CreateAccountsResp) {
    for r in res.iter_results() {
        r.unwrap();
    }
}

/// Creates an empty account with ID=current_timestamp
#[tokio::test]
async fn create_accounts_sanity() {
    struct SanityAccountToCreate;

    impl HasId for SanityAccountToCreate {
        fn id(&self) -> U128Id {
            U128Id::new(u128::from(
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            ))
            .unwrap()
        }
    }

    impl EmptyUserData for SanityAccountToCreate {}

    impl HasLedger for SanityAccountToCreate {
        fn ledger(&self) -> NonZeroU32 {
            unsafe { NonZeroU32::new_unchecked(1) }
        }
    }

    impl HasCode for SanityAccountToCreate {
        fn code(&self) -> NonZeroU16 {
            unsafe { NonZeroU16::new_unchecked(1) }
        }
    }

    impl CreateNativeEventTimestamp for SanityAccountToCreate {}

    impl HasAccountFlags for SanityAccountToCreate {
        fn account_flags(&self) -> AccountFlags {
            AccountFlags::NONE
        }
    }

    let c = live_test_client();
    let res = c
        .create_accounts(&[account_to_create(SanityAccountToCreate)])
        .await
        .unwrap();
    assert_all_create_accounts_success(&res);
}

/// Expects running cluster to have:
/// - an account with ID 1 with different flags
#[tokio::test]
async fn create_accounts_fail_already_exists_with_different_flags() {
    struct CreateAccountFailAlreadyExists;

    impl HasId for CreateAccountFailAlreadyExists {
        fn id(&self) -> U128Id {
            unsafe { U128Id::new_unchecked(1) }
        }
    }

    impl EmptyUserData for CreateAccountFailAlreadyExists {}

    impl HasLedger for CreateAccountFailAlreadyExists {
        fn ledger(&self) -> NonZeroU32 {
            unsafe { NonZeroU32::new_unchecked(1) }
        }
    }

    impl HasCode for CreateAccountFailAlreadyExists {
        fn code(&self) -> NonZeroU16 {
            unsafe { NonZeroU16::new_unchecked(1) }
        }
    }

    impl CreateNativeEventTimestamp for CreateAccountFailAlreadyExists {}

    impl HasAccountFlags for CreateAccountFailAlreadyExists {
        fn account_flags(&self) -> AccountFlags {
            AccountFlags::NONE
        }
    }

    let c = live_test_client();
    let res = c
        .create_accounts(&[account_to_create(CreateAccountFailAlreadyExists)])
        .await
        .unwrap();
    let res = res.iter_results().next().unwrap();
    assert!(matches!(
        res.unwrap_err(),
        (0, CreateAccountErr::ExistsWithDifferentFlags)
    ));
}
