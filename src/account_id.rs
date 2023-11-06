use std::{borrow::Cow, fmt, ops::Deref, str::FromStr};

use crate::{AccountIdRef, ParseAccountError};

/// NEAR Account Identifier.
///
/// This is a unique, syntactically valid, human-readable account identifier on the NEAR network.
///
/// [See the crate-level docs for information about validation.](index.html#account-id-rules)
///
/// Also see [Error kind precedence](AccountId#error-kind-precedence).
///
/// ## Examples
///
/// ```
/// use near_account_id::AccountId;
///
/// let alice: AccountId = "alice.near".parse().unwrap();
///
/// assert!("ƒelicia.near".parse::<AccountId>().is_err()); // (ƒ is not f)
/// ```
#[derive(Eq, Ord, Hash, Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct AccountId(pub(crate) Box<str>);

impl AccountId {
    /// Shortest valid length for a NEAR Account ID.
    pub const MIN_LEN: usize = crate::validation::MIN_LEN;
    /// Longest valid length for a NEAR Account ID.
    pub const MAX_LEN: usize = crate::validation::MAX_LEN;

    /// Creates an `AccountId` without any validation checks.
    ///
    /// Please note that this is restrictively for internal use only. Plus, being behind a feature flag,
    /// this could be removed later in the future.
    ///
    /// ## Safety
    ///
    /// Since this skips validation and constructs an `AccountId` regardless,
    /// the caller bears the responsibility of ensuring that the Account ID is valid.
    /// You can use the [`AccountId::validate`] function sometime after its creation but before it's use.
    ///
    /// ## Examples
    ///
    /// ```
    /// use near_account_id::AccountId;
    ///
    /// let alice = AccountId::new_unvalidated("alice.near".to_string());
    /// assert!(AccountId::validate(alice.as_str()).is_ok());
    /// ```
    #[doc(hidden)]
    #[cfg(feature = "internal_unstable")]
    #[deprecated = "AccountId construction without validation is illegal since nearcore#4440"]
    pub fn new_unvalidated(account_id: String) -> Self {
        Self(account_id.into_boxed_str())
    }

    /// Validates a string as a well-structured NEAR Account ID.
    ///
    /// Checks Account ID validity without constructing an `AccountId` instance.
    ///
    /// ## Examples
    ///
    /// ```
    /// use near_account_id::{AccountId, ParseErrorKind};
    ///
    /// assert!(AccountId::validate("alice.near").is_ok());
    ///
    /// assert!(
    ///   matches!(
    ///     AccountId::validate("ƒelicia.near"), // fancy ƒ!
    ///     Err(err) if err.kind() == &ParseErrorKind::InvalidChar
    ///   )
    /// );
    /// ```
    ///
    /// ## Error kind precedence
    ///
    /// If an Account ID has multiple format violations, the first one would be reported.
    ///
    /// ### Examples
    ///
    /// ```
    /// use near_account_id::{AccountId, ParseErrorKind};
    ///
    /// assert!(
    ///   matches!(
    ///     AccountId::validate("A__ƒƒluent."),
    ///     Err(err) if err.kind() == &ParseErrorKind::InvalidChar
    ///   )
    /// );
    ///
    /// assert!(
    ///   matches!(
    ///     AccountId::validate("a__ƒƒluent."),
    ///     Err(err) if err.kind() == &ParseErrorKind::RedundantSeparator
    ///   )
    /// );
    ///
    /// assert!(
    ///   matches!(
    ///     AccountId::validate("aƒƒluent."),
    ///     Err(err) if err.kind() == &ParseErrorKind::InvalidChar
    ///   )
    /// );
    ///
    /// assert!(
    ///   matches!(
    ///     AccountId::validate("affluent."),
    ///     Err(err) if err.kind() == &ParseErrorKind::RedundantSeparator
    ///   )
    /// );
    /// ```
    pub fn validate(account_id: &str) -> Result<(), ParseAccountError> {
        crate::validation::validate(account_id)
    }
}

impl AsRef<str> for AccountId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<AccountIdRef> for AccountId {
    fn as_ref(&self) -> &AccountIdRef {
        self
    }
}

impl Deref for AccountId {
    type Target = AccountIdRef;

    fn deref(&self) -> &Self::Target {
        AccountIdRef::new_unvalidated(&self.0)
    }
}

impl std::borrow::Borrow<AccountIdRef> for AccountId {
    fn borrow(&self) -> &AccountIdRef {
        AccountIdRef::new_unvalidated(self)
    }
}

impl FromStr for AccountId {
    type Err = ParseAccountError;

    fn from_str(account_id: &str) -> Result<Self, Self::Err> {
        crate::validation::validate(account_id)?;
        Ok(Self(account_id.into()))
    }
}

impl TryFrom<Box<str>> for AccountId {
    type Error = ParseAccountError;

    fn try_from(account_id: Box<str>) -> Result<Self, Self::Error> {
        crate::validation::validate(&account_id)?;
        Ok(Self(account_id))
    }
}

impl TryFrom<String> for AccountId {
    type Error = ParseAccountError;

    fn try_from(account_id: String) -> Result<Self, Self::Error> {
        crate::validation::validate(&account_id)?;
        Ok(Self(account_id.into_boxed_str()))
    }
}

impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl From<AccountId> for String {
    fn from(account_id: AccountId) -> Self {
        account_id.0.into_string()
    }
}

impl From<AccountId> for Box<str> {
    fn from(value: AccountId) -> Box<str> {
        value.0
    }
}

impl PartialEq<AccountId> for AccountIdRef {
    fn eq(&self, other: &AccountId) -> bool {
        &self.0 == other.as_str()
    }
}

impl PartialEq<AccountIdRef> for AccountId {
    fn eq(&self, other: &AccountIdRef) -> bool {
        self.as_str() == &other.0
    }
}

impl<'a> PartialEq<AccountId> for &'a AccountIdRef {
    fn eq(&self, other: &AccountId) -> bool {
        &self.0 == other.as_str()
    }
}

impl<'a> PartialEq<&'a AccountIdRef> for AccountId {
    fn eq(&self, other: &&'a AccountIdRef) -> bool {
        self.as_str() == &other.0
    }
}

impl PartialEq<AccountId> for String {
    fn eq(&self, other: &AccountId) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<String> for AccountId {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<AccountId> for str {
    fn eq(&self, other: &AccountId) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<str> for AccountId {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<'a> PartialEq<AccountId> for &'a str {
    fn eq(&self, other: &AccountId) -> bool {
        *self == other.as_str()
    }
}

impl<'a> PartialEq<&'a str> for AccountId {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}

impl PartialOrd<AccountId> for AccountIdRef {
    fn partial_cmp(&self, other: &AccountId) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl PartialOrd<AccountIdRef> for AccountId {
    fn partial_cmp(&self, other: &AccountIdRef) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(&other.0)
    }
}

impl<'a> PartialOrd<AccountId> for &'a AccountIdRef {
    fn partial_cmp(&self, other: &AccountId) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.as_str())
    }
}

impl<'a> PartialOrd<&'a AccountIdRef> for AccountId {
    fn partial_cmp(&self, other: &&'a AccountIdRef) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(&other.0)
    }
}

impl PartialOrd<AccountId> for String {
    fn partial_cmp(&self, other: &AccountId) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<String> for AccountId {
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<AccountId> for str {
    fn partial_cmp(&self, other: &AccountId) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.as_str())
    }
}

impl PartialOrd<str> for AccountId {
    fn partial_cmp(&self, other: &str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other)
    }
}

impl<'a> PartialOrd<AccountId> for &'a str {
    fn partial_cmp(&self, other: &AccountId) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.as_str())
    }
}

impl<'a> PartialOrd<&'a str> for AccountId {
    fn partial_cmp(&self, other: &&'a str) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(*other)
    }
}

impl<'a> From<AccountId> for Cow<'a, AccountIdRef> {
    fn from(value: AccountId) -> Self {
        Cow::Owned(value)
    }
}

impl<'a> From<&'a AccountId> for Cow<'a, AccountIdRef> {
    fn from(value: &'a AccountId) -> Self {
        Cow::Borrowed(value)
    }
}

impl<'a> From<Cow<'a, AccountIdRef>> for AccountId {
    fn from(value: Cow<'a, AccountIdRef>) -> Self {
        value.into_owned()
    }
}

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for AccountId {
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        <&AccountIdRef as arbitrary::Arbitrary>::size_hint(depth)
    }

    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(u.arbitrary::<&AccountIdRef>()?.into())
    }

    fn arbitrary_take_rest(u: arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(<&AccountIdRef as arbitrary::Arbitrary>::arbitrary_take_rest(u)?.into())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

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
                u.arbitrary::<AccountId>().map(Into::<String>::into).ok(),
                expected_output.map(Into::<String>::into)
            );
        }
    }
    #[test]
    #[cfg(feature = "schemars")]
    fn test_schemars() {
        let schema = schemars::schema_for!(AccountId);
        let json_schema = serde_json::to_value(&schema).unwrap();
        dbg!(&json_schema);
        assert_eq!(
            json_schema,
            serde_json::json!({
                    "$schema": "http://json-schema.org/draft-07/schema#",
                    "description": "NEAR Account Identifier.\n\nThis is a unique, syntactically valid, human-readable account identifier on the NEAR network.\n\n[See the crate-level docs for information about validation.](index.html#account-id-rules)\n\nAlso see [Error kind precedence](AccountId#error-kind-precedence).\n\n## Examples\n\n``` use near_account_id::AccountId;\n\nlet alice: AccountId = \"alice.near\".parse().unwrap();\n\nassert!(\"ƒelicia.near\".parse::<AccountId>().is_err()); // (ƒ is not f) ```",
                    "title": "AccountId",
                    "type": "string"
                }
            )
        );
    }
}
