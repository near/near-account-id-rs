use std::{borrow::Borrow, fmt, ops::Deref, str::FromStr};

use crate::{AccountId, AccountIdRef};

const UNIVERSAL_HASH_LEN: usize = 32;
const DATA_SYMBOLS: usize = 52;
const FULL_SYMBOLS: usize = DATA_SYMBOLS - 1;
const CROCKFORD: &[u8; 32] = b"0123456789abcdefghjkmnpqrstvwxyz";
const INVALID_SYMBOL: u8 = u8::MAX;
const DECODE: [u8; 256] = build_decode_table();

const fn build_decode_table() -> [u8; 256] {
    let mut table = [INVALID_SYMBOL; 256];
    let mut index = 0;
    while index < CROCKFORD.len() {
        table[CROCKFORD[index] as usize] = index as u8;
        index += 1;
    }
    table
}

/// Error returned when parsing a [`UniversalAccountId`].
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseUniversalAccountIdError {
    /// Universal account IDs are exactly 54 bytes long.
    InvalidLength {
        /// Expected byte length.
        expected: usize,
        /// Actual byte length.
        actual: usize,
    },
    /// The account ID does not start with `0u`.
    InvalidPrefix,
    /// The account ID contains a character outside the lowercase Crockford base32 alphabet.
    InvalidSymbol {
        /// Byte index of the invalid character.
        index: usize,
        /// Invalid character.
        symbol: char,
    },
    /// The final symbol sets one or more of the four padding bits.
    NonCanonicalEncoding,
}

impl fmt::Display for ParseUniversalAccountIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength { expected, actual } => write!(
                f,
                "invalid universal account ID length: expected {expected} bytes, got {actual}"
            ),
            Self::InvalidPrefix => f.write_str("universal account ID must start with `0u`"),
            Self::InvalidSymbol { index, symbol } => write!(
                f,
                "invalid universal account ID symbol {symbol:?} at byte index {index}"
            ),
            Self::NonCanonicalEncoding => f.write_str(
                "non-canonical universal account ID encoding: trailing padding bits are set",
            ),
        }
    }
}

impl std::error::Error for ParseUniversalAccountIdError {}

/// A canonical `0u` universal account ID.
///
/// Unlike [`AccountId`], this type guarantees that the value has the `0u` prefix,
/// uses the lowercase Crockford base32 alphabet, and has canonical trailing padding.
///
/// # Examples
///
/// Constructing from a hash is infallible and always produces the canonical spelling:
///
/// ```
/// use near_account_id::UniversalAccountId;
///
/// let hash = [0xff; 32];
/// let account_id = UniversalAccountId::from_hash(hash);
///
/// assert_eq!(
///     account_id.as_str(),
///     "0uzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzg"
/// );
/// assert_eq!(account_id.hash(), hash);
/// ```
///
/// Parsing rejects strings that are valid generic account IDs but are not canonical
/// universal account IDs:
///
/// ```
/// use near_account_id::{AccountId, UniversalAccountId};
///
/// let non_canonical = "0u0000000000000000000000000000000000000000000000000001";
/// assert!(non_canonical.parse::<AccountId>().is_ok());
/// assert!(non_canonical.parse::<UniversalAccountId>().is_err());
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "abi", derive(borsh::BorshSchema))]
pub struct UniversalAccountId(AccountId);

impl UniversalAccountId {
    /// Scheme and hash-function marker for universal account IDs.
    pub const PREFIX: &'static str = "0u";
    /// Number of bytes in the hash encoded by a universal account ID.
    pub const HASH_LEN: usize = UNIVERSAL_HASH_LEN;
    /// Total byte length of a universal account ID.
    pub const LEN: usize = Self::PREFIX.len() + DATA_SYMBOLS;

    /// Encodes a 32-byte hash as a canonical universal account ID.
    pub fn from_hash(hash: [u8; UNIVERSAL_HASH_LEN]) -> Self {
        Self(CanonicalUniversalBody::from_hash(&hash).into_account_id())
    }

    /// Decodes the 32-byte hash represented by this universal account ID.
    pub fn hash(&self) -> [u8; UNIVERSAL_HASH_LEN] {
        CanonicalUniversalBody::parse(self.as_str())
            .unwrap_or_else(|_| unreachable!("UniversalAccountId must remain canonical"))
            .into_hash()
    }

    /// Returns this value as a generic account ID reference.
    pub fn as_account_id(&self) -> &AccountIdRef {
        self.0.as_ref()
    }

    /// Returns the account ID as a string slice.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Converts this value into a generic owned account ID without allocation.
    pub fn into_account_id(self) -> AccountId {
        self.0
    }
}

impl From<[u8; UNIVERSAL_HASH_LEN]> for UniversalAccountId {
    fn from(hash: [u8; UNIVERSAL_HASH_LEN]) -> Self {
        Self::from_hash(hash)
    }
}

impl FromStr for UniversalAccountId {
    type Err = ParseUniversalAccountIdError;

    fn from_str(account_id: &str) -> Result<Self, Self::Err> {
        CanonicalUniversalBody::parse(account_id)?;
        Ok(Self(account_id.parse().unwrap_or_else(|_| {
            unreachable!("canonical universal ID is a valid account ID")
        })))
    }
}

impl TryFrom<AccountId> for UniversalAccountId {
    type Error = ParseUniversalAccountIdError;

    fn try_from(account_id: AccountId) -> Result<Self, Self::Error> {
        CanonicalUniversalBody::parse(account_id.as_str())?;
        Ok(Self(account_id))
    }
}

impl TryFrom<&AccountIdRef> for UniversalAccountId {
    type Error = ParseUniversalAccountIdError;

    fn try_from(account_id: &AccountIdRef) -> Result<Self, Self::Error> {
        account_id.as_str().parse()
    }
}

impl From<UniversalAccountId> for AccountId {
    fn from(account_id: UniversalAccountId) -> Self {
        account_id.into_account_id()
    }
}

impl From<&UniversalAccountId> for AccountId {
    fn from(account_id: &UniversalAccountId) -> Self {
        account_id.0.clone()
    }
}

impl AsRef<str> for UniversalAccountId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<AccountIdRef> for UniversalAccountId {
    fn as_ref(&self) -> &AccountIdRef {
        self.as_account_id()
    }
}

impl Borrow<AccountIdRef> for UniversalAccountId {
    fn borrow(&self) -> &AccountIdRef {
        self.as_account_id()
    }
}

impl Deref for UniversalAccountId {
    type Target = AccountIdRef;

    fn deref(&self) -> &Self::Target {
        self.as_account_id()
    }
}

impl fmt::Display for UniversalAccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Copy)]
struct CrockfordSymbol(u8);

impl CrockfordSymbol {
    const ZERO: Self = Self(0);

    fn parse(
        symbol: u8,
        index: usize,
        account_id: &str,
    ) -> Result<Self, ParseUniversalAccountIdError> {
        let value = DECODE[symbol as usize];
        if value == INVALID_SYMBOL {
            let symbol = account_id[index..]
                .chars()
                .next()
                .unwrap_or_else(|| unreachable!("symbol index must be in bounds"));
            return Err(ParseUniversalAccountIdError::InvalidSymbol { index, symbol });
        }
        Ok(Self(value))
    }

    fn encoded(self) -> u8 {
        CROCKFORD[self.0 as usize]
    }
}

/// The final base32 position carries one hash bit. Representing only that bit makes
/// non-zero padding unrepresentable after parsing.
#[derive(Clone, Copy)]
enum FinalHashBit {
    Zero,
    One,
}

impl FinalHashBit {
    fn symbol(self) -> CrockfordSymbol {
        match self {
            Self::Zero => CrockfordSymbol(0),
            Self::One => CrockfordSymbol(16),
        }
    }
}

impl TryFrom<CrockfordSymbol> for FinalHashBit {
    type Error = ();

    fn try_from(symbol: CrockfordSymbol) -> Result<Self, Self::Error> {
        match symbol.0 {
            0 => Ok(Self::Zero),
            16 => Ok(Self::One),
            _ => Err(()),
        }
    }
}

struct CanonicalUniversalBody {
    head: [CrockfordSymbol; FULL_SYMBOLS],
    tail: FinalHashBit,
}

impl CanonicalUniversalBody {
    fn parse(account_id: &str) -> Result<Self, ParseUniversalAccountIdError> {
        if account_id.len() != UniversalAccountId::LEN {
            return Err(ParseUniversalAccountIdError::InvalidLength {
                expected: UniversalAccountId::LEN,
                actual: account_id.len(),
            });
        }
        if !account_id.starts_with(UniversalAccountId::PREFIX) {
            return Err(ParseUniversalAccountIdError::InvalidPrefix);
        }

        let body: &[u8; DATA_SYMBOLS] = account_id.as_bytes()[UniversalAccountId::PREFIX.len()..]
            .try_into()
            .unwrap_or_else(|_| unreachable!("universal account body has a fixed length"));
        let mut head = [CrockfordSymbol::ZERO; FULL_SYMBOLS];
        for (offset, slot) in head.iter_mut().enumerate() {
            let index = UniversalAccountId::PREFIX.len() + offset;
            *slot = CrockfordSymbol::parse(body[offset], index, account_id)?;
        }

        let tail_index = UniversalAccountId::LEN - 1;
        let tail_symbol = CrockfordSymbol::parse(body[FULL_SYMBOLS], tail_index, account_id)?;
        let tail = FinalHashBit::try_from(tail_symbol)
            .map_err(|_| ParseUniversalAccountIdError::NonCanonicalEncoding)?;

        Ok(Self { head, tail })
    }

    fn from_hash(hash: &[u8; UNIVERSAL_HASH_LEN]) -> Self {
        let symbols = base32_encode(hash);
        let mut head = [CrockfordSymbol::ZERO; FULL_SYMBOLS];
        head.copy_from_slice(&symbols[..FULL_SYMBOLS]);
        let tail = FinalHashBit::try_from(symbols[FULL_SYMBOLS])
            .unwrap_or_else(|_| unreachable!("base32 encoder must emit zero padding"));
        Self { head, tail }
    }

    fn symbols(&self) -> [CrockfordSymbol; DATA_SYMBOLS] {
        let mut symbols = [CrockfordSymbol::ZERO; DATA_SYMBOLS];
        symbols[..FULL_SYMBOLS].copy_from_slice(&self.head);
        symbols[FULL_SYMBOLS] = self.tail.symbol();
        symbols
    }

    fn into_account_id(self) -> AccountId {
        let mut account_id = String::with_capacity(UniversalAccountId::LEN);
        account_id.push_str(UniversalAccountId::PREFIX);
        for symbol in self.symbols() {
            account_id.push(symbol.encoded() as char);
        }
        account_id
            .parse()
            .unwrap_or_else(|_| unreachable!("canonical universal ID is a valid account ID"))
    }

    fn into_hash(self) -> [u8; UNIVERSAL_HASH_LEN] {
        base32_decode(&self.symbols())
    }
}

pub(crate) fn is_universal_account_id(account_id: &str) -> bool {
    CanonicalUniversalBody::parse(account_id).is_ok()
}

/// 32 bytes to 52 five-bit symbol values, most-significant bit first. The final
/// symbol contains one hash bit and four zero padding bits.
fn base32_encode(hash: &[u8; UNIVERSAL_HASH_LEN]) -> [CrockfordSymbol; DATA_SYMBOLS] {
    let mut output = [CrockfordSymbol::ZERO; DATA_SYMBOLS];
    let mut accumulator = 0u32;
    let mut bits = 0u32;
    let mut index = 0;

    for &byte in hash {
        accumulator = (accumulator << 8) | u32::from(byte);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            output[index] = CrockfordSymbol(((accumulator >> bits) & 0x1f) as u8);
            index += 1;
        }
        accumulator &= (1u32 << bits) - 1;
    }

    output[index] = CrockfordSymbol(((accumulator << (5 - bits)) & 0x1f) as u8);
    index += 1;
    debug_assert_eq!(index, DATA_SYMBOLS);
    debug_assert_eq!(bits, 1);
    output
}

/// 52 canonical five-bit symbols to 32 bytes. The typed final symbol guarantees
/// that the four leftover bits are zero.
fn base32_decode(symbols: &[CrockfordSymbol; DATA_SYMBOLS]) -> [u8; UNIVERSAL_HASH_LEN] {
    let mut output = [0u8; UNIVERSAL_HASH_LEN];
    let mut accumulator = 0u32;
    let mut bits = 0u32;
    let mut index = 0;

    for symbol in symbols {
        accumulator = (accumulator << 5) | u32::from(symbol.0);
        bits += 5;
        while bits >= 8 {
            bits -= 8;
            output[index] = ((accumulator >> bits) & 0xff) as u8;
            index += 1;
            accumulator &= (1u32 << bits) - 1;
        }
    }

    debug_assert_eq!(index, UNIVERSAL_HASH_LEN);
    debug_assert_eq!(bits, 4);
    debug_assert_eq!(accumulator, 0);
    output
}

#[cfg(feature = "schemars-v0_8")]
impl schemars_v0_8::JsonSchema for UniversalAccountId {
    fn is_referenceable() -> bool {
        false
    }

    fn schema_name() -> String {
        "UniversalAccountId".to_string()
    }

    fn json_schema(_: &mut schemars_v0_8::r#gen::SchemaGenerator) -> schemars_v0_8::schema::Schema {
        use schemars_v0_8::schema::{
            InstanceType, Metadata, Schema, SchemaObject, SingleOrVec, StringValidation,
        };
        Schema::Object(SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
            metadata: Some(Box::new(Metadata {
                description: Some("A canonical `0u` universal account ID.".to_string()),
                ..Default::default()
            })),
            string: Some(Box::new(StringValidation {
                pattern: Some("^0u[0-9a-hjkmnp-tv-z]{51}[0g]$".to_string()),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

#[cfg(feature = "schemars-v1")]
impl schemars_v1::JsonSchema for UniversalAccountId {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "UniversalAccountId".into()
    }

    fn json_schema(_: &mut schemars_v1::SchemaGenerator) -> schemars_v1::Schema {
        schemars_v1::json_schema!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "description": "A canonical `0u` universal account ID.",
            "pattern": "^0u[0-9a-hjkmnp-tv-z]{51}[0g]$",
            "title": "UniversalAccountId",
            "type": "string"
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KNOWN_ANSWERS: &[([u8; UNIVERSAL_HASH_LEN], &str)] = &[
        (
            [0x00; UNIVERSAL_HASH_LEN],
            "0u0000000000000000000000000000000000000000000000000000",
        ),
        (
            [0xff; UNIVERSAL_HASH_LEN],
            "0uzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzg",
        ),
        (
            [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
                0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
                0x1c, 0x1d, 0x1e, 0x1f,
            ],
            "0u000g40r40m30e209185gr38e1w8124gk2gahc5rr34d1p70x3rfg",
        ),
    ];

    #[test]
    fn known_answer_vectors_round_trip() {
        for (hash, expected) in KNOWN_ANSWERS {
            let account_id = UniversalAccountId::from_hash(*hash);
            assert_eq!(account_id.as_str(), *expected);
            assert_eq!(account_id.hash(), *hash);
            assert_eq!(expected.parse::<UniversalAccountId>().unwrap(), account_id);
        }
    }

    #[test]
    fn arbitrary_hashes_round_trip() {
        for seed in 0u8..=255 {
            let mut hash = [0u8; UNIVERSAL_HASH_LEN];
            for (index, byte) in hash.iter_mut().enumerate() {
                *byte = seed.wrapping_add(index as u8).wrapping_mul(31);
            }
            let account_id = UniversalAccountId::from_hash(hash);
            assert_eq!(account_id.hash(), hash);
            assert_eq!(
                account_id.as_str().parse::<UniversalAccountId>().unwrap(),
                account_id
            );
        }
    }

    #[test]
    fn parser_round_trip_fuzz() {
        bolero::check!().for_each(|input: &[u8]| {
            if let Ok(input) = std::str::from_utf8(input) {
                if let Ok(account_id) = input.parse::<UniversalAccountId>() {
                    assert_eq!(account_id.as_str(), input);
                    assert_eq!(UniversalAccountId::from_hash(account_id.hash()), account_id);
                }
            }
        });
    }

    #[test]
    fn reports_structural_errors() {
        assert_eq!(
            "".parse::<UniversalAccountId>(),
            Err(ParseUniversalAccountIdError::InvalidLength {
                expected: UniversalAccountId::LEN,
                actual: 0,
            })
        );

        let wrong_prefix = format!("0s{}", "0".repeat(DATA_SYMBOLS));
        assert_eq!(
            wrong_prefix.parse::<UniversalAccountId>(),
            Err(ParseUniversalAccountIdError::InvalidPrefix)
        );

        let invalid_symbol = format!("0u{}i{}", "0".repeat(10), "0".repeat(41));
        assert_eq!(
            invalid_symbol.parse::<UniversalAccountId>(),
            Err(ParseUniversalAccountIdError::InvalidSymbol {
                index: 12,
                symbol: 'i',
            })
        );

        let multibyte_symbol = format!("0ué{}", "0".repeat(50));
        assert_eq!(multibyte_symbol.len(), UniversalAccountId::LEN);
        assert_eq!(
            multibyte_symbol.parse::<UniversalAccountId>(),
            Err(ParseUniversalAccountIdError::InvalidSymbol {
                index: 2,
                symbol: 'é',
            })
        );

        let non_canonical = format!("0u{}1", "0".repeat(FULL_SYMBOLS));
        assert_eq!(
            non_canonical.parse::<UniversalAccountId>(),
            Err(ParseUniversalAccountIdError::NonCanonicalEncoding)
        );
    }

    #[test]
    fn accepts_exactly_two_final_symbols() {
        let prefix = format!("0u{}", "0".repeat(FULL_SYMBOLS));
        let mut accepted = Vec::new();
        for &symbol in CROCKFORD {
            let candidate = format!("{prefix}{}", symbol as char);
            if candidate.parse::<UniversalAccountId>().is_ok() {
                accepted.push(symbol as char);
            }
        }
        assert_eq!(accepted, ['0', 'g']);
    }

    #[test]
    fn converts_to_and_from_generic_account_ids() {
        let expected = UniversalAccountId::from_hash([0x5a; UNIVERSAL_HASH_LEN]);
        let generic: AccountId = expected.clone().into();
        assert_eq!(
            UniversalAccountId::try_from(generic.clone()).unwrap(),
            expected
        );
        assert_eq!(
            UniversalAccountId::try_from(generic.as_ref()).unwrap(),
            expected
        );
        assert!(UniversalAccountId::try_from("alice.near".parse::<AccountId>().unwrap()).is_err());
    }

    #[cfg(feature = "schemars-v1")]
    #[test]
    fn schemars_v1_describes_the_canonical_format() {
        let schema = serde_json::to_value(schemars_v1::schema_for!(UniversalAccountId)).unwrap();
        assert_eq!(schema["pattern"], "^0u[0-9a-hjkmnp-tv-z]{51}[0g]$");
    }

    #[cfg(feature = "schemars-v0_8")]
    #[test]
    fn schemars_v0_8_describes_the_canonical_format() {
        let schema = serde_json::to_value(schemars_v0_8::schema_for!(UniversalAccountId)).unwrap();
        assert_eq!(schema["pattern"], "^0u[0-9a-hjkmnp-tv-z]{51}[0g]$");
    }
}
