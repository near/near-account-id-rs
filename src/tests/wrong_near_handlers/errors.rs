use near_account_id::AccountIdRef;
const ALICE: &AccountIdRef = AccountIdRef::new_or_panic("0__0");
const ALICE1: &AccountIdRef = AccountIdRef::new_or_panic("@near");
const ALICE2: &AccountIdRef = AccountIdRef::new_or_panic("a");
const ALICE3: &AccountIdRef = AccountIdRef::new_or_panic("A");
const ALICE4: &AccountIdRef = AccountIdRef::new_or_panic("Abc");
const ALICE5: &AccountIdRef = AccountIdRef::new_or_panic("-near");
const ALICE6: &AccountIdRef = AccountIdRef::new_or_panic("near-");
const ALICE7: &AccountIdRef = AccountIdRef::new_or_panic("-near-");
const ALICE8: &AccountIdRef = AccountIdRef::new_or_panic("near.");
const ALICE9: &AccountIdRef = AccountIdRef::new_or_panic(".near");
const ALICE10: &AccountIdRef = AccountIdRef::new_or_panic("near@");
const ALICE11: &AccountIdRef = AccountIdRef::new_or_panic("@near");
const ALICE12: &AccountIdRef = AccountIdRef::new_or_panic("неар");
const ALICE13: &AccountIdRef = AccountIdRef::new_or_panic("@@@@@");
const ALICE14: &AccountIdRef = AccountIdRef::new_or_panic("0__0");
const ALICE15: &AccountIdRef = AccountIdRef::new_or_panic("0_-_0");
const ALICE16: &AccountIdRef = AccountIdRef::new_or_panic("0_-_0");
const ALICE17: &AccountIdRef = AccountIdRef::new_or_panic("..");
const ALICE18: &AccountIdRef = AccountIdRef::new_or_panic("a..near");
const ALICE19: &AccountIdRef = AccountIdRef::new_or_panic("nEar");
const ALICE20: &AccountIdRef = AccountIdRef::new_or_panic("_bowen");
const ALICE21: &AccountIdRef = AccountIdRef::new_or_panic("hello world");
const ALICE22: &AccountIdRef = AccountIdRef::new_or_panic(
    "abcdefghijklmnopqrstuvwxyz.abcdefghijklmnopqrstuvwxyz.abcdefghijklmnopqrstuvwxyz",
);
const ALICE23: &AccountIdRef =
    AccountIdRef::new_or_panic("01234567890123456789012345678901234567890123456789012345678901234");
// `@` separators are banned now
const ALICE24: &AccountIdRef = AccountIdRef::new_or_panic("some-complex-address@gmail.com");
const ALICE25: &AccountIdRef = AccountIdRef::new_or_panic("sub.buy_d1gitz@atata@b0-rg.c_0_m");
fn main() {}
