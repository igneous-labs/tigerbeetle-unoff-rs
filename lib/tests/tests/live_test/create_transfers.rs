use std::{
    num::{NonZeroU128, NonZeroU16, NonZeroU32},
    time::SystemTime,
};

use tigerbeetle_unoff::{
    data_model::{
        transfer::{
            transfer_to_create, HasCreditAccountId, HasDebitAccountId, HasPendingId, HasTimeout,
            HasTransferAmt, HasTransferFlags, TransferFlags,
        },
        CreateNativeEventTimestamp, EmptyUserData, HasCode, HasId, HasLedger,
    },
    resp::create_transfers::CreateTransfersResp,
    u128_id::U128Id,
};

use crate::common::live_test_client;

fn assert_all_create_transfers_success(res: &CreateTransfersResp) {
    for r in res.iter_results() {
        r.unwrap();
    }
}

const LEDGER: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(700) };

/// Expects running cluster to have:
/// - an account with ID 1 that can debited by 1 on ledger 700
/// - an account with ID 2 that can be credited by 1 on ledger 700
#[tokio::test]
async fn create_transfers_sanity() {
    struct SanityTransferToCreate;

    impl HasId for SanityTransferToCreate {
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

    impl EmptyUserData for SanityTransferToCreate {}

    impl HasLedger for SanityTransferToCreate {
        fn ledger(&self) -> NonZeroU32 {
            LEDGER
        }
    }

    impl HasCode for SanityTransferToCreate {
        fn code(&self) -> NonZeroU16 {
            NonZeroU16::new(1).unwrap()
        }
    }

    impl CreateNativeEventTimestamp for SanityTransferToCreate {}

    impl HasTransferFlags for SanityTransferToCreate {
        fn transfer_flags(&self) -> TransferFlags {
            TransferFlags::NONE
        }
    }

    impl HasDebitAccountId for SanityTransferToCreate {
        fn debit_account_id(&self) -> U128Id {
            U128Id::new(1).unwrap()
        }
    }

    impl HasCreditAccountId for SanityTransferToCreate {
        fn credit_account_id(&self) -> U128Id {
            U128Id::new(2).unwrap()
        }
    }

    impl HasTransferAmt for SanityTransferToCreate {
        fn transfer_amt(&self) -> NonZeroU128 {
            NonZeroU128::new(1).unwrap()
        }
    }

    impl HasTimeout for SanityTransferToCreate {
        fn timeout(&self) -> u32 {
            0
        }
    }

    impl HasPendingId for SanityTransferToCreate {
        fn pending_id(&self) -> u128 {
            0
        }
    }

    let c = live_test_client();
    let res = c
        .create_transfers(&[transfer_to_create(SanityTransferToCreate)])
        .await
        .unwrap();
    assert_all_create_transfers_success(&res);
}
