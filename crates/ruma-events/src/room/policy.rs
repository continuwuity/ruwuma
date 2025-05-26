//! Types for the [`org.matrix.msc4284.policy`] event.
//!
//! [`org.matrix.msc4284.policy`]: https://github.com/matrix-org/matrix-spec-proposals/pull/4284

use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

use crate::EmptyStateKey;

#[derive(Clone, Debug, Default, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "org.matrix.msc4284.policy", kind = State, state_key_type = EmptyStateKey)]
pub struct RoomPolicyServerContent {
    /// The server name of the room's policy server.
    ///
    /// If the value is empty or unreachable, the policy server should be ignored.
    pub via: Option<String>,
}

impl RoomPolicyServerContent {
    /// Create an empty `RoomPolicyServerContent`.
    pub fn new() -> Self {
        Self::default()
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
