#![no_main]

use borsh::{BorshDeserialize, BorshSerialize};
use libfuzzer_sys::fuzz_target;
use near_account_id::AccountId;

fuzz_target!(|account_id: AccountId| {
    assert_eq!(
        account_id,
        AccountId::try_from_slice(account_id.try_to_vec().unwrap().as_slice()).unwrap()
    );
});
