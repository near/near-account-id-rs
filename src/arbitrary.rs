use core::iter;

use arbitrary::{Arbitrary, Error, Result, Unstructured};
use arbitrary_with::{ArbitraryAs, UnstructuredExt};

use crate::{AccountId, AccountIdRef, AccountType};

impl Arbitrary<'_> for AccountId {
    fn arbitrary(u: &mut Unstructured<'_>) -> Result<AccountId> {
        match u.choose(&[
            AccountType::NamedAccount,
            AccountType::NearImplicitAccount,
            AccountType::EthImplicitAccount,
            AccountType::NearDeterministicAccount,
        ])? {
            AccountType::NamedAccount => u.arbitrary_as::<_, ArbitraryNamedAccountId>(),
            AccountType::NearImplicitAccount => {
                u.arbitrary_as::<_, ArbitraryNearImplicitAccountId>()
            }
            AccountType::EthImplicitAccount => u.arbitrary_as::<_, ArbitraryEthImplicitAccountId>(),
            AccountType::NearDeterministicAccount => {
                u.arbitrary_as::<_, ArbitraryNearDeterministicAccountId>()
            }
        }
    }
}

pub struct ArbitraryNearImplicitAccountId;

impl<'a> ArbitraryAs<'a, AccountId> for ArbitraryNearImplicitAccountId {
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<AccountId> {
        let pk = u.arbitrary::<[u8; 32]>()?;
        Ok(hex::encode(pk).parse().unwrap_or_else(|_| unreachable!()))
    }

    #[inline]
    fn size_hint_as(_depth: usize) -> (usize, Option<usize>) {
        (32, Some(32))
    }
}

pub struct ArbitraryEthImplicitAccountId;

impl<'a> ArbitraryAs<'a, AccountId> for ArbitraryEthImplicitAccountId {
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<AccountId> {
        let hash = u.arbitrary::<[u8; 20]>()?;
        Ok(format!("0x{}", hex::encode(hash))
            .parse()
            .unwrap_or_else(|_| unreachable!()))
    }

    #[inline]
    fn size_hint_as(_depth: usize) -> (usize, Option<usize>) {
        (20, Some(20))
    }
}

pub struct ArbitraryNearDeterministicAccountId;

impl<'a> ArbitraryAs<'a, AccountId> for ArbitraryNearDeterministicAccountId {
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<AccountId> {
        let hash = u.arbitrary::<[u8; 20]>()?;
        Ok(format!("0s{}", hex::encode(hash))
            .parse()
            .unwrap_or_else(|_| unreachable!()))
    }

    #[inline]
    fn size_hint_as(_depth: usize) -> (usize, Option<usize>) {
        (20, Some(20))
    }
}

pub struct ArbitraryNamedAccountId;

impl ArbitraryNamedAccountId {
    const NON_EDGE_ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz-_";
    const EDGE_ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";

    fn char(u: &mut Unstructured<'_>, on_edge: bool) -> Result<char> {
        u.choose(if on_edge {
            Self::EDGE_ALPHABET
        } else {
            Self::NON_EDGE_ALPHABET
        })
        .copied()
        .map(Into::into)
    }

    pub fn arbitrary_subaccount(
        u: &mut Unstructured<'_>,
        parent: Option<&AccountIdRef>,
    ) -> Result<AccountId> {
        if parent.is_some_and(|p| p.len() >= AccountId::MAX_LEN - 1) {
            // parent is too long already
            return Err(Error::IncorrectFormat);
        }

        let len = u
            .int_in_range(parent.map_or(
                2..=AccountId::MAX_LEN, // TLA
                #[allow(clippy::range_minus_one)]
                |p| 1..=AccountId::MAX_LEN - p.len() - 1,
            ))?
            // subaccount can't be empty
            .max(1);

        // account_id can't start with '-' or '_'
        let first = Self::char(u, true)?;

        let subaccount: String = if len == 1 {
            first.into()
        } else {
            let last = Self::char(u, true)?;

            let mid: String = iter::repeat_with({
                // '-' and '_' must be followed by edge char
                let mut last_not_edge = false;
                move || {
                    Self::char(u, last_not_edge).inspect(|c| last_not_edge = ['-', '_'].contains(c))
                }
            })
            .take(len - 2)
            .collect::<Result<_>>()?;

            format!("{first}{mid}{last}")
        };

        if let Some(parent) = parent {
            format!("{subaccount}.{parent}")
        } else {
            subaccount
        }
        .parse()
        .map_err(|_| Error::IncorrectFormat)
    }
}

impl<'a> ArbitraryAs<'a, AccountId> for ArbitraryNamedAccountId {
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<AccountId> {
        // TLA
        let mut account_id = Self::arbitrary_subaccount(u, None)?;

        // keep adding subaccounts while there is enough space for at least
        // single character + '.'
        while account_id.len() < AccountId::MAX_LEN - 2 && u.arbitrary()? {
            account_id = Self::arbitrary_subaccount(u, Some(&account_id))?;
        }

        Ok(account_id)
    }
}
