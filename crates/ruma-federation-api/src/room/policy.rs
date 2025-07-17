//! `POST /_matrix/policy/unstable/org.matrix.msc4284/event/:eventId/check`
//!
//! Checks if an event is allowed by the room's policy server.

pub mod v1 {
    //! `/policy/unstable/org.matrix.msc4284` ([spec])
    //!
    //! [spec]: https://github.com/matrix-org/matrix-spec-proposals/pull/4284

    use ruma_common::{
        api::{request, response, Metadata},
        metadata, OwnedEventId,
    };
    use serde_json::value::RawValue as RawJsonValue;

    const METADATA: Metadata = metadata! {
        method: POST,
        rate_limited: false,
        authentication: ServerSignatures,
        history: {
            unstable => "/_matrix/policy/unstable/org.matrix.msc4284/event/:event_id/check",
        }
    };

    /// Response type for the `check` endpoint.
    #[response]
    pub struct Response {
        /// Either `ok` or `spam`, indicating the policy server's recommendation.
        pub recommendation: String,
    }

    impl Response {
        /// Creates a new `Response` with the given recommendation.
        pub fn new(recommendation: String) -> Self {
            Self { recommendation }
        }
    }

    /// Request type for the `check` endpoint.
    #[request]
    pub struct Request {
        /// The event ID to check.
        #[ruma_api(path)]
        pub event_id: OwnedEventId,

        /// The PDU body (optional)
        #[ruma_api(body)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pdu: Option<Box<RawJsonValue>>,
    }

    impl Request {
        /// Creates a new `Request` with the given event ID.
        pub fn new(event_id: OwnedEventId) -> Self {
            Self { event_id, pdu: None }
        }
    }
}
