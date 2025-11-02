//! Types for the [`org.matrix.msc4284.policy`] event.
//!
//! [`org.matrix.msc4284.policy`]: https://github.com/matrix-org/matrix-spec-proposals/pull/4284

use ruma_common::serde::Base64;
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

use crate::EmptyStateKey;

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "org.matrix.msc4284.policy", kind = State, state_key_type = EmptyStateKey)]
pub struct RoomPolicyEventContent {
    /// The server name of the room's policy server.
    ///
    /// If the value is empty or unreachable, the policy server should be ignored.
    pub via: Option<String>,
    /// The public key this policy server will sign with.
    pub public_key: Option<Base64>
}

impl RoomPolicyEventContent {
    /// Create an empty `RoomPolicyEventContent`.
    pub fn new() -> Self {
        Self { via: None, public_key: None }
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
    use ruma_common::serde::Base64;
    use serde_json::{from_value as from_json_value, json, to_value as to_json_value};

    use super::RoomPolicyEventContent;
    use crate::OriginalStateEvent;

    #[test]
    fn serialization() {
        let content = RoomPolicyEventContent { 
            via: Some("example.com".to_owned()),
            public_key: Some(Base64::parse("6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s".as_bytes()).expect("valid pubkey"))
        };

        let actual = to_json_value(content).unwrap();
        let expected = json!({
            "via": "example.com",
            "public_key": "6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s"
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn deserialization() {
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
            content.public_key,
            Some(Base64::parse("6yhHGKhCiXTSEN2ksjV7kX_N6rBQZ3Xb-M7LlC6NS-s".as_bytes()).expect("valid pubkey"))
        );
    }
}
