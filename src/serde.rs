use crate::{AccountIdRef, UniversalAccountId};

use super::AccountId;

use serde::{de, ser};

impl ser::Serialize for AccountId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl ser::Serialize for AccountIdRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl ser::Serialize for UniversalAccountId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for AccountId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let account_id = Box::<str>::deserialize(deserializer)?;
        crate::validation::validate(&account_id).map_err(|err| {
            de::Error::custom(format!("invalid value: \"{}\", {}", account_id, err))
        })?;
        Ok(AccountId(account_id))
    }
}

impl<'de> de::Deserialize<'de> for UniversalAccountId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Box::<str>::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

impl<'de> de::Deserialize<'de> for &'de AccountIdRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        <&str as de::Deserialize>::deserialize(deserializer)
            .and_then(|s| Self::try_from(s).map_err(de::Error::custom))
    }
}

#[cfg(test)]
mod tests {
    use crate::test_data::{BAD_ACCOUNT_IDS, OK_ACCOUNT_IDS};
    use crate::{AccountId, UniversalAccountId};

    use serde_json::json;

    #[test]
    fn test_is_valid_account_id() {
        for account_id in OK_ACCOUNT_IDS.iter() {
            let parsed_account_id = account_id.parse::<AccountId>().unwrap_or_else(|err| {
                panic!("Valid account id {:?} marked invalid: {}", account_id, err)
            });

            let deserialized_account_id: AccountId = serde_json::from_value(json!(account_id))
                .unwrap_or_else(|err| {
                    panic!("failed to deserialize account ID {:?}: {}", account_id, err)
                });
            assert_eq!(deserialized_account_id, parsed_account_id);

            let serialized_account_id = serde_json::to_value(&deserialized_account_id)
                .unwrap_or_else(|err| {
                    panic!("failed to serialize account ID {:?}: {}", account_id, err)
                });
            assert_eq!(serialized_account_id, json!(account_id));
        }

        for account_id in BAD_ACCOUNT_IDS.iter() {
            assert!(
                serde_json::from_value::<AccountId>(json!(account_id)).is_err(),
                "successfully deserialized invalid account ID {:?}",
                account_id
            );
        }
    }

    #[test]
    fn fuzz() {
        bolero::check!().for_each(|input: &[u8]| {
            if let Ok(account_id) = std::str::from_utf8(input) {
                if let Ok(account_id) = serde_json::from_value::<AccountId>(json!(account_id)) {
                    assert_eq!(
                        account_id,
                        serde_json::from_value::<AccountId>(
                            serde_json::to_value(&account_id).unwrap()
                        )
                        .unwrap()
                    );
                }
            }
        });
    }

    #[test]
    fn universal_account_id_round_trip() {
        let account_id = UniversalAccountId::from_hash([0x5a; 32]);
        let json = serde_json::to_string(&account_id).unwrap();
        let generic: AccountId = account_id.clone().into();
        assert_eq!(json, serde_json::to_string(&generic).unwrap());
        assert_eq!(
            serde_json::from_str::<UniversalAccountId>(&json).unwrap(),
            account_id
        );

        let non_canonical = format!("\"0u{}1\"", "0".repeat(51));
        assert!(serde_json::from_str::<UniversalAccountId>(&non_canonical).is_err());
    }
}
