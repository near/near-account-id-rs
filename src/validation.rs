use crate::{ParseAccountError, ParseErrorKind};

/// Shortest valid length for a NEAR Account ID.
pub const MIN_LEN: usize = 2;
/// Longest valid length for a NEAR Account ID.
pub const MAX_LEN: usize = 64;

pub const fn validate_const(account_id: &str) {
    const fn validate_format_const(id: &[u8], idx: usize, current_char_is_separator: bool) {
        if idx >= id.len() {
            if current_char_is_separator {
                panic!("NEAR Account ID cannot end with char separator (-, _, .)");
            }
            return;
        }

        match id[idx] {
            b'a'..=b'z' | b'0'..=b'9' => validate_format_const(id, idx + 1, false),
            b'-' | b'_' | b'.' => {
                if current_char_is_separator {
                    panic!("NEAR Account ID cannot contain redundant separator (-, _, .)")
                } else if idx == 0 {
                    panic!("NEAR Account ID cannot start with char separator (-, _, .)")
                } else {
                    validate_format_const(id, idx + 1, true)
                }
            }
            _ => panic!(
                "NEAR Account ID cannot contain invalid chars (only a-z, 0-9, -, _, and . are allowed)"
            ),
        }
    }

    if account_id.len() < MIN_LEN {
        panic!("NEAR Account ID is too short")
    } else if account_id.len() > MAX_LEN {
        panic!("NEAR Account ID is too long")
    }

    validate_format_const(account_id.as_bytes(), 0, false);
}

pub fn validate(account_id: &str) -> Result<(), ParseAccountError> {
    if account_id.len() < MIN_LEN {
        Err(ParseAccountError {
            kind: ParseErrorKind::TooShort,
            char: None,
        })
    } else if account_id.len() > MAX_LEN {
        Err(ParseAccountError {
            kind: ParseErrorKind::TooLong,
            char: None,
        })
    } else {
        // Adapted from https://github.com/near/near-sdk-rs/blob/fd7d4f82d0dfd15f824a1cf110e552e940ea9073/near-sdk/src/environment/env.rs#L819

        // NOTE: We don't want to use Regex here, because it requires extra time to compile it.
        // The valid account ID regex is /^(([a-z\d]+[-_])*[a-z\d]+\.)*([a-z\d]+[-_])*[a-z\d]+$/
        // Instead the implementation is based on the previous character checks.

        // We can safely assume that last char was a separator.
        let mut last_char_is_separator = true;

        let mut this = None;
        for (i, c) in account_id.chars().enumerate() {
            this.replace((i, c));
            let current_char_is_separator = match c {
                'a'..='z' | '0'..='9' => false,
                '-' | '_' | '.' => true,
                _ => {
                    return Err(ParseAccountError {
                        kind: ParseErrorKind::InvalidChar,
                        char: this,
                    });
                }
            };
            if current_char_is_separator && last_char_is_separator {
                return Err(ParseAccountError {
                    kind: ParseErrorKind::RedundantSeparator,
                    char: this,
                });
            }
            last_char_is_separator = current_char_is_separator;
        }

        if last_char_is_separator {
            return Err(ParseAccountError {
                kind: ParseErrorKind::RedundantSeparator,
                char: this,
            });
        }
        Ok(())
    }
}

pub fn is_eth_implicit(account_id: &str) -> bool {
    account_id.len() == 42
        && account_id.starts_with("0x")
        && account_id[2..].as_bytes().iter().all(|b| matches!(b, b'a'..=b'f' | b'0'..=b'9'))
}

pub fn is_near_implicit(account_id: &str) -> bool {
    account_id.len() == 64
        && account_id
            .as_bytes()
            .iter()
            .all(|b| matches!(b, b'a'..=b'f' | b'0'..=b'9'))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_data::{BAD_ACCOUNT_IDS, OK_ACCOUNT_IDS};

    #[test]
    fn test_is_valid_account_id() {
        for account_id in OK_ACCOUNT_IDS {
            if let Err(err) = validate(account_id) {
                panic!(
                    "Valid account id {:?} marked invalid: {}",
                    account_id,
                    err.kind()
                );
            }
        }

        for account_id in BAD_ACCOUNT_IDS {
            assert!(
                validate(account_id).is_err(),
                "Invalid account id {} marked valid",
                account_id
            );
        }
    }
    #[test]
    fn test_is_valid_account_id_const() {
        for account_id in OK_ACCOUNT_IDS {
            validate_const(account_id);
        }
    }

    #[test]
    fn test_is_invalid_account_id_const() {
        for account_id in BAD_ACCOUNT_IDS {
            // Do not print panic message for caught panic
            std::panic::set_hook(Box::new(|_| {}));

            let result = std::panic::catch_unwind(|| validate_const(account_id));

            // Restore panic hook to default to properly handle assertion failure
            let _ = std::panic::take_hook();

            assert!(
                result.is_err(),
                "Invalid account id {} marked valid",
                account_id
            );
        }
    }
}
