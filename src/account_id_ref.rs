use std::borrow::Cow;

use crate::{validation::validate, AccountId, ParseAccountError};

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
/// use near_sdk::{AccountId, AccountIdRef};
/// use std::convert::{TryFrom, TryInto};
///
/// // Construction
/// let alice = AccountIdRef::new("alice.near").unwrap();
/// assert!(AccountIdRef::new("invalid.").is_err());
///
/// // Initialize without validating
/// let alice_unchecked = AccountIdRef::new_unchecked("alice.near");
/// assert_eq!(alice, alice_unchecked);
/// ```
///
/// [`FromStr`]: std::str::FromStr
/// [`Path`]: std::path::Path
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "abi", derive(schemars::JsonSchema, BorshSchema))]
pub struct AccountIdRef(str);

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
        validate(id)?;

        // Safety:
        // - a newtype struct is guaranteed to have the same memory layout as its only field
        // - the borrow checker will enforce its rules appropriately on the resulting reference
        Ok(unsafe { &*(id as *const str as *const Self) })
    }

    /// Construct a [`&AccountIdRef`](AccountIdRef) from a string reference without validating the address.
    /// It is the responsibility of the caller to ensure the account ID is valid.
    ///
    /// For more information, read: <https://docs.near.org/docs/concepts/account#account-id-rules>
    pub fn new_unchecked<S: AsRef<str> + ?Sized>(id: &S) -> &Self {
        let id = id.as_ref();
        debug_assert!(validate(id).is_ok());

        // Safety: see `AccountId::new`
        unsafe { &*(id as *const str as *const Self) }
    }

    /// Construct a [`&mut AccountIdRef`](AccountIdRef) from a mutable string reference.
    ///
    /// This constructor validates the provided ID and will produce an error when validation fails.
    pub fn new_mut<S: AsMut<str> + ?Sized>(id: &mut S) -> Result<&mut Self, ParseAccountError> {
        let id = id.as_mut();
        validate(id)?;

        // Safety: see `AccountId::new`
        Ok(unsafe { &mut *(id as *mut str as *mut Self) })
    }

    /// Construct a [`&mut AccountIdRef`](AccountIdRef) from a mutable string reference without validating
    /// the address. It is the responsibility of the caller to ensure the account ID is valid.
    pub fn new_unchecked_mut<S: AsMut<str> + ?Sized>(id: &mut S) -> &mut Self {
        let id = id.as_mut();
        debug_assert!(validate(id).is_ok());

        // Safety: see `AccountId::new`
        unsafe { &mut *(id as *mut str as *mut Self) }
    }

    /// Returns a reference to the account ID bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Returns a reference to the account ID string.
    pub fn as_str(&self) -> &str {
        &self.0
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

impl<'s> TryFrom<&'s mut str> for &'s mut AccountIdRef {
    type Error = ParseAccountError;

    fn try_from(value: &'s mut str) -> Result<Self, Self::Error> {
        AccountIdRef::new_mut(value)
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
