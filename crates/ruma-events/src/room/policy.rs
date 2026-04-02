//! Types for the [`org.matrix.msc4284.policy`] event.
//!
//! [`org.matrix.msc4284.policy`]: https://github.com/matrix-org/matrix-spec-proposals/pull/4284

use std::collections::BTreeMap;
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};
use ruma_common::serde::Base64;
use ruma_common::serde::base64::Standard;
use crate::EmptyStateKey;

type PolicyServerSigningKey = Base64<Standard, Vec<u8>>;

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type="org.matrix.msc4284.policy",kind=State,state_key_type=EmptyStateKey)]
pub struct UnstableRoomPolicyEventContent {
    /// The server name of the room's policy server.
    ///
    /// If the value is empty, the policy server should be ignored.
    pub via: Option<String>,

    /// The public key this policy server will sign with.
    ///
    /// If omitted, public_keys must be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<PolicyServerSigningKey>,

    /// The public keys this policy server may sign with.
    ///
    /// If omitted, public_key must be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_keys: Option<BTreeMap<String, PolicyServerSigningKey>>,
}

impl UnstableRoomPolicyEventContent {
    /// Create an empty `UnstableRoomPolicyEventContent`.
    pub fn new() -> Self {
        Self { via: None, public_key: None, public_keys: None }
    }

    pub fn effective_key(&self) -> Result<PolicyServerSigningKey, Box<dyn std::error::Error>> {
        if let Some(keys) = &self.public_keys {
            if let Some(key) = keys.get("ed25519") {
                return Ok(key.clone());
            };
        }
        self.public_key.clone().ok_or_else(|| "No public key in configuration".into())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type="m.room,policy",kind=State,state_key_type=EmptyStateKey)]
pub struct RoomPolicyEventContent {
    /// The server name of the room's policy server.
    ///
    /// If the value is empty, the policy server should be ignored.
    pub via: Option<String>,

    /// The public keys this policy server may sign with.
    ///
    /// If omitted, public_key must be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_keys: Option<BTreeMap<String, PolicyServerSigningKey>>,
}

impl RoomPolicyEventContent {
    /// Create an empty `RoomPolicyEventContent`.
    pub fn new() -> Self {
        Self { via: None, public_keys: None }
    }

    pub fn effective_key(&self) -> Result<PolicyServerSigningKey, Box<dyn std::error::Error>> {
        if let Some(keys) = &self.public_keys {
            if let Some(key) = keys.get("ed25519") {
                return Ok(key.clone());
            };
        }
        Err("No public key in configuration".into())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use serde_json::{from_value as from_json_value, json, to_value as to_json_value};
    use ruma_common::serde::Base64;
    use super::{RoomPolicyEventContent, UnstableRoomPolicyEventContent};
    use crate::{EventContent, OriginalStateEvent};

    #[test]
    fn no_public_keys() {
        let content = UnstableRoomPolicyEventContent {
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
        let content = UnstableRoomPolicyEventContent {
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
            "event_id": "$123:example.com",
            "origin_server_ts": 1,
            "room_id": "!123456:example.com",
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "org.matrix.msc4284.policy",
            "unsigned": {}
        });

        let content = from_json_value::<OriginalStateEvent<UnstableRoomPolicyEventContent>>(json_data)
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
            "event_id": "$123:example.com",
            "origin_server_ts": 1,
            "room_id": "!123456:example.com",
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "org.matrix.msc4284.policy"
        });

        let content = from_json_value::<OriginalStateEvent<UnstableRoomPolicyEventContent>>(json_data)
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
            "event_id": "$123:example.com",
            "origin_server_ts": 1,
            "room_id": "!123456:example.com",
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "org.matrix.msc4284.policy"
        });

        let content = from_json_value::<OriginalStateEvent<UnstableRoomPolicyEventContent>>(json_data)
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
            "event_id": "$123:example.com",
            "origin_server_ts": 1,
            "room_id": "!123456:example.com",
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "org.matrix.msc4284.policy"
        });

        let content = from_json_value::<OriginalStateEvent<UnstableRoomPolicyEventContent>>(json_data)
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
    fn legacy_state_event() {
        let json_data = json!({
            "content": {
                "via": "example.com",
                "public_key": "6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s"
            },
            "event_id": "$123:example.com",
            "origin_server_ts": 1,
            "room_id": "!123456:example.com",
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "org.matrix.msc4284.policy"
        });
        let event = from_json_value::<OriginalStateEvent<UnstableRoomPolicyEventContent>>(json_data).unwrap();
        assert_eq!(event.content.via, Some("example.com".to_owned()));
        assert_eq!(event.content.effective_key().expect("Effective key should be present"), Base64::parse("6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s").expect("failed to parse base64 key"));
        assert_eq!(event.content.event_type(), "org.matrix.msc4284.policy".into());
    }

    #[test]
    fn modern_state_event() {
        let json_data = json!({
            "content": {
                "via": "example.com",
                "public_key": "6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s"
            },
            "event_id": "$123:example.com",
            "origin_server_ts": 1,
            "room_id": "!123456:example.com",
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "m.room.policy"
        });
        let event = from_json_value::<OriginalStateEvent<RoomPolicyEventContent>>(json_data).unwrap();
        assert_eq!(event.content.via, Some("example.com".to_owned()));
        assert_eq!(event.content.effective_key().expect("Effective key should be present"), Base64::parse("6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s").expect("failed to parse base64 key"));
        assert_eq!(event.content.event_type(), "m.room.policy".into());
    }

    #[test]
    fn real_state_event() {
        let json_data = json!({
            "public_key": "+f6V33kgj98Wb4LcgVvO/0INL0jEQjMkl77+1O3wRY4",
            "public_keys": {
                "ed25519": "+f6V33kgj98Wb4LcgVvO/0INL0jEQjMkl77+1O3wRY4"
            },
            "via": "corellia.timedout.uk"
        });
        let event = from_json_value::<RoomPolicyEventContent>(json_data);
        assert!(event.is_ok(), "Failed to deserialize real state event: {:?}", event.err());
        let event = event.unwrap();
        assert_eq!(event.via, Some("corellia.timedout.uk".to_owned()));
        assert_eq!(event.effective_key().expect("Effective key should be present"), Base64::parse("+f6V33kgj98Wb4LcgVvO/0INL0jEQjMkl77+1O3wRY4").expect("failed to parse base64 key"));
    }
}
