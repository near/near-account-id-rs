//! This crate provides a type for representing a syntactically valid, unique account identifier on the [NEAR](https://near.org) network.
//!
//! ## Account ID Rules
//!
//! - Minimum length is `2`
//! - Maximum length is `64`
//! - An **Account ID** consists of **Account ID parts** separated by `.`, example:
//!   - `root` ✔
//!   - `alice.near` ✔
//!   - `app.stage.testnet` ✔
//! - Must not start or end with separators (`_`, `-` or `.`):
//!   - `_alice.` ✗
//!   - `.bob.near-` ✗
//! - Each part of the **Account ID** consists of lowercase alphanumeric symbols separated either by `_` or `-`, example:
//!   - `ƒelicia.near` ✗ (`ƒ` is not `f`)
//!   - `1_4m_n0t-al1c3.near` ✔
//! - Separators are not permitted to immediately follow each other, example:
//!   - `alice..near` ✗
//!   - `not-_alice.near` ✗
//! - An **Account ID** that is 64 characters long and consists of lowercase hex characters is a specific **implicit account ID**
//!
//! Learn more here: <https://docs.near.org/docs/concepts/account#account-id-rules>
//!
//! Also see [Error kind precedence](AccountId#error-kind-precedence).
//!
//! ## Usage
//!
//! ```
//! use near_account_id::AccountId;
//!
//! let alice: AccountId = "alice.near".parse().unwrap();
//!
//! assert!("ƒelicia.near".parse::<AccountId>().is_err()); // (ƒ is not f)
//! ```

mod errors;

mod account_id;
#[cfg(feature = "borsh")]
mod borsh;
#[cfg(feature = "serde")]
mod serde;
#[cfg(test)]
mod test_data;

pub use account_id::AccountId;
pub use errors::{ParseAccountError, ParseErrorKind};
