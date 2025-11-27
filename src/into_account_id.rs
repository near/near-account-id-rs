/// A trait for types that can be converted into an [`AccountId`](crate::AccountId)
///
/// This trait allows functions to accept account IDs in multiple forms without
/// requiring explicit cloning or type conversions from the caller.
///
/// When writing functions that accept account IDs, you often want to support
/// multiple input types for convenience:
/// - Owned `AccountId` - when the caller already has ownership
/// - `&AccountId` - when the caller wants to keep their copy
/// - `&AccountIdRef` - when working with borrowed account ID references
///
/// Without this trait, you'd need either:
/// - Multiple function overloads
/// - Explicit `.clone()` calls from users
/// - A single restrictive type that's inconvenient
///
/// # Examples
///
/// ```
/// use near_account_id::{AccountId, AccountIdRef, IntoAccountId};
///
/// fn process_account(account: impl IntoAccountId) {
///     let account_id: AccountId = account.into_account_id();
///     // Use account_id...
/// }
///
/// // All of these work:
/// let owned: AccountId = "alice.near".parse().unwrap();
/// process_account(owned); // Moves ownership, no clone
///
/// let owned: AccountId = "bob.near".parse().unwrap();
/// process_account(&owned); // Clones internally, caller keeps ownership
///
/// let borrowed: &AccountIdRef = AccountIdRef::new("carol.near").unwrap();
/// process_account(borrowed); // Clones internally
/// ```
pub trait IntoAccountId {
    /// Converts this type into an owned [`AccountId`](crate::AccountId).
    ///
    /// For owned types, this moves the value. For borrowed types, this clones.
    fn into_account_id(self) -> crate::AccountId;
}

impl IntoAccountId for crate::AccountId {
    fn into_account_id(self) -> crate::AccountId {
        self
    }
}

impl IntoAccountId for &crate::AccountId {
    fn into_account_id(self) -> crate::AccountId {
        self.to_owned()
    }
}

impl IntoAccountId for &crate::AccountIdRef {
    fn into_account_id(self) -> crate::AccountId {
        self.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AccountId, AccountIdRef};

    fn accept_account(account: impl IntoAccountId) -> AccountId {
        account.into_account_id()
    }

    #[test]
    fn test_owned_account_id() {
        let account: AccountId = "bob.near".parse().unwrap();
        let result = accept_account(account);
        assert_eq!(result, "bob.near");
    }

    #[test]
    fn test_borrowed_account_id() {
        let account: AccountId = "bob.near".parse().unwrap();
        let result = accept_account(&account);
        assert_eq!(result, "bob.near");
        // Original still exists
        assert_eq!(account, "bob.near");
    }

    #[test]
    fn test_account_id_ref() {
        let account_ref = AccountIdRef::new("bob.near").unwrap();
        let result = accept_account(account_ref);
        assert_eq!(result, "bob.near");
    }

    #[test]
    fn test_no_unnecessary_clone() {
        // This test verifies the owned case doesn't clone
        let account: AccountId = "dave.near".parse().unwrap();
        let ptr_before = account.as_str().as_ptr();

        let result = accept_account(account);
        let ptr_after = result.0.as_ptr();

        // Same pointer = no allocation happened
        assert_eq!(ptr_before, ptr_after);
    }
}
