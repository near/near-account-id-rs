use std::borrow::Cow;

#[cfg(feature = "schemars")]
use crate::schemars_exports::schemars;
use crate::{AccountId, ParseAccountError};

/// Account identifier. This is the human readable UTF-8 string which is used internally to index
/// accounts on the network and their respective state.
///
/// This is the "referenced" version of the account ID. It is to [`AccountId`] what [`str`] is to [`String`],
/// and works quite similarly to [`Path`]. Like with [`str`] and [`Path`], you
/// can't have a value of type `AccountIdRef`, but you can have a reference like `&AccountIdRef` or
/// `&mut AccountIdRef`.
///
/// This type supports zero-copy deserialization offered by [`serde`](https://docs.rs/serde/), but cannot
/// do the same for [`borsh`](https://docs.rs/borsh/) since the latter does not support zero-copy.
///
/// # Examples
/// ```
/// use near_account_id::{AccountId, AccountIdRef};
/// use std::convert::{TryFrom, TryInto};
///
/// // Construction
/// let alice = AccountIdRef::new("alice.near").unwrap();
/// assert!(AccountIdRef::new("invalid.").is_err());
/// ```
///
/// [`FromStr`]: std::str::FromStr
/// [`Path`]: std::path::Path
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "abi", derive(borsh::BorshSchema))]
pub struct AccountIdRef(pub(crate) str);

/// Enum representing possible types of accounts.
/// This `enum` is returned by the [`get_account_type`] method on [`AccountIdRef`].
/// See its documentation for more.
///
/// [`get_account_type`]: AccountIdRef::get_account_type
/// [`AccountIdRef`]: struct.AccountIdRef.html
#[derive(PartialEq)]
pub enum AccountType {
    /// Any valid account, that is neither NEAR-implicit nor ETH-implicit.
    NamedAccount,
    /// An account with 64 characters long hexadecimal address.
    NearImplicitAccount,
    /// An account which address starts with '0x', followed by 40 hex characters.
    EthImplicitAccount,
}

impl AccountType {
    pub fn is_implicit(&self) -> bool {
        match &self {
            Self::NearImplicitAccount => true,
            Self::EthImplicitAccount => true,
            Self::NamedAccount => false,
        }
    }
}

impl AccountIdRef {
    /// Shortest valid length for a NEAR Account ID.
    pub const MIN_LEN: usize = crate::validation::MIN_LEN;
    /// Longest valid length for a NEAR Account ID.
    pub const MAX_LEN: usize = crate::validation::MAX_LEN;

    /// Construct a [`&AccountIdRef`](AccountIdRef) from a string reference.
    ///
    /// This constructor validates the provided ID, and will produce an error when validation fails.
    pub fn new<S: AsRef<str> + ?Sized>(id: &S) -> Result<&Self, ParseAccountError> {
        let id = id.as_ref();
        crate::validation::validate(id)?;

        // Safety:
        // - a newtype struct is guaranteed to have the same memory layout as its only field
        // - the borrow checker will enforce its rules appropriately on the resulting reference
        Ok(unsafe { &*(id as *const str as *const Self) })
    }

    /// Construct a [`&AccountIdRef`](AccountIdRef) from with validation at compile time.
    /// This constructor will panic if validation fails.
    /// ```rust
    /// use near_account_id::AccountIdRef;
    /// const ALICE: &AccountIdRef = AccountIdRef::new_or_panic("alice.near");
    /// ```
    pub const fn new_or_panic(id: &str) -> &Self {
        crate::validation::validate_const(id);

        unsafe { &*(id as *const str as *const Self) }
    }

    /// Construct a [`&AccountIdRef`](AccountIdRef) from a string reference without validating the address.
    /// It is the responsibility of the caller to ensure the account ID is valid.
    ///
    /// For more information, read: <https://docs.near.org/docs/concepts/account#account-id-rules>
    pub(crate) fn new_unvalidated<S: AsRef<str> + ?Sized>(id: &S) -> &Self {
        let id = id.as_ref();
        // In nearcore, due to legacy reasons, AccountId construction and validation are separated.
        // In order to avoid protocol change, `internal_unstable` feature was implemented and it is
        // expected that AccountId might be invalid and it will be explicitly validated at the
        // later stage.
        #[cfg(not(feature = "internal_unstable"))]
        debug_assert!(crate::validation::validate(id).is_ok());

        // Safety: see `AccountIdRef::new`
        unsafe { &*(id as *const str as *const Self) }
    }

    /// Returns a reference to the account ID bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Returns a string slice of the entire Account ID.
    ///
    /// ## Examples
    ///
    /// ```
    /// use near_account_id::AccountIdRef;
    ///
    /// let carol = AccountIdRef::new("carol.near").unwrap();
    /// assert_eq!("carol.near", carol.as_str());
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns `true` if the account ID is a top-level NEAR Account ID.
    ///
    /// See [Top-level Accounts](https://docs.near.org/docs/concepts/account#top-level-accounts).
    ///
    /// ## Examples
    ///
    /// ```
    /// use near_account_id::AccountIdRef;
    ///
    /// let near_tla = AccountIdRef::new("near").unwrap();
    /// assert!(near_tla.is_top_level());
    ///
    /// // "alice.near" is a sub account of "near" account
    /// let alice = AccountIdRef::new("alice.near").unwrap();
    /// assert!(!alice.is_top_level());
    /// ```
    pub fn is_top_level(&self) -> bool {
        !self.is_system() && !self.0.contains('.')
    }

    /// Returns `true` if the `AccountId` is a direct sub-account of the provided parent account.
    ///
    /// See [Subaccounts](https://docs.near.org/docs/concepts/account#subaccounts).
    ///
    /// ## Examples
    ///
    /// ```
    /// use near_account_id::AccountId;
    ///
    /// let near_tla: AccountId = "near".parse().unwrap();
    /// assert!(near_tla.is_top_level());
    ///
    /// let alice: AccountId = "alice.near".parse().unwrap();
    /// assert!(alice.is_sub_account_of(&near_tla));
    ///
    /// let alice_app: AccountId = "app.alice.near".parse().unwrap();
    ///
    /// // While app.alice.near is a sub account of alice.near,
    /// // app.alice.near is not a sub account of near
    /// assert!(alice_app.is_sub_account_of(&alice));
    /// assert!(!alice_app.is_sub_account_of(&near_tla));
    /// ```
    pub fn is_sub_account_of(&self, parent: &AccountIdRef) -> bool {
        self.0
            .strip_suffix(parent.as_str())
            .and_then(|s| s.strip_suffix('.'))
            .map_or(false, |s| !s.contains('.'))
    }

    /// Returns `AccountType::EthImplicitAccount` if the `AccountId` is a 40 characters long hexadecimal prefixed with '0x'.
    /// Returns `AccountType::NearImplicitAccount` if the `AccountId` is a 64 characters long hexadecimal.
    /// Otherwise, returns `AccountType::NamedAccount`.
    ///
    /// See [Implicit-Accounts](https://docs.near.org/docs/concepts/account#implicit-accounts).
    ///
    /// ## Examples
    ///
    /// ```
    /// use near_account_id::{AccountId, AccountType};
    ///
    /// let alice: AccountId = "alice.near".parse().unwrap();
    /// assert!(alice.get_account_type() == AccountType::NamedAccount);
    ///
    /// let eth_rando = "0xb794f5ea0ba39494ce839613fffba74279579268"
    ///     .parse::<AccountId>()
    ///     .unwrap();
    /// assert!(eth_rando.get_account_type() == AccountType::EthImplicitAccount);
    ///
    /// let near_rando = "98793cd91a3f870fb126f66285808c7e094afcfc4eda8a970f6648cdf0dbd6de"
    ///     .parse::<AccountId>()
    ///     .unwrap();
    /// assert!(near_rando.get_account_type() == AccountType::NearImplicitAccount);
    /// ```
    pub fn get_account_type(&self) -> AccountType {
        if crate::validation::is_eth_implicit(self.as_str()) {
            return AccountType::EthImplicitAccount;
        }
        if crate::validation::is_near_implicit(self.as_str()) {
            return AccountType::NearImplicitAccount;
        }
        AccountType::NamedAccount
    }

    /// Returns `true` if this `AccountId` is the system account.
    ///
    /// See [System account](https://nomicon.io/DataStructures/Account.html?highlight=system#system-account).
    ///
    /// ## Examples
    ///
    /// ```
    /// use near_account_id::AccountId;
    ///
    /// let alice: AccountId = "alice.near".parse().unwrap();
    /// assert!(!alice.is_system());
    ///
    /// let system: AccountId = "system".parse().unwrap();
    /// assert!(system.is_system());
    /// ```
    pub fn is_system(&self) -> bool {
        self == "system"
    }

    /// Returns the length of the underlying account id string.
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns parent's account id reference
    ///
    /// ## Examples
    /// ```
    /// use near_account_id::AccountIdRef;
    ///
    /// let alice: &AccountIdRef = AccountIdRef::new_or_panic("alice.near");
    /// let parent: &AccountIdRef = alice.get_parent_account_id().unwrap();
    ///
    /// assert!(alice.is_sub_account_of(parent));
    ///
    /// let near: &AccountIdRef = AccountIdRef::new_or_panic("near");
    ///
    /// assert!(near.get_parent_account_id().is_none());
    ///
    /// let implicit: &AccountIdRef = AccountIdRef::new_or_panic("248e104d1d4764d713c4211c13808c8fc887869c580f4178e60538ac5c2a0b26");
    ///
    /// assert!(implicit.get_parent_account_id().is_none());
    /// ```
    pub fn get_parent_account_id(&self) -> Option<&AccountIdRef> {
        let parent_str = self.as_str().split_once('.')?.1;
        Some(AccountIdRef::new_unvalidated(parent_str))
    }
}

impl std::fmt::Display for AccountIdRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl ToOwned for AccountIdRef {
    type Owned = AccountId;

    fn to_owned(&self) -> Self::Owned {
        AccountId(self.0.into())
    }
}

impl<'a> From<&'a AccountIdRef> for AccountId {
    fn from(id: &'a AccountIdRef) -> Self {
        id.to_owned()
    }
}

impl<'s> TryFrom<&'s str> for &'s AccountIdRef {
    type Error = ParseAccountError;

    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        AccountIdRef::new(value)
    }
}

impl AsRef<str> for AccountIdRef {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<AccountIdRef> for String {
    fn eq(&self, other: &AccountIdRef) -> bool {
        self == &other.0
    }
}

impl PartialEq<String> for AccountIdRef {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl PartialEq<AccountIdRef> for str {
    fn eq(&self, other: &AccountIdRef) -> bool {
        self == &other.0
    }
}

impl PartialEq<str> for AccountIdRef {
    fn eq(&self, other: &str) -> bool {
        &self.0 == other
    }
}

impl<'a> PartialEq<AccountIdRef> for &'a str {
    fn eq(&self, other: &AccountIdRef) -> bool {
        *self == &other.0
    }
}

impl<'a> PartialEq<&'a str> for AccountIdRef {
    fn eq(&self, other: &&'a str) -> bool {
        &self.0 == *other
    }
}

impl<'a> PartialEq<&'a AccountIdRef> for str {
    fn eq(&self, other: &&'a AccountIdRef) -> bool {
        self == &other.0
    }
}

impl<'a> PartialEq<str> for &'a AccountIdRef {
    fn eq(&self, other: &str) -> bool {
        &self.0 == other
    }
}

impl<'a> PartialEq<&'a AccountIdRef> for String {
    fn eq(&self, other: &&'a AccountIdRef) -> bool {
        self == &other.0
    }
}

impl<'a> PartialEq<String> for &'a AccountIdRef {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl PartialOrd<AccountIdRef> for String {
    fn partial_cmp(&self, other: &AccountIdRef) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(&other.0)
    }
}

impl PartialOrd<String> for AccountIdRef {
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl PartialOrd<AccountIdRef> for str {
    fn partial_cmp(&self, other: &AccountIdRef) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.as_str())
    }
}

impl PartialOrd<str> for AccountIdRef {
    fn partial_cmp(&self, other: &str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other)
    }
}

impl<'a> PartialOrd<AccountIdRef> for &'a str {
    fn partial_cmp(&self, other: &AccountIdRef) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.as_str())
    }
}

impl<'a> PartialOrd<&'a str> for AccountIdRef {
    fn partial_cmp(&self, other: &&'a str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(*other)
    }
}

impl<'a> PartialOrd<&'a AccountIdRef> for String {
    fn partial_cmp(&self, other: &&'a AccountIdRef) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(&other.0)
    }
}

impl<'a> PartialOrd<String> for &'a AccountIdRef {
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl<'a> PartialOrd<&'a AccountIdRef> for str {
    fn partial_cmp(&self, other: &&'a AccountIdRef) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.as_str())
    }
}

impl<'a> PartialOrd<str> for &'a AccountIdRef {
    fn partial_cmp(&self, other: &str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other)
    }
}

impl<'a> From<&'a AccountIdRef> for Cow<'a, AccountIdRef> {
    fn from(value: &'a AccountIdRef) -> Self {
        Cow::Borrowed(value)
    }
}

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for &'a AccountIdRef {
    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        (crate::validation::MIN_LEN, Some(crate::validation::MAX_LEN))
    }

    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut s = u.arbitrary::<&str>()?;

        loop {
            match AccountIdRef::new(s) {
                Ok(account_id) => break Ok(account_id),
                Err(ParseAccountError {
                    char: Some((idx, _)),
                    ..
                }) => {
                    s = &s[..idx];
                    continue;
                }
                _ => break Err(arbitrary::Error::IncorrectFormat),
            }
        }
    }

    fn arbitrary_take_rest(u: arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let s = <&str as arbitrary::Arbitrary>::arbitrary_take_rest(u)?;
        AccountIdRef::new(s).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseErrorKind;

    use super::*;

    #[test]
    #[cfg(feature = "schemars-v1")]
    fn test_schemars() {
        let schema = schemars::schema_for!(AccountIdRef);
        let json_schema = serde_json::to_value(&schema).unwrap();
        dbg!(&json_schema);
        assert_eq!(
            json_schema,
            serde_json::json!({
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                    "description": "Account identifier. This is the human readable UTF-8 string which is used internally to index\naccounts on the network and their respective state.\n\nThis is the \"referenced\" version of the account ID. It is to [`AccountId`] what [`str`] is to [`String`],\nand works quite similarly to [`Path`]. Like with [`str`] and [`Path`], you\ncan't have a value of type `AccountIdRef`, but you can have a reference like `&AccountIdRef` or\n`&mut AccountIdRef`.\n\nThis type supports zero-copy deserialization offered by [`serde`](https://docs.rs/serde/), but cannot\ndo the same for [`borsh`](https://docs.rs/borsh/) since the latter does not support zero-copy.\n\n# Examples\n```\nuse near_account_id::{AccountId, AccountIdRef};\nuse std::convert::{TryFrom, TryInto};\n\n// Construction\nlet alice = AccountIdRef::new(\"alice.near\").unwrap();\nassert!(AccountIdRef::new(\"invalid.\").is_err());\n```\n\n[`FromStr`]: std::str::FromStr\n[`Path`]: std::path::Path",
                    "title": "AccountIdRef",
                    "type": "string"
                }
            )
        );
    }

    #[test]
    #[cfg(feature = "schemars-v0_8")]
    fn test_schemars() {
        let schema = schemars::schema_for!(AccountIdRef);
        let json_schema = serde_json::to_value(&schema).unwrap();
        dbg!(&json_schema);
        assert_eq!(
            json_schema,
            serde_json::json!({
                    "$schema": "http://json-schema.org/draft-07/schema#",
                    "description": "Account identifier. This is the human readable UTF-8 string which is used internally to index accounts on the network and their respective state.\n\nThis is the \"referenced\" version of the account ID. It is to [`AccountId`] what [`str`] is to [`String`], and works quite similarly to [`Path`]. Like with [`str`] and [`Path`], you can't have a value of type `AccountIdRef`, but you can have a reference like `&AccountIdRef` or `&mut AccountIdRef`.\n\nThis type supports zero-copy deserialization offered by [`serde`](https://docs.rs/serde/), but cannot do the same for [`borsh`](https://docs.rs/borsh/) since the latter does not support zero-copy.\n\n# Examples ``` use near_account_id::{AccountId, AccountIdRef}; use std::convert::{TryFrom, TryInto};\n\n// Construction let alice = AccountIdRef::new(\"alice.near\").unwrap(); assert!(AccountIdRef::new(\"invalid.\").is_err()); ```\n\n[`FromStr`]: std::str::FromStr [`Path`]: std::path::Path",
                    "title": "AccountIdRef",
                    "type": "string"
                }
            )
        );
    }

    #[test]
    fn test_err_kind_classification() {
        let id = AccountIdRef::new("ErinMoriarty.near");
        debug_assert!(
            matches!(
                id,
                Err(ParseAccountError {
                    kind: ParseErrorKind::InvalidChar,
                    char: Some((0, 'E'))
                })
            ),
            "{:?}",
            id
        );

        let id = AccountIdRef::new("-KarlUrban.near");
        debug_assert!(
            matches!(
                id,
                Err(ParseAccountError {
                    kind: ParseErrorKind::RedundantSeparator,
                    char: Some((0, '-'))
                })
            ),
            "{:?}",
            id
        );

        let id = AccountIdRef::new("anthonystarr.");
        debug_assert!(
            matches!(
                id,
                Err(ParseAccountError {
                    kind: ParseErrorKind::RedundantSeparator,
                    char: Some((12, '.'))
                })
            ),
            "{:?}",
            id
        );

        let id = AccountIdRef::new("jack__Quaid.near");
        debug_assert!(
            matches!(
                id,
                Err(ParseAccountError {
                    kind: ParseErrorKind::RedundantSeparator,
                    char: Some((5, '_'))
                })
            ),
            "{:?}",
            id
        );
    }

    #[test]
    fn test_is_valid_top_level_account_id() {
        let ok_top_level_account_ids = &[
            "aa",
            "a-a",
            "a-aa",
            "100",
            "0o",
            "com",
            "near",
            "bowen",
            "b-o_w_e-n",
            "0o0ooo00oo00o",
            "alex-skidanov",
            "b-o_w_e-n",
            "no_lols",
            // ETH-implicit account
            "0xb794f5ea0ba39494ce839613fffba74279579268",
            // NEAR-implicit account
            "0123456789012345678901234567890123456789012345678901234567890123",
        ];
        for account_id in ok_top_level_account_ids {
            assert!(
                AccountIdRef::new(account_id).map_or(false, |account_id| account_id.is_top_level()),
                "Valid top level account id {:?} marked invalid",
                account_id
            );
        }

        let bad_top_level_account_ids = &[
            "ƒelicia.near", // fancy ƒ!
            "near.a",
            "b.owen",
            "bro.wen",
            "a.ha",
            "a.b-a.ra",
            "some-complex-address@gmail.com",
            "sub.buy_d1gitz@atata@b0-rg.c_0_m",
            "over.9000",
            "google.com",
            "illia.cheapaccounts.near",
            "10-4.8-2",
            "a",
            "A",
            "Abc",
            "-near",
            "near-",
            "-near-",
            "near.",
            ".near",
            "near@",
            "@near",
            "неар",
            "@@@@@",
            "0__0",
            "0_-_0",
            "0_-_0",
            "..",
            "a..near",
            "nEar",
            "_bowen",
            "hello world",
            "abcdefghijklmnopqrstuvwxyz.abcdefghijklmnopqrstuvwxyz.abcdefghijklmnopqrstuvwxyz",
            "01234567890123456789012345678901234567890123456789012345678901234",
            // Valid regex and length, but reserved
            "system",
        ];
        for account_id in bad_top_level_account_ids {
            assert!(
                !AccountIdRef::new(account_id)
                    .map_or(false, |account_id| account_id.is_top_level()),
                "Invalid top level account id {:?} marked valid",
                account_id
            );
        }
    }

    #[test]
    fn test_is_valid_sub_account_id() {
        let ok_pairs = &[
            ("test", "a.test"),
            ("test-me", "abc.test-me"),
            ("gmail.com", "abc.gmail.com"),
            ("gmail.com", "abc-lol.gmail.com"),
            ("gmail.com", "abc_lol.gmail.com"),
            ("gmail.com", "bro-abc_lol.gmail.com"),
            ("g0", "0g.g0"),
            ("1g", "1g.1g"),
            ("5-3", "4_2.5-3"),
        ];
        for (signer_id, sub_account_id) in ok_pairs {
            assert!(
                matches!(
                    (AccountIdRef::new(signer_id), AccountIdRef::new(sub_account_id)),
                    (Ok(signer_id), Ok(sub_account_id)) if sub_account_id.is_sub_account_of(signer_id)
                ),
                "Failed to create sub-account {:?} by account {:?}",
                sub_account_id,
                signer_id
            );
        }

        let bad_pairs = &[
            ("test", ".test"),
            ("test", "test"),
            ("test", "a1.a.test"),
            ("test", "est"),
            ("test", ""),
            ("test", "st"),
            ("test5", "ббб"),
            ("test", "a-test"),
            ("test", "etest"),
            ("test", "a.etest"),
            ("test", "retest"),
            ("test-me", "abc-.test-me"),
            ("test-me", "Abc.test-me"),
            ("test-me", "-abc.test-me"),
            ("test-me", "a--c.test-me"),
            ("test-me", "a_-c.test-me"),
            ("test-me", "a-_c.test-me"),
            ("test-me", "_abc.test-me"),
            ("test-me", "abc_.test-me"),
            ("test-me", "..test-me"),
            ("test-me", "a..test-me"),
            ("gmail.com", "a.abc@gmail.com"),
            ("gmail.com", ".abc@gmail.com"),
            ("gmail.com", ".abc@gmail@com"),
            ("gmail.com", "abc@gmail@com"),
            ("test", "a@test"),
            ("test_me", "abc@test_me"),
            ("gmail.com", "abc@gmail.com"),
            ("gmail@com", "abc.gmail@com"),
            ("gmail.com", "abc-lol@gmail.com"),
            ("gmail@com", "abc_lol.gmail@com"),
            ("gmail@com", "bro-abc_lol.gmail@com"),
            (
                "gmail.com",
                "123456789012345678901234567890123456789012345678901234567890@gmail.com",
            ),
            (
                "123456789012345678901234567890123456789012345678901234567890",
                "1234567890.123456789012345678901234567890123456789012345678901234567890",
            ),
            (
                "b794f5ea0ba39494ce839613fffba74279579268",
                // ETH-implicit account
                "0xb794f5ea0ba39494ce839613fffba74279579268",
            ),
            ("aa", "ъ@aa"),
            ("aa", "ъ.aa"),
        ];
        for (signer_id, sub_account_id) in bad_pairs {
            assert!(
                !matches!(
                    (AccountIdRef::new(signer_id), AccountIdRef::new(sub_account_id)),
                    (Ok(signer_id), Ok(sub_account_id)) if sub_account_id.is_sub_account_of(&signer_id)
                ),
                "Invalid sub-account {:?} created by account {:?}",
                sub_account_id,
                signer_id
            );
        }
    }

    #[test]
    fn test_is_account_id_near_implicit() {
        let valid_near_implicit_account_ids = &[
            "0000000000000000000000000000000000000000000000000000000000000000",
            "6174617461746174617461746174617461746174617461746174617461746174",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            "20782e20662e64666420482123494b6b6c677573646b6c66676a646b6c736667",
        ];
        for valid_account_id in valid_near_implicit_account_ids {
            assert!(
                matches!(
                    AccountIdRef::new(valid_account_id),
                    Ok(account_id) if account_id.get_account_type() == AccountType::NearImplicitAccount
                ),
                "Account ID {} should be valid 64-len hex",
                valid_account_id
            );
        }

        let invalid_near_implicit_account_ids = &[
            "000000000000000000000000000000000000000000000000000000000000000",
            "6.74617461746174617461746174617461746174617461746174617461746174",
            "012-456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "fffff_ffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            "oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo",
            "00000000000000000000000000000000000000000000000000000000000000",
        ];
        for invalid_account_id in invalid_near_implicit_account_ids {
            assert!(
                !matches!(
                    AccountIdRef::new(invalid_account_id),
                    Ok(account_id) if account_id.get_account_type() == AccountType::NearImplicitAccount
                ),
                "Account ID {} is not a NEAR-implicit account",
                invalid_account_id
            );
        }
    }

    #[test]
    fn test_is_account_id_eth_implicit() {
        let valid_eth_implicit_account_ids = &[
            "0x0000000000000000000000000000000000000000",
            "0x6174617461746174617461746174617461746174",
            "0x0123456789abcdef0123456789abcdef01234567",
            "0xffffffffffffffffffffffffffffffffffffffff",
            "0x20782e20662e64666420482123494b6b6c677573",
        ];
        for valid_account_id in valid_eth_implicit_account_ids {
            assert!(
                matches!(
                    valid_account_id.parse::<AccountId>(),
                    Ok(account_id) if account_id.get_account_type() == AccountType::EthImplicitAccount
                ),
                "Account ID {} should be valid 42-len hex, starting with 0x",
                valid_account_id
            );
        }

        let invalid_eth_implicit_account_ids = &[
            "04b794f5ea0ba39494ce839613fffba74279579268",
            "0x000000000000000000000000000000000000000",
            "0x6.74617461746174617461746174617461746174",
            "0x012-456789abcdef0123456789abcdef01234567",
            "0xfffff_ffffffffffffffffffffffffffffffffff",
            "0xoooooooooooooooooooooooooooooooooooooooo",
            "0x00000000000000000000000000000000000000000",
            "0000000000000000000000000000000000000000000000000000000000000000",
        ];
        for invalid_account_id in invalid_eth_implicit_account_ids {
            assert!(
                !matches!(
                    invalid_account_id.parse::<AccountId>(),
                    Ok(account_id) if account_id.get_account_type() == AccountType::EthImplicitAccount
                ),
                "Account ID {} is not an ETH-implicit account",
                invalid_account_id
            );
        }
    }

    #[test]
    #[cfg(feature = "arbitrary")]
    fn test_arbitrary() {
        let corpus = [
            ("a|bcd", None),
            ("ab|cde", Some("ab")),
            ("a_-b", None),
            ("ab_-c", Some("ab")),
            ("a", None),
            ("miraclx.near", Some("miraclx.near")),
            (
                "01234567890123456789012345678901234567890123456789012345678901234",
                None,
            ),
        ];

        for (input, expected_output) in corpus {
            assert!(input.len() <= u8::MAX as usize);
            let data = [input.as_bytes(), &[input.len() as _]].concat();
            let mut u = arbitrary::Unstructured::new(&data);

            assert_eq!(
                u.arbitrary::<&AccountIdRef>()
                    .ok()
                    .map(AsRef::<str>::as_ref),
                expected_output
            );
        }
    }
}
