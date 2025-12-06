#![no_main]

use borsh::BorshDeserialize;
use libfuzzer_sys::fuzz_target;
use near_account_id::AccountId;

fuzz_target!(|account_id: AccountId| {
    assert_eq!(
        account_id,
        AccountId::try_from_slice(borsh::to_vec(&account_id).unwrap().as_slice()).unwrap()
    );
});
