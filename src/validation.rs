use crate::{ParseAccountError, ParseErrorKind};

/// Shortest valid length for a NEAR Account ID.
pub const MIN_LEN: usize = 2;
/// Longest valid length for a NEAR Account ID.
pub const MAX_LEN: usize = 64;

pub const fn validate_const(id: &[u8], idx: usize, current_char_is_separator: bool) -> () {
    if idx == id.len() && current_char_is_separator {
        panic!("account ID ends with char separator")
    }
    if idx == id.len() {
        return;
    }

    match id[idx] {
        b'a'..=b'z' | b'0'..=b'9' => validate_const(id, idx + 1, false),
        b'-' | b'_' | b'.' => {
            if current_char_is_separator {
                panic!("account ID contains redundant separator")
            } else if idx == 0 {
                panic!("account ID starts with char separator")
            } else {
                validate_const(id, idx + 1, true)
            }
        }
        _ => panic!("account ID contains invalid char"),
    };
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_data::{BAD_ACCOUNT_IDS, OK_ACCOUNT_IDS};

    #[test]
    fn test_is_valid_account_id() {
        for account_id in OK_ACCOUNT_IDS.iter().cloned() {
            if let Err(err) = validate(account_id) {
                panic!(
                    "Valid account id {:?} marked invalid: {}",
                    account_id,
                    err.kind()
                );
            }
        }

        for account_id in BAD_ACCOUNT_IDS.iter().cloned() {
            if validate(account_id).is_ok() {
                panic!("Invalid account id {:?} marked valid", account_id);
            }
        }
    }
    #[test]
    fn test_is_valid_account_id_const() {
        for account_id in OK_ACCOUNT_IDS.iter().cloned() {
            validate_const(account_id.as_bytes(), 0, false)
        }
    }
    #[test]
    fn compile_tests() {
        let t = trybuild::TestCases::new();
        t.compile_fail("src/tests/wrong_near_handlers/errors.rs");
    }
}
