use std::borrow::Cow;

/// A trait for types that can be converted into an [`AccountId`](crate::AccountId)
///
/// This trait allows functions to accept account IDs in multiple forms without
/// requiring explicit cloning or type conversions from the caller.
///
/// When writing functions that accept account IDs, you often want to support
/// multiple input types for convenience:
/// - Owned `AccountId` - when the caller already has ownership
/// - `&AccountId`      - when the caller wants to keep their copy
/// - `&AccountIdRef`   - when working with borrowed account ID references
/// - `&str`            - when you have string slice that needs validation
/// - Owned `String`    - when you have and owned string that needs validation
///
/// Without this trait, you'd need either:
/// - Multiple function overloads
/// - Explicit `.clone()` calls from users
/// - A single restrictive type that's inconvenient
///
/// # Examples
///
/// ```
/// use near_account_id::{AccountId, AccountIdRef, TryIntoAccountId};
///
/// fn process_account(account: impl TryIntoAccountId) -> Result<(), Box<dyn std::error::Error>> {
///     let account_id: AccountId = account.try_into_account_id()?;
///     println!("Processing {}", account_id);
///     // Use account_id...
///     Ok(())
/// }
///
/// // All of these work:
/// let owned: AccountId = "alice.near".parse().unwrap();
/// process_account(owned)?; // Moves ownership, no clone
///
/// let owned: AccountId = "bob.near".parse().unwrap();
/// process_account(&owned)?; // Clones internally, caller keeps ownership
///
/// let borrowed: &AccountIdRef = AccountIdRef::new("carol.near").unwrap();
/// process_account(borrowed)?; // Clones internally
///
/// process_account("impostor.near")?; // Validates the string
/// process_account("other.near".to_string())?; // Validates and consumes
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
/// # Errors
///
/// Returns [`ParseAccountError`](crate::ParseAccountError) when the input string
/// is not a valid NEAR account ID. This can happen when:
/// - The string is too short (< 2 characters) or too long (> 64 characters)
/// - The string contains invalid characters (only `a-z`, `0-9`, `-`, `_`, `.` are allowed)
/// - The string has redundant separators (e.g., `..`, `--`, `__`, or starts/ends with separators)
///
/// Note that validated types (`AccountId`, `&AccountId`, `&AccountIdRef`)
/// will never return an error since they've already passed validation.
pub trait TryIntoAccountId {
    /// Converts this type into an owned [`AccountId`](crate::AccountId).
    ///
    /// For already-validated types (`AccountId`, `&AccountId`, `&AccountIdRef`),
    /// this moves or clones the value without re-validation.
    ///
    /// For string types (`&str`, `String`), this validates the format and
    /// returns an error if invalid.
    ///
    /// # Errors
    ///
    /// Returns [`ParseAccountError`](crate::ParseAccountError) if the input
    /// string is not a valid NEAR account ID.
    fn try_into_account_id(self) -> Result<crate::AccountId, crate::ParseAccountError>;

    fn as_str(&self) -> &str;
}

impl TryIntoAccountId for crate::AccountId {
    fn try_into_account_id(self) -> Result<crate::AccountId, crate::ParseAccountError> {
        Ok(self)
    }

    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl TryIntoAccountId for &crate::AccountId {
    fn try_into_account_id(self) -> Result<crate::AccountId, crate::ParseAccountError> {
        Ok(self.to_owned())
    }

    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl TryIntoAccountId for &crate::AccountIdRef {
    fn try_into_account_id(self) -> Result<crate::AccountId, crate::ParseAccountError> {
        Ok(self.to_owned())
    }

    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl TryIntoAccountId for &str {
    fn try_into_account_id(self) -> Result<crate::AccountId, crate::ParseAccountError> {
        self.parse()
    }

    fn as_str(&self) -> &str {
        self
    }
}

impl TryIntoAccountId for String {
    fn try_into_account_id(self) -> Result<crate::AccountId, crate::ParseAccountError> {
        crate::AccountId::try_from(self)
    }

    fn as_str(&self) -> &str {
        self
    }
}

impl TryIntoAccountId for Cow<'_, crate::AccountIdRef> {
    fn try_into_account_id(self) -> Result<crate::AccountId, crate::ParseAccountError> {
        Ok(self.into_owned())
    }

    fn as_str(&self) -> &str {
        self.as_ref().as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AccountId, AccountIdRef, ParseAccountError, ParseErrorKind};

    fn accept_account(account: impl TryIntoAccountId) -> Result<AccountId, ParseAccountError> {
        account.try_into_account_id()
    }

    #[test]
    fn test_owned_account_id() {
        let account: AccountId = "bob.near".parse().unwrap();
        let result = accept_account(account);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "bob.near");
    }

    #[test]
    fn test_borrowed_account_id() {
        let account: AccountId = "bob.near".parse().unwrap();
        let result = accept_account(&account);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "bob.near");
        // Original still exists
        assert_eq!(account, "bob.near");
    }

    #[test]
    fn test_account_id_ref() {
        let account_ref = AccountIdRef::new("bob.near").unwrap();
        let result = accept_account(account_ref);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "bob.near");
    }

    #[test]
    fn test_valid_str_ref() {
        let account_str = "bob.near";
        let result = accept_account(account_str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "bob.near");
    }

    #[test]
    fn test_valid_string() {
        let account_string = String::from("bob.near");
        let result = accept_account(account_string);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "bob.near");
    }

    #[test]
    fn test_invalid_str_ref() {
        let account_str = "a";
        let result = accept_account(account_str);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ParseErrorKind::TooShort);
    }

    #[test]
    fn test_invalid_string() {
        let account_string = "неар";
        let result = accept_account(account_string);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ParseErrorKind::InvalidChar);
    }

    #[test]
    fn test_no_unnecessary_clone() {
        // This test verifies the owned case doesn't clone
        let account: AccountId = "dave.near".parse().unwrap();
        let ptr_before = account.as_str().as_ptr();

        let result = accept_account(account);
        assert!(result.is_ok());
        let ptr_after = result.unwrap().0.as_ptr();

        // Same pointer = no allocation happened
        assert_eq!(ptr_before, ptr_after);
    }

    #[test]
    fn test_no_unnecessary_clone_string() {
        // This test verifies the owned String doesn't allocate extra memory
        let account: String = "dave.near".to_owned();
        let ptr_before = account.as_ptr();

        let result = accept_account(account);
        assert!(result.is_ok());
        let ptr_after = result.unwrap().0.as_ptr();

        // Same pointer = no allocation happened
        assert_eq!(ptr_before, ptr_after);
    }
}
