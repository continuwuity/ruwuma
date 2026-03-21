//! Types for the [`org.matrix.msc4284.policy`] event.
//!
//! [`org.matrix.msc4284.policy`]: https://github.com/matrix-org/matrix-spec-proposals/pull/4284

use std::collections::BTreeMap;
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};
use ruma_common::serde::Base64;
use ruma_common::serde::base64::UrlSafe;
use crate::EmptyStateKey;

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "org.matrix.msc4284.policy", kind = State, state_key_type = EmptyStateKey)]
pub struct RoomPolicyEventContent {
    /// The server name of the room's policy server.
    ///
    /// If the value is empty, the policy server should be ignored.
    pub via: Option<String>,

    /// The public key this policy server will sign with.
    ///
    /// If omitted, public_keys must be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<Base64<UrlSafe, Vec<u8>>>,

    /// The public keys this policy server may sign with.
    ///
    /// If omitted, public_key must be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_keys: Option<BTreeMap<String, Base64<UrlSafe, Vec<u8>>>>,
}

impl RoomPolicyEventContent {
    /// Create an empty `RoomPolicyEventContent`.
    pub fn new() -> Self {
        Self { via: None, public_key: None, public_keys: None }
    }

    pub fn effective_key(&self) -> Result<Base64<UrlSafe, Vec<u8>>, Box<dyn std::error::Error>> {
        if let Some(keys) = &self.public_keys {
            if let Some(key) = keys.get("ed25519") {
                return Ok(key.clone());
            };
        }
        self.public_key.clone().ok_or_else(|| "No public key in configuration".into())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PolicyServerResponseContent {
    /// The policy server's verdict. Either `ok` or `spam`.
    pub recommendation: String,
}

impl PolicyServerResponseContent {
    /// Create a new `PolicyServerResponseContent` with the given recommendation.
    pub fn new(recommendation: String) -> Self {
        Self { recommendation }
    }
}

impl From<String> for PolicyServerResponseContent {
    fn from(recommendation: String) -> Self {
        Self::new(recommendation)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use serde_json::{from_value as from_json_value, json, to_value as to_json_value};
    use ruma_common::serde::Base64;
    use super::RoomPolicyEventContent;
    use crate::OriginalStateEvent;

    #[test]
    fn no_public_keys() {
        let content = RoomPolicyEventContent { 
            via: Some("example.com".to_owned()),
            public_key: Some(
                Base64::parse("6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s")
                    .expect("failed to parse base64 key")
            ),
            public_keys: None

        };

        let actual = to_json_value(content).unwrap();
        let expected = json!({
            "via": "example.com",
            "public_key": "6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s"
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_public_key() {
        let content = RoomPolicyEventContent {
            via: Some("example.com".to_owned()),
            public_key: None,
            public_keys: Some(
                BTreeMap::from(
                    [(
                        "ed25519".to_owned(),
                        Base64::parse("6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s")
                            .expect("Failed to parse base64 key")
                    )]
                )
            )
        };

        let actual = to_json_value(content).unwrap();
        let expected = json!({
            "via": "example.com",
            "public_keys": {
                "ed25519": "6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s"
            }
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn deserialization_legacy_pubkey() {
        let json_data = json!({
            "content": {
                "via": "example.com",
                "public_key": "6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s"
            },
            "event_id": "123:example.com",
            "origin_server_ts": 1,
            "room_id": "!123456:example.com",
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "org.matrix.msc4284.policy",
            "unsigned": {}
        });

        let content = from_json_value::<OriginalStateEvent<RoomPolicyEventContent>>(json_data)
            .unwrap()
            .content;
        assert_eq!(
            content.via,
            Some("example.com".to_owned())
        );
        assert_eq!(
            content.effective_key().expect("Effective key should be present"),
            Base64::parse("6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s")
                .expect("failed to parse base64 key")
        );
    }

    #[test]
    fn deserialization_pubkey_map() {
        let json_data = json!({
            "content": {
                "via": "example.com",
                "public_keys": {
                    "ed25519": "6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s"
                }
            },
            "event_id": "123:example.com",
            "origin_server_ts": 1,
            "room_id": "!123456:example.com",
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "org.matrix.msc4284.policy"
        });

        let content = from_json_value::<OriginalStateEvent<RoomPolicyEventContent>>(json_data)
            .unwrap()
            .content;
        assert_eq!(
            content.via,
            Some("example.com".to_owned())
        );
        assert_eq!(
            content.effective_key().expect("Effective key should be present"),
            Base64::parse("6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s")
                .expect("failed to parse base64 key")
        );
    }

    #[test]
    fn deserialization_pubkey_map_and_legacy_pubkey() {
        let json_data = json!({
            "content": {
                "via": "example.com",
                "public_keys": "6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s",
                "public_keys": {
                    "ed25519": "6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s"
                }
            },
            "event_id": "123:example.com",
            "origin_server_ts": 1,
            "room_id": "!123456:example.com",
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "org.matrix.msc4284.policy"
        });

        let content = from_json_value::<OriginalStateEvent<RoomPolicyEventContent>>(json_data)
            .unwrap()
            .content;
        assert_eq!(
            content.via,
            Some("example.com".to_owned())
        );
        assert_eq!(
            content.effective_key().expect("Effective key should be present"),
            Base64::parse("6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s")
                .expect("failed to parse base64 key")
        );
    }

    #[test]
    fn deserialization_pubkey_map_takes_priority() {
        let json_data = json!({
            "content": {
                "via": "example.com",
                "public_keys": "aGVsbG8gd29ybGQ",
                "public_keys": {
                    "ed25519": "6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s"
                }
            },
            "event_id": "123:example.com",
            "origin_server_ts": 1,
            "room_id": "!123456:example.com",
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "org.matrix.msc4284.policy"
        });

        let content = from_json_value::<OriginalStateEvent<RoomPolicyEventContent>>(json_data)
            .unwrap()
            .content;
        assert_eq!(
            content.via,
            Some("example.com".to_owned())
        );
        assert_eq!(
            content.effective_key().expect("Effective key should be present"),
            Base64::parse("6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s")
                .expect("failed to parse base64 key")
        );
    }
}
