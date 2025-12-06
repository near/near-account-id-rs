#![no_main]

use libfuzzer_sys::fuzz_target;
use near_account_id::AccountId;

fuzz_target!(|account_id: AccountId| {
    assert_eq!(
        account_id,
        serde_json::from_value::<AccountId>(serde_json::to_value(&account_id).unwrap()).unwrap()
    );
});
