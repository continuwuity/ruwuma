//! `POST /_matrix/policy/unstable/org.matrix.msc4284/event/:eventId/check`
//!
//! Checks if an event is allowed by the room's policy server.

pub mod v1 {
    //! `/policy/unstable/org.matrix.msc4284` ([spec])
    //!
    //! [spec]: https://github.com/matrix-org/matrix-spec-proposals/pull/4284

    use ruma_common::{
        api::{response, Metadata},
        metadata,
    };

    const METADATA: Metadata = metadata! {
        method: GET,
        rate_limited: false,
        authentication: ServerSignatures,
        history: {
            unstable => "/_matrix/policy/unstable/event/:event_id/check",
        }
    };

    /// Response type for the `check` endpoint.
    #[response]
    pub struct Response {
        /// Either `ok` or `spam`, indicating the policy server's recommendation.
        pub recommendation: String,
    }

    impl Response {
        /// Creates a new `Response` with the given server name, timestamp, and event.
        pub fn new(recommendation: String) -> Self {
            Self { recommendation }
        }
    }
}
