use crate::{AccountIdRef, UniversalAccountId};

use super::AccountId;

use std::io::{Read, Write};

use borsh::{BorshDeserialize, BorshSerialize};

impl BorshSerialize for AccountId {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.0.serialize(writer)
    }
}

impl BorshSerialize for AccountIdRef {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.0.serialize(writer)
    }
}

impl BorshSerialize for UniversalAccountId {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.as_str().serialize(writer)
    }
}

impl BorshDeserialize for AccountId {
    fn deserialize_reader<R: Read>(rd: &mut R) -> std::io::Result<Self> {
        let account_id = Box::<str>::deserialize_reader(rd)?;
        crate::validation::validate(&account_id).map_err(|err| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid value: \"{}\", {}", account_id, err),
            )
        })?;
        Ok(Self(account_id))
    }
}

impl BorshDeserialize for UniversalAccountId {
    fn deserialize_reader<R: Read>(rd: &mut R) -> std::io::Result<Self> {
        let account_id = Box::<str>::deserialize_reader(rd)?;
        account_id.parse().map_err(|error| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid universal account ID {account_id:?}: {error}"),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use borsh::BorshDeserialize as _;

    use crate::test_data::{BAD_ACCOUNT_IDS, OK_ACCOUNT_IDS};
    use crate::{AccountId, UniversalAccountId};

    #[test]
    fn test_is_valid_account_id() {
        for account_id in OK_ACCOUNT_IDS {
            let parsed_account_id = account_id.parse::<AccountId>().unwrap_or_else(|err| {
                panic!("Valid account id {:?} marked invalid: {}", account_id, err)
            });

            let str_serialized_account_id = borsh::to_vec(account_id).unwrap();

            let deserialized_account_id = AccountId::try_from_slice(&str_serialized_account_id)
                .unwrap_or_else(|err| {
                    panic!("failed to deserialize account ID {:?}: {}", account_id, err)
                });
            assert_eq!(deserialized_account_id, parsed_account_id);

            let serialized_account_id =
                borsh::to_vec(&deserialized_account_id).unwrap_or_else(|err| {
                    panic!("failed to serialize account ID {:?}: {}", account_id, err)
                });
            assert_eq!(serialized_account_id, str_serialized_account_id);
        }

        for account_id in BAD_ACCOUNT_IDS {
            let str_serialized_account_id = borsh::to_vec(account_id).unwrap();

            assert!(
                AccountId::try_from_slice(&str_serialized_account_id).is_err(),
                "successfully deserialized invalid account ID {:?}",
                account_id
            );
        }
    }

    #[test]
    fn fuzz() {
        bolero::check!().for_each(|input: &[u8]| {
            if let Ok(account_id) = AccountId::try_from_slice(input) {
                assert_eq!(
                    account_id,
                    AccountId::try_from_slice(borsh::to_vec(&account_id).unwrap().as_slice())
                        .unwrap()
                );
            }
        });
    }

    #[test]
    fn universal_account_id_round_trip() {
        let account_id = UniversalAccountId::from_hash([0x5a; 32]);
        let bytes = borsh::to_vec(&account_id).unwrap();
        let generic: AccountId = account_id.clone().into();
        assert_eq!(bytes, borsh::to_vec(&generic).unwrap());
        assert_eq!(
            UniversalAccountId::try_from_slice(&bytes).unwrap(),
            account_id
        );

        let non_canonical = format!("0u{}1", "0".repeat(51));
        let bytes = borsh::to_vec(&non_canonical).unwrap();
        assert!(UniversalAccountId::try_from_slice(&bytes).is_err());
    }
}
